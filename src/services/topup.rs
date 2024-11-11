use crate::{
    abstract_trait::{
        saldo::DynSaldoRepository,
        topup::{DynTopupRepository,  TopupServiceTrait},
        user::DynUserRepository,
    },
    domain::{
        request::{
            saldo::{CreateSaldoRequest, UpdateSaldoBalance},
            topup::{CreateTopupRequest, UpdateTopupRequest},
        },
        response::{topup::TopupResponse, ApiResponse, ErrorResponse},
    },
    utils::errors::AppError,
};
use tracing::{error, info};

use async_trait::async_trait;

pub struct TopupService {
    topup_repository: DynTopupRepository,
    saldo_repository: DynSaldoRepository,
    user_repository: DynUserRepository,
}

impl TopupService {
    pub fn new(
        topup_repository: DynTopupRepository,
        saldo_repository: DynSaldoRepository,
        user_repository: DynUserRepository,
    ) -> Self {
        Self {
            topup_repository,
            saldo_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl TopupServiceTrait for TopupService {
    async fn get_topups(&self) -> Result<ApiResponse<Vec<TopupResponse>>, ErrorResponse> {
        let topup = self
            .topup_repository
            .find_all()
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let topup_response: Vec<TopupResponse> = topup
            .into_iter()
            .map(|topup| TopupResponse::from(topup))
            .collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Topup retrieved successfully".to_string(),
            data: topup_response,
        })
    }

    async fn get_topup(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TopupResponse>>, ErrorResponse> {
        let topup = self
            .topup_repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if let Some(topup) = topup {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: "Topup retrieved successfully".to_string(),
                data: Some(TopupResponse::from(topup)),
            })
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!(
                "Topup with id {} not found",
                id
            ))))
        }
    }

    async fn get_topup_users(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<Vec<TopupResponse>>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let topup = self
            .topup_repository
            .find_by_users(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let topup_response: Option<Vec<TopupResponse>> = if topup.is_empty() {
            None
        } else {
            Some(
                topup
                    .into_iter()
                    .filter_map(|s| s.map(TopupResponse::from))
                    .collect(),
            )
        };

        let response = ApiResponse {
            status: "success".to_string(),
            data: topup_response,
            message: "Success ".to_string(),
        };

        Ok(response)
    }

    async fn get_topup_user(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TopupResponse>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let topup: Option<TopupResponse> = self
            .topup_repository
            .find_by_user(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .map(TopupResponse::from);

        let response = ApiResponse {
            status: "success".to_string(),
            data: topup,
            message: "Success".to_string(),
        };

        Ok(response)
    }

    async fn create_topup(
        &self,
        input: &CreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ErrorResponse> {
        // Validate input
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for topup create: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        // Verify user exists
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

        // Create topup record first
        let topup = self
            .topup_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        // Get user's current saldo
        let saldo = self
            .saldo_repository
            .find_by_user_id(input.user_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                )))
            })?;

        // Handle saldo update
        match saldo {
            Some(current_saldo) => {
                // Calculate new balance by adding topup amount
                let request = UpdateSaldoBalance {
                    user_id: input.user_id,
                    withdraw_amount: None, 
                    withdraw_time: None,  
                    total_balance: current_saldo.total_balance + topup.topup_amount,
                };

                // Update saldo balance
                if let Err(db_err) = self.saldo_repository.update_balance(&request).await {
                    // If saldo update fails, rollback topup creation
                    error!("Failed to update saldo balance: {}", db_err);
                    self.topup_repository
                        .delete(topup.topup_id)
                        .await
                        .map_err(AppError::from)
                        .map_err(ErrorResponse::from)?;

                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }

                info!(
                    "Topup successful: Amount {} for user_id {}",
                    topup.topup_amount, input.user_id
                );
            }
            None => {
                // If no saldo exists, create new saldo record with topup amount as initial balance
                let create_saldo_request = CreateSaldoRequest {
                    user_id: input.user_id,
                    total_balance: topup.topup_amount,
                };

                if let Err(db_err) = self.saldo_repository.create(&create_saldo_request).await {
                    // If saldo creation fails, rollback topup creation
                    error!("Failed to create initial saldo: {}", db_err);
                    self.topup_repository
                        .delete(topup.topup_id)
                        .await
                        .map_err(AppError::from)
                        .map_err(ErrorResponse::from)?;

                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }

                info!(
                    "Created initial saldo with topup amount {} for user_id {}",
                    topup.topup_amount, input.user_id
                );
            }
        }

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Topup created successfully".to_string(),
            data: TopupResponse::from(topup),
        })
    }

    async fn update_topup(
        &self,
        input: &UpdateTopupRequest,
    ) -> Result<ApiResponse<Option<TopupResponse>>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for topup update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        // find user id
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

        let saldo = self
            .saldo_repository
            .find_by_user_id(input.user_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user id {} not found",
                    input.user_id
                )))
            })?;

        let existing_topup = self
            .topup_repository
            .find_by_id(input.topup_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with id {} not found",
                    input.user_id
                )))
            })?;

        match existing_topup {
            Some(ref topup) => {
                let updated_saldo = self
                    .topup_repository
                    .update(input)
                    .await
                    .map_err(AppError::from)
                    .map_err(ErrorResponse::from)?;

                info!("Topup updated successfully for id: {}", input.topup_id);

                let request = UpdateSaldoBalance {
                    user_id: input.user_id,
                    withdraw_amount: None, 
                    withdraw_time: None,  
                    total_balance: saldo.unwrap().total_balance - topup.topup_amount, 
                };

                
                if let Err(db_err) = self.saldo_repository.update_balance(&request).await {
                    error!("Failed to update saldo balance: {}", db_err);
                    self.topup_repository
                        .delete(topup.topup_id) 
                        .await
                        .map_err(AppError::from)
                        .map_err(ErrorResponse::from)?;

                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }

                info!(
                    "Topup successful: Amount {} for user_id {}",
                    topup.topup_amount, input.user_id
                );

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Topup updated successfully".to_string(),
                    data: Some(TopupResponse::from(updated_saldo)),
                })
            }
            None => {
                error!("Topup with id {} not found", input.topup_id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with id {} not found",
                    input.topup_id
                ))))
            }
        }
    }

    async fn delete_topup(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        let user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let existing_topup = self
            .topup_repository
            .find_by_user(user.unwrap().user_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match existing_topup {
            Some(_) => {
                self.topup_repository
                    .delete(existing_topup.unwrap().topup_id)
                    .await
                    .map_err(AppError::from)
                    .map_err(ErrorResponse::from)?;

                info!("Topup deleted successfully for id: {}", id);

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Topup deleted successfully".to_string(),
                    data: (),
                })
            }
            None => {
                error!("Topup with id {} not found", id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with id {} not found",
                    id
                ))))
            }
        }
    }
}
