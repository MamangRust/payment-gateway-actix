use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::{
    abstract_trait::topup::TopupRepositoryTrait,
    domain::request::topup::{CreateTopupRequest, UpdateTopupAmount, UpdateTopupRequest},
    entities::topups,
};

pub struct TopupRepository {
    db_pool: DatabaseConnection,
}

impl TopupRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl TopupRepositoryTrait for TopupRepository {
    async fn find_all(&self) -> Result<Vec<topups::Model>, DbErr> {
        topups::Entity::find().all(&self.db_pool).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<topups::Model>, DbErr> {
        topups::Entity::find_by_id(id).one(&self.db_pool).await
    }

    async fn find_by_users(&self, id: i32) -> Result<Vec<Option<topups::Model>>, DbErr> {
        topups::Entity::find()
            .filter(topups::Column::UserId.eq(id))
            .all(&self.db_pool)
            .await
            .map(|res| res.into_iter().map(Some).collect())
    }

    async fn find_by_user(&self, id: i32) -> Result<Option<topups::Model>, DbErr> {
        topups::Entity::find()
            .filter(topups::Column::UserId.eq(id))
            .one(&self.db_pool)
            .await
    }

    async fn create(&self, input: &CreateTopupRequest) -> Result<topups::Model, DbErr> {
        let new_topup = topups::ActiveModel {
            user_id: Set(input.user_id),
            topup_no: Set(input.topup_no.clone()),
            topup_amount: Set(input.topup_amount),
            topup_method: Set(input.topup_method.clone()),
            topup_time: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        new_topup.insert(&self.db_pool).await
    }

    async fn update(&self, input: &UpdateTopupRequest) -> Result<topups::Model, DbErr> {
        let mut topup_record: topups::ActiveModel = topups::Entity::find_by_id(input.topup_id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Topup not found".to_owned()))?
            .into();

        topup_record.topup_amount = Set(input.topup_amount);
        topup_record.topup_method = Set(input.topup_method.clone());
        topup_record.topup_time = Set(Utc::now().naive_utc());

        topup_record.update(&self.db_pool).await
    }

    async fn update_amount(&self, input: &UpdateTopupAmount) -> Result<topups::Model, DbErr> {
        let mut topup_record: topups::ActiveModel = topups::Entity::find_by_id(input.topup_id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Topup not found".to_owned()))?
            .into();

        topup_record.topup_amount = Set(input.topup_amount);

        topup_record.update(&self.db_pool).await
    }

    // Delete a topup record by user ID
    async fn delete(&self, id: i32) -> Result<(), DbErr> {
        let result = topups::Entity::delete_many()
            .filter(topups::Column::UserId.eq(id))
            .exec(&self.db_pool)
            .await?;
        if result.rows_affected > 0 {
            Ok(())
        } else {
            Err(DbErr::RecordNotFound("Topup not found".to_owned()))
        }
    }
}
