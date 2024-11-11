use std::sync::Arc;
use sea_orm::DbErr;
use async_trait::async_trait;

use crate::{domain::{request::saldo::{CreateSaldoRequest, UpdateSaldoBalance, UpdateSaldoRequest}, response::{saldo::SaldoResponse, ApiResponse, ErrorResponse}}, entities::saldo};



pub type DynSaldoRepository = Arc<dyn SaldoRepositoryTrait + Send + Sync>;
pub type DynSaldoService = Arc<dyn SaldoServiceTrait + Send + Sync>;



#[async_trait]
pub trait SaldoRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<saldo::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<saldo::Model>, DbErr>;

    async fn find_by_users_id(&self, id: i32) -> Result<Vec<Option<saldo::Model>>, DbErr>;
    async fn find_by_user_id(&self, id: i32) -> Result<Option<saldo::Model>, DbErr>;
   
    async fn create(&self, input: &CreateSaldoRequest) -> Result<saldo::Model, DbErr>;
    async fn update(&self, input: &UpdateSaldoRequest) -> Result<saldo::Model, DbErr>;
    async fn update_balance(&self, input: &UpdateSaldoBalance) -> Result<saldo::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait SaldoServiceTrait {
    async fn get_saldos(&self) -> Result<ApiResponse<Vec<SaldoResponse>>, ErrorResponse>;
    async fn get_saldo(&self, id: i32) -> Result<ApiResponse<Option<SaldoResponse>>, ErrorResponse>;
    async fn get_saldo_users(&self, id: i32) -> Result<ApiResponse<Option<Vec<SaldoResponse>>>, ErrorResponse>;
    async fn get_saldo_user(&self, id: i32) -> Result<ApiResponse<Option<SaldoResponse>>, ErrorResponse> ;
    async fn create_saldo(&self, input: &CreateSaldoRequest) -> Result<ApiResponse<SaldoResponse>, ErrorResponse>;
    async fn update_saldo(&self, input: &UpdateSaldoRequest) -> Result<ApiResponse<Option<SaldoResponse>>, ErrorResponse>;
    async fn delete_saldo(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}