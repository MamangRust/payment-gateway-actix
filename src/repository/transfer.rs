use crate::{
    abstract_trait::transfer::TransferRepositoryTrait,
    domain::request::transfer::{
        CreateTransferRequest, UpdateTransferAmountRequest, UpdateTransferRequest,
    },
    entities::{transfers, Transfer},
};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

pub struct TransferRepository {
    db_pool: DatabaseConnection,
}

impl TransferRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl TransferRepositoryTrait for TransferRepository {
    async fn find_all(&self) -> Result<Vec<transfers::Model>, DbErr> {
        Transfer::find().all(&self.db_pool).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<transfers::Model>, DbErr> {
        Transfer::find_by_id(id).one(&self.db_pool).await
    }

    async fn find_by_users(&self, id: i32) -> Result<Option<Vec<transfers::Model>>, DbErr> {
        let transfers = Transfer::find()
            .filter(
                transfers::Column::TransferFrom
                    .eq(id)
                    .or(transfers::Column::TransferTo.eq(id)),
            )
            .all(&self.db_pool)
            .await?;

        if transfers.is_empty() {
            Ok(None)
        } else {
            Ok(Some(transfers))
        }
    }

    async fn find_by_user(&self, id: i32) -> Result<Option<transfers::Model>, DbErr> {
        Transfer::find()
            .filter(
                transfers::Column::TransferFrom
                    .eq(id)
                    .or(transfers::Column::TransferTo.eq(id)),
            )
            .one(&self.db_pool)
            .await
    }

    async fn create(&self, input: &CreateTransferRequest) -> Result<transfers::Model, DbErr> {
        let new_transfer = transfers::ActiveModel {
            transfer_from: Set(input.transfer_from),
            transfer_to: Set(input.transfer_to),
            transfer_amount: Set(input.transfer_amount),
            transfer_time: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        new_transfer.insert(&self.db_pool).await
    }

    async fn update(&self, input: &UpdateTransferRequest) -> Result<transfers::Model, DbErr> {
        let transfer = transfers::ActiveModel {
            transfer_id: Set(input.transfer_id),
            transfer_from: Set(input.transfer_from),
            transfer_to: Set(input.transfer_to),
            transfer_amount: Set(input.transfer_amount),
            transfer_time: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        transfer.update(&self.db_pool).await
    }

    async fn update_amount(
        &self,
        input: &UpdateTransferAmountRequest,
    ) -> Result<transfers::Model, DbErr> {
        let transfer = transfers::ActiveModel {
            transfer_id: Set(input.transfer_id),
            transfer_amount: Set(input.transfer_amount),
            ..Default::default()
        };

        transfer.update(&self.db_pool).await
    }

    async fn delete(&self, id: i32) -> Result<(), DbErr> {
        Transfer::delete_by_id(id)
            .exec(&self.db_pool)
            .await
            .map(|_| ())
    }
}
