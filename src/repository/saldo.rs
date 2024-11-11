use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter,
    Set,
};

use crate::{
    abstract_trait::saldo::SaldoRepositoryTrait,
    domain::request::saldo::{CreateSaldoRequest, UpdateSaldoBalance, UpdateSaldoRequest},
    entities::saldo,
};

pub struct SaldoRepository {
    db_pool: DatabaseConnection,
}

impl SaldoRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl SaldoRepositoryTrait for SaldoRepository {
    async fn find_all(&self) -> Result<Vec<saldo::Model>, DbErr> {
        saldo::Entity::find().all(&self.db_pool).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<saldo::Model>, DbErr> {
        saldo::Entity::find()
            .filter(saldo::Column::SaldoId.eq(id))
            .one(&self.db_pool)
            .await
    }

    async fn find_by_user_id(&self, id: i32) -> Result<Option<saldo::Model>, DbErr> {
        saldo::Entity::find()
            .filter(saldo::Column::UserId.eq(id))
            .one(&self.db_pool)
            .await
    }

    async fn find_by_users_id(&self, id: i32) -> Result<Vec<Option<saldo::Model>>, DbErr> {
        saldo::Entity::find()
            .filter(saldo::Column::UserId.eq(id))
            .all(&self.db_pool)
            .await
            .map(|res| res.into_iter().map(Some).collect()) // Wrap each result in `Some` to match `Vec<Option<saldo::Model>>`
    }

    async fn create(&self, input: &CreateSaldoRequest) -> Result<saldo::Model, DbErr> {
        let new_saldo = saldo::ActiveModel {
            user_id: Set(input.user_id),
            total_balance: Set(input.total_balance),
            ..Default::default()
        };
        new_saldo.insert(&self.db_pool).await
    }

    async fn update(&self, input: &UpdateSaldoRequest) -> Result<saldo::Model, DbErr> {
        let mut saldo_record: saldo::ActiveModel = saldo::Entity::find_by_id(input.saldo_id)
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Saldo not found".to_owned()))?
            .into();

        saldo_record.total_balance = Set(input.total_balance);
        saldo_record.withdraw_amount = Set(Some(input.withdraw_amount.unwrap_or_default()));
        saldo_record.withdraw_time =
            Set(Some(input.withdraw_time.unwrap_or(Utc::now().naive_utc())));

        saldo_record.update(&self.db_pool).await
    }

    async fn update_balance(&self, input: &UpdateSaldoBalance) -> Result<saldo::Model, DbErr> {
        let mut saldo_record: saldo::ActiveModel = saldo::Entity::find()
            .filter(saldo::Column::UserId.eq(input.user_id))
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Saldo not found".to_owned()))?
            .into();

        if let Some(withdraw_amount) = input.withdraw_amount {
            saldo_record.withdraw_amount = Set(Some(withdraw_amount));
        }

       
        if let Some(withdraw_time) = input.withdraw_time {
            saldo_record.withdraw_time = Set(Some(withdraw_time));
        }

        saldo_record.total_balance = Set(input.total_balance);

        saldo_record.update(&self.db_pool).await
    }

    async fn delete(&self, id: i32) -> Result<(), DbErr> {
        let saldo_record = saldo::Entity::find()
            .filter(saldo::Column::UserId.eq(id))
            .one(&self.db_pool)
            .await?
            .ok_or(DbErr::RecordNotFound("Saldo not found".to_owned()))?;

        saldo_record.delete(&self.db_pool).await.map(|_| ())
    }
}
