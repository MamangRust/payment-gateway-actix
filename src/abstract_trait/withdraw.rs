use std::sync::Arc;
use sea_orm::DbErr;
use async_trait::async_trait;

use crate::{domain::{request::withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest}, response::{withdraw::WithdrawResponse, ApiResponse, ErrorResponse}}, entities::withdraws};


pub type DynWithdrawRepository = Arc<dyn WithdrawRepositoryTrait + Send + Sync>;
pub type DynWithdrawService = Arc<dyn WithdrawServiceTrait + Send + Sync>;



#[async_trait]
pub trait WithdrawRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<withdraws::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<withdraws::Model>, DbErr>;
    async fn find_by_users(&self, id: i32) -> Result<Option<Vec<withdraws::Model>>, DbErr> ;
    async fn find_by_user(&self, id: i32) ->  Result<Option<withdraws::Model>, DbErr>;
    async fn create(&self, input: &CreateWithdrawRequest) -> Result<withdraws::Model, DbErr>;
    async fn update(&self, input: &UpdateWithdrawRequest) -> Result<withdraws::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait WithdrawServiceTrait {
    async fn get_withdraws(&self) -> Result<ApiResponse<Vec<WithdrawResponse>>, ErrorResponse>;
    async fn get_withdraw(&self, id: i32) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse>;
    async fn get_withdraw_users(&self, id: i32) -> Result<ApiResponse<Option<Vec<WithdrawResponse>>>, ErrorResponse>;
    async fn get_withdraw_user(&self, id: i32) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> ;
    async fn create_withdraw(&self, input: &CreateWithdrawRequest) -> Result<ApiResponse<WithdrawResponse>, ErrorResponse>;
    async fn update_withdraw(&self, input: &UpdateWithdrawRequest) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> ;
    async fn delete_withdraw(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}