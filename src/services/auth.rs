use async_trait::async_trait;
use tracing::{info, error};

use crate::{
    abstract_trait::{auth::AuthServiceTrait, user::DynUserRepository},
    config::{hashing::Hashing, jwt_config::JwtConfig},
    domain::{
        request::{
            auth::{LoginRequest, RegisterRequest},
            user::CreateUserRequest,
        },
        response::{user::UserResponse, ApiResponse, ErrorResponse},
    },
    utils::{errors::AppError, random_vcc::random_vcc},
};

pub struct AuthService {
    repository: DynUserRepository,
    hashing: Hashing,
    jwt_config: JwtConfig,
}

impl AuthService {
    pub fn new(repository: DynUserRepository, hashing: Hashing, jwt_config: JwtConfig) -> Self {
        Self {
            repository,
            hashing,
            jwt_config,
        }
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn register_user(
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
            error!("Validation failed for user registration: {}", validation_err);
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

        info!("User registered successfully with email: {}", input.email);

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User registered successfully".to_string(),
            data: UserResponse::from(create_user),
        })
    }

    async fn login_user(&self, input: &LoginRequest) -> Result<ApiResponse<String>, ErrorResponse> {
        info!("Attempting to login user with email: {}", input.email);

        let user = self
            .repository
            .find_by_email(&input.email)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .ok_or_else(|| ErrorResponse::from(AppError::NotFound("User not found".to_string())))?;


            if let Err(validation_err) = input.validate() {
                error!("Validation failed for user login: {}", validation_err);
                return Err(ErrorResponse::from(AppError::ValidationError(validation_err)));
            }

        if self
            .hashing
            .compare_password(&user.password, &input.password)
            .await
            .is_err()
        {
            error!("Invalid credentials for email: {}", input.email);
            return Err(ErrorResponse::from(AppError::InvalidCredentials));
        }

        let token = self
            .jwt_config
            .generate_token(user.user_id as i64)
            .map_err(ErrorResponse::from)?;

        info!("User logged in successfully with email: {}", input.email);

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Login successful".to_string(),
            data: token,
        })
    }
}
