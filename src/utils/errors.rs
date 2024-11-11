use bcrypt::BcryptError;
use sea_orm::DbErr;
use thiserror::Error;
use jsonwebtoken::errors::Error as JwtError;
use serde::Serialize;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] DbErr),

    #[error("Hashing error: {0}")]
    HashingError(#[from] BcryptError),


    #[error("Not Found: {0}")]
    NotFound(String),


    #[error("Error Validation: {0}")]
    ValidationError(String),

    #[error("No Password Not Same: {0}")]
    PasswordError(String),

    #[error("Token expired")]
    TokenExpiredError,

    #[error("Token validation error")]
    TokenValidationError,

    #[error("Token generation error")]
    TokenGenerationError(#[from] JwtError),

    #[error("Bcrypt error: {0}")]
    BcryptError(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Email already exists")]
    EmailAlreadyExists,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, Error)]
pub enum ConnectionManagerError {
    #[error("Database connection failed: {0}")]
    ConnectionError(#[from] DbErr),

    #[error("Migration error: {0}")]
    MigrationError(DbErr),
}

impl ConnectionManagerError {
    pub fn from_migration_error(err: DbErr) -> Self {
        ConnectionManagerError::MigrationError(err)
    }
}
