use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::DbErr;

use crate::{domain::{request::{auth::RegisterRequest, user::{CreateUserRequest, UpdateUserRequest}}, response::{user::UserResponse, ApiResponse, ErrorResponse}}, entities::users};


pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<users::Model>, DbErr>;
    async fn find_by_email_exists(&self, email: &str) -> Result<bool, DbErr>;
    async fn create_user(
        &self,
        input: &CreateUserRequest
    ) -> Result<users::Model, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<users::Model>, DbErr>;
    async fn update_user(
        &self,
        input: &UpdateUserRequest
    ) -> Result<users::Model, DbErr>;
    async fn delete_user(&self, id: i32) -> Result<(), DbErr>;
}


#[async_trait]
pub trait UserServiceTrait{
    async fn get_users(&self) -> Result<ApiResponse<Vec<UserResponse>>, ErrorResponse>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<Option<UserResponse>>, ErrorResponse>;
    async fn create_user(
        &self,
        input: &RegisterRequest
    ) -> Result<ApiResponse<UserResponse>, ErrorResponse>;
    async fn update_user(
        &self,
        input: &UpdateUserRequest
    ) -> Result<Option<ApiResponse<UserResponse>>, ErrorResponse>;
    async fn delete_user(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}