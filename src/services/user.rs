use tracing::{error, info};

use crate::{abstract_trait::user::{DynUserRepository, UserServiceTrait}, config::hashing::Hashing, domain::{request::{auth::RegisterRequest, user::{CreateUserRequest, UpdateUserRequest}}, response::{user::UserResponse, ApiResponse, ErrorResponse}}, utils::{errors::AppError, random_vcc::random_vcc}};

use async_trait::async_trait;

pub struct UserService {
    repository: DynUserRepository,
    hashing: Hashing
}

impl UserService {
    pub fn new(repository: DynUserRepository, hashing: Hashing) -> Self {
        Self { repository, hashing }
    }
}



#[async_trait]
impl UserServiceTrait for UserService{
    async fn get_users(&self) -> Result<ApiResponse<Vec<UserResponse>>, ErrorResponse>{
        let users = self.repository.find_all().await.map_err(AppError::from).map_err(ErrorResponse::from)?;

        let users_response: Vec<UserResponse> = users.into_iter().map(|users| UserResponse::from(users)).collect();
    
       
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Users retrieved successfully".to_string(),
            data: users_response,
        })
    }

    async fn find_by_id(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<UserResponse>>, ErrorResponse>  {
        let user = self.repository.find_by_id(id).await.map_err(AppError::from).map_err(ErrorResponse::from)?;
        
        if let Some(user) = user {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: "User retrieved successfully".to_string(),
                data: Some(UserResponse::from(user)),
            })
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id))))
        }
    }

    async fn create_user(
        &self,
        input: &RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, ErrorResponse> {
        info!("Attempting to register user with email: {}", input.email);

        let exists = self
            .repository
            .find_by_email_exists(&input.email)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if exists {
            error!("Email already exists: {}", input.email);
            return Err(ErrorResponse::from(AppError::EmailAlreadyExists));
        }

        if let Err(validation_err) = input.validate() {
            error!("Validation failed for user create: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(validation_err)));
        }

        let hashed_password = self
            .hashing
            .hash_password(&input.password)
            .await
            .map_err(|e| ErrorResponse::from(AppError::HashingError(e)))?;

        let noc_transfer = random_vcc().map(|value| Some(value)).unwrap_or(None);

        let request = CreateUserRequest {
            firstname: input.firstname.clone(),
            lastname: input.lastname.clone(),
            email: input.email.clone(),
            password: hashed_password,
            confirm_password: input.confirm_password.clone(),
            noc_transfer: noc_transfer.to_owned(),
        };

        info!("Creating user with email: {}", input.email);
        let create_user = self
            .repository
            .create_user(&request)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        info!("User Create successfully with email: {}", input.email);

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User Create successfully".to_string(),
            data: UserResponse::from(create_user),
        })
    }

    async fn update_user(
        &self,
        input: &UpdateUserRequest,
    ) -> Result<Option<ApiResponse<UserResponse>>, ErrorResponse> {
        let user = self.repository.update_user(input).await.map_err(AppError::from).map_err(ErrorResponse::from)?;

        if let Err(validation_err) = input.validate() {
            error!("Validation failed for user update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(validation_err)));
        }
        
        Ok(Some(ApiResponse {
            status: "success".to_string(),
            message: "User updated successfully".to_string(),
            data: UserResponse::from(user),
        }))
    }

    async fn delete_user(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        self.repository.delete_user(id).await.map_err(AppError::from).map_err(ErrorResponse::from)?;
        
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User deleted successfully".to_string(),
            data: (),
        })
    }
}