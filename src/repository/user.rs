use async_trait::async_trait;
use sea_orm::{prelude::*, Set};
use sea_orm::{DatabaseConnection, DbErr};

use crate::abstract_trait::user::UserRepositoryTrait;
use crate::domain::request::user::{CreateUserRequest, UpdateUserRequest};
use crate::entities::{users, Users};


pub struct UserRepository {
    db_pool: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db_pool)
            .await
    }


    async fn find_by_email_exists(&self, email: &str) -> Result<bool, DbErr> {
        let user_count = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .count(&self.db_pool)
            .await?;
        Ok(user_count > 0)
    }


    async fn find_all(&self) -> Result<Vec<users::Model>, DbErr> {
        Users::find().all(&self.db_pool).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(&self.db_pool).await
    }

    async fn create_user(&self, input: &CreateUserRequest) -> Result<users::Model, DbErr> {
        let user = users::ActiveModel {
            firstname: Set(input.firstname.clone()),
            lastname: Set(input.lastname.clone()),
            email: Set(input.email.clone()),
            password: Set(input.password.clone()),
            noc_transfer: Set(input
                .noc_transfer
                .as_ref()
                .map(|s| s.clone())
                .unwrap_or_default()),
            ..Default::default()
        };

        user.insert(&self.db_pool).await
    }

    async fn update_user(&self, input: &UpdateUserRequest) -> Result<users::Model, DbErr> {
        let id = match input.id {
            Some(id) => id,
            None => return Err(DbErr::Custom("User ID is required".to_string())),
        };

        let mut user: users::ActiveModel = users::Entity::find_by_id(id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::Custom("User not found".to_string()))?
            .into();

        if let Some(firstname) = &input.firstname {
            user.firstname = Set(firstname.clone());
        }

        if let Some(lastname) = &input.lastname {
            user.lastname = Set(lastname.clone());
        }

        if let Some(email) = &input.email {
            user.email = Set(email.clone());
        }

       
        user.update(&self.db_pool).await
    }

    async fn delete_user(&self, id: i32) -> Result<(), DbErr> {
        let user: users::ActiveModel = users::Entity::find()
            .filter(users::Column::UserId.eq(id))
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::Custom("User not found".to_string()))?
            .into();

        user.delete(&self.db_pool).await.map(|_| ())
    }
}
