use tracing::{error, info};

use async_trait::async_trait;

use crate::{
    abstract_trait::{
        saldo::{DynSaldoRepository, SaldoServiceTrait},
        user::DynUserRepository,
    },
    domain::{
        request::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
        response::{saldo::SaldoResponse, ApiResponse, ErrorResponse},
    },
   
    utils::errors::AppError,
};

pub struct SaldoService {
    user_repository: DynUserRepository,
    saldo_repository: DynSaldoRepository,
}

impl SaldoService {
    pub fn new(user_repository: DynUserRepository, saldo_repository: DynSaldoRepository) -> Self {
        Self {
            user_repository,
            saldo_repository,
        }
    }
}

#[async_trait]
impl SaldoServiceTrait for SaldoService {
    async fn get_saldos(&self) -> Result<ApiResponse<Vec<SaldoResponse>>, ErrorResponse> {
        let saldo = self
            .saldo_repository
            .find_all()
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let saldo_response: Vec<SaldoResponse> = saldo
            .into_iter()
            .map(|saldo| SaldoResponse::from(saldo))
            .collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Saldos retrieved successfully".to_string(),
            data: saldo_response,
        })
    }

    async fn get_saldo(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<SaldoResponse>>, ErrorResponse>{
        let saldo = self
            .saldo_repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if let Some(saldo) = saldo {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: "Saldo retrieved successfully".to_string(),
                data: Some(SaldoResponse::from(saldo)),
            })
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!(
                "Saldo with id {} not found",
                id
            ))))
        }
    }

    async fn get_saldo_users(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<Vec<SaldoResponse>>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let saldo = self
            .saldo_repository
            .find_by_users_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let saldo_responses: Option<Vec<SaldoResponse>> = if saldo.is_empty() {
            None
        } else {
            Some(
                saldo
                    .into_iter()
                    .filter_map(|s| s.map(SaldoResponse::from))
                    .collect(),
            )
        };

        let response = ApiResponse {
            status: "success".to_string(),
            data: saldo_responses,
            message: "Success".to_string(),
        };

        Ok(response)
    }

    async fn get_saldo_user(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<SaldoResponse>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let saldo: Option<SaldoResponse> = self
            .saldo_repository
            .find_by_users_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .into_iter()
            .next()
            .flatten()
            .map(SaldoResponse::from);

        // Create the response object
        let response = ApiResponse {
            status: "success".to_string(),
            data: saldo,
            message: "Success".to_string(),
        };

        Ok(response)
    }

    async fn create_saldo(
        &self,
        input: &CreateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for saldo create: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        let _user = self
            .user_repository
            .find_by_id(input.user_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.user_id
                )))
            })?;

        info!("Saldo created successfully for user_id: {}", input.user_id);

        

        let saldo = self
            .saldo_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Saldo created successfully".to_string(),
            data: SaldoResponse::from(saldo),
        })
    }

    async fn update_saldo(
        &self,
        input: &UpdateSaldoRequest,
    ) -> Result<ApiResponse<Option<SaldoResponse>>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for saldo update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        let _user = self
            .user_repository
            .find_by_id(input.user_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.user_id
                )))
            })?;

        let existing_saldo = self
            .saldo_repository
            .find_by_id(input.saldo_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match existing_saldo {
            Some(_) => {
                let updated_saldo = self
                    .saldo_repository
                    .update(input)
                    .await
                    .map_err(AppError::from)
                    .map_err(ErrorResponse::from)?;

                info!("Saldo updated successfully for id: {}", input.saldo_id);

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Saldo updated successfully".to_string(),
                    data: Some(SaldoResponse::from(updated_saldo)),
                })
            }
            None => {
                error!("Saldo with id {} not found", input.saldo_id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with id {} not found",
                    input.saldo_id
                ))))
            }
        }
    }

    async fn delete_saldo(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        let user = self
            .user_repository
            .find_by_id(id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    id
                )))
            })?;

        let existing_saldo = self
            .saldo_repository
            .find_by_user_id(user.unwrap().user_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match existing_saldo {
            Some(_) => {
                self.saldo_repository
                    .delete(existing_saldo.unwrap().saldo_id)
                    .await
                    .map_err(AppError::from)
                    .map_err(ErrorResponse::from)?;

                info!("Saldo deleted successfully for id: {}", id);

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Saldo deleted successfully".to_string(),
                    data: (),
                })
            }
            None => {
                error!("Saldo with id {} not found", id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with id {} not found",
                    id
                ))))
            }
        }
    }
}
