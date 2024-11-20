use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};


use crate::{abstract_trait::withdraw::WithdrawRepositoryTrait, domain::request::withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest}, entities::withdraws};

pub struct WithdrawRepository {
    db_pool: DatabaseConnection,
}

impl WithdrawRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl WithdrawRepositoryTrait for WithdrawRepository {
    async fn find_all(&self) -> Result<Vec<withdraws::Model>, DbErr> {
        withdraws::Entity::find().all(&self.db_pool).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<withdraws::Model>, DbErr> {
        withdraws::Entity::find_by_id(id).one(&self.db_pool).await
    }

    async fn find_by_users(&self, id: i32) -> Result<Option<Vec<withdraws::Model>>, DbErr> {
        let results = withdraws::Entity::find()
            .filter(withdraws::Column::UserId.eq(id))
            .all(&self.db_pool)
            .await?;
        Ok(Some(results))
    }

    async fn find_by_user(&self, id: i32) -> Result<Option<withdraws::Model>, DbErr> {
        withdraws::Entity::find()
            .filter(withdraws::Column::UserId.eq(id))
            .one(&self.db_pool)
            .await
    }

    async fn create(&self, input: &CreateWithdrawRequest) -> Result<withdraws::Model, DbErr> {
        let withdraw_time_naive = input.withdraw_time.naive_utc();

        let new_withdraw = withdraws::ActiveModel {
            user_id: Set(input.user_id),
            withdraw_amount: Set(input.withdraw_amount),
            withdraw_time: Set(withdraw_time_naive),
            ..Default::default()
        };

        new_withdraw.insert(&self.db_pool).await
    }

    async fn update(&self, input: &UpdateWithdrawRequest) -> Result<withdraws::Model, DbErr> {
        let mut withdraw_record: withdraws::ActiveModel =
            withdraws::Entity::find_by_id(input.withdraw_id)
                .one(&self.db_pool)
                .await?
                .ok_or(DbErr::RecordNotFound("Withdraw not found".to_owned()))?
                .into();

        let withdraw_time_naive = input.withdraw_time.naive_utc();

        withdraw_record.withdraw_amount = Set(input.withdraw_amount);

        withdraw_record.withdraw_time = Set(withdraw_time_naive);

        withdraw_record.update(&self.db_pool).await
    }

    async fn delete(&self, id: i32) -> Result<(), DbErr> {
        let result = withdraws::Entity::delete_many()
            .filter(withdraws::Column::UserId.eq(id))
            .exec(&self.db_pool)
            .await?;
        if result.rows_affected > 0 {
            Ok(())
        } else {
            Err(DbErr::RecordNotFound("Withdraw not found".to_owned()))
        }
    }
}

