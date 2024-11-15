use std::sync::Arc;
use sea_orm::DbErr;
use async_trait::async_trait;

use crate::{domain::{request::topup::{CreateTopupRequest, UpdateTopupAmount, UpdateTopupRequest}, response::{topup::TopupResponse, ApiResponse, ErrorResponse}}, entities::topups};



pub type DynTopupRepository = Arc<dyn TopupRepositoryTrait + Send + Sync>;
pub type DynTopupService = Arc<dyn TopupServiceTrait + Send + Sync>;



#[async_trait]
pub trait TopupRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<topups::Model>, DbErr>;
    
    async fn find_by_id(&self, id: i32) -> Result<Option<topups::Model>, DbErr>;
    
    async fn find_by_users(&self, id: i32) -> Result<Vec<Option<topups::Model>>, DbErr>;
    
    async fn find_by_user(&self, id: i32) ->  Result<Option<topups::Model>, DbErr>;

    async fn create(&self, input: &CreateTopupRequest) -> Result<topups::Model, DbErr>;
    
    async fn update(&self, input: &UpdateTopupRequest) -> Result<topups::Model, DbErr>;

    async fn update_amount(&self, input: &UpdateTopupAmount) -> Result<topups::Model, DbErr>;
    
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait TopupServiceTrait {
    async fn get_topups(&self) -> Result<ApiResponse<Vec<TopupResponse>>, ErrorResponse>;
    async fn get_topup(&self, id: i32) -> Result<ApiResponse<Option<TopupResponse>>, ErrorResponse>;
    async fn get_topup_users(&self, id: i32) -> Result<ApiResponse<Option<Vec<TopupResponse>>>, ErrorResponse>;
    async fn get_topup_user(&self, id: i32) -> Result<ApiResponse<Option<TopupResponse>>, ErrorResponse> ;
    async fn create_topup(&self, input: &CreateTopupRequest) -> Result<ApiResponse<TopupResponse>, ErrorResponse>;
    async fn update_topup(&self, input: &UpdateTopupRequest) -> Result<ApiResponse<TopupResponse>, ErrorResponse>;
    async fn delete_topup(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}