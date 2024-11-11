use std::sync::Arc;
use sea_orm::DbErr;
use async_trait::async_trait;

use crate::{domain::{request::transfer::{CreateTransferRequest, UpdateTransferAmountRequest, UpdateTransferRequest}, response::{transfer::TransferResponse, ApiResponse, ErrorResponse}}, entities::transfers};


pub type DynTransferRepository = Arc<dyn TransferRepositoryTrait + Send + Sync>;
pub type DynTransferService = Arc<dyn TransferServiceTrait + Send + Sync>;



#[async_trait]
pub trait TransferRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<transfers::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<transfers::Model>, DbErr>;
    async fn find_by_users(&self, id: i32) -> Result<Option<Vec<transfers::Model>>, DbErr> ;
    async fn find_by_user(&self, id: i32) ->  Result<Option<transfers::Model>, DbErr>;
    async fn create(&self, input: &CreateTransferRequest) -> Result<transfers::Model, DbErr>;
    async fn update(&self, input: &UpdateTransferRequest) -> Result<transfers::Model, DbErr>;
    async fn update_amount(&self, input: &UpdateTransferAmountRequest) -> Result<transfers::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait TransferServiceTrait {
    async fn get_transfers(&self) -> Result<ApiResponse<Vec<TransferResponse>>, ErrorResponse>;
    async fn get_transfer(&self, id: i32) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse>;
    async fn get_transfer_users(&self, id: i32) -> Result<ApiResponse<Option<Vec<TransferResponse>>>, ErrorResponse>;
    async fn get_transfer_user(&self, id: i32) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse> ;
    async fn create_transfer(&self, input: &CreateTransferRequest) -> Result<ApiResponse<TransferResponse>, ErrorResponse>;
    async fn update_transfer(&self, input: &UpdateTransferRequest) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse> ;
    async fn delete_transfer(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}