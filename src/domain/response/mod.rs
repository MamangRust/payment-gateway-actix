use core::fmt;

use serde::Serialize;

use crate::utils::errors::AppError;

pub mod user;
pub mod saldo;
pub mod topup;
pub mod transfer;
pub mod withdraw;


#[derive(Debug, Serialize)]

pub struct  ApiResponse<T>{
    pub status: String,
    pub message: String,
    pub data: T
}

impl<T: std::fmt::Debug> fmt::Display for ApiResponse<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApiResponse {{ status: {}, message: {}, data: {:?} }}",
            self.status,
            self.message,
            self.data
        )
    }
}
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        let (status, message) = match error {
            AppError::DbError(_) => ("error".to_string(), "Database error occurred".to_string()),
            AppError::PasswordError(ref msg) => ("error".to_string(), msg.clone()),

            AppError::ValidationError(ref msg) => ("Error Validation".to_string(), msg.clone()),

            AppError::HashingError(_) => ("error".to_string(), "Error during password hashing".to_string()),
            AppError::NotFound(ref msg) => ("error".to_string(), msg.clone()),
            AppError::TokenExpiredError => ("error".to_string(), "Token has expired".to_string()),
            AppError::TokenValidationError => ("error".to_string(), "Token validation failed".to_string()),
            AppError::TokenGenerationError(_) => ("error".to_string(), "Token generation failed".to_string()),
            AppError::BcryptError(ref msg) => ("error".to_string(), format!("Bcrypt error: {}", msg)),
            AppError::InvalidCredentials => ("error".to_string(), "Invalid credentials".to_string()),
            AppError::EmailAlreadyExists => ("error".to_string(), "Email already exists".to_string()),
        };
        ErrorResponse { status, message }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}