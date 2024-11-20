use crate::{
    abstract_trait::{
        saldo::DynSaldoRepository,
        topup::{DynTopupRepository,  TopupServiceTrait},
        user::DynUserRepository,
    },
    domain::{
        request::{
            saldo::{CreateSaldoRequest, UpdateSaldoBalance},
            topup::{CreateTopupRequest, UpdateTopupAmount, UpdateTopupRequest},
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
            .map_err(ErrorResponse::from);

        match topup {
            Ok(topup) => {
                let topup_response: Vec<TopupResponse> = topup
                    .into_iter()
                    .map(|topup| TopupResponse::from(topup))
                    .collect();

                info!("Successfully retrieved {} topups.", topup_response.len());

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Topup retrieved successfully".to_string(),
                    data: topup_response,
                })
            }
            Err(err) => {
                error!("Failed to fetch topups: {}", err);
                Err(err)
            }
        }
    }

    async fn get_topup(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TopupResponse>>, ErrorResponse> {
        info!("Fetching topup with id {}", id);

        let topup = self
            .topup_repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from);

        match topup {
            Ok(Some(topup)) => {
                info!("Successfully retrieved topup with id {}", id);
                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Topup retrieved successfully".to_string(),
                    data: Some(TopupResponse::from(topup)),
                })
            }
            Ok(None) => {
                error!("Topup with id {} not found", id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with id {} not found",
                    id
                ))))
            }
            Err(err) => {
                error!("Error fetching topup with id {}: {}", id, err);
                Err(err)
            }
        }
    }

    async fn get_topup_users(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<Vec<TopupResponse>>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            error!("User with id {} not found", id);
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let topup = self
            .topup_repository
            .find_by_users(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from);

        match topup {
            Ok(topup) => {
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

                if topup_response.is_none() {
                    info!("No topups found for user with id {}", id);

                    let response = ApiResponse {
                        status: "success".to_string(),
                        data: None,
                        message: format!("No topup found for user with id {}", id),
                    };

                    return Ok(response);
                }

                info!("Successfully retrieved topups for user with id {}", id);
                Ok(ApiResponse {
                    status: "success".to_string(),
                    data: topup_response,
                    message: "Success".to_string(),
                })
            }
            Err(err) => {
                error!("Failed to fetch topups for user with id {}: {}", id, err);
                Err(err)
            }
        }
    }

    async fn get_topup_user(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TopupResponse>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            error!("User with id {} not found", id);
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let topup: Option<TopupResponse> = self
            .topup_repository
            .find_by_user(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .map(TopupResponse::from);

        match topup {
            Some(topup) => {
                info!("Successfully retrieved topup for user with id {}", id);

                Ok(ApiResponse {
                    status: "success".to_string(),
                    data: Some(topup),
                    message: "Success".to_string(),
                })
            }
            None => {
                info!("No topup found for user with id {}", id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with user id {} not found",
                    id
                ))))
            }
        }
    }

    async fn create_topup(
        &self,
        input: &CreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for topup create: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        let _user = self
            .user_repository
            .find_by_id(input.user_id)
            .await
            .map_err(|_| {
                error!("User with id {} not found", input.user_id);
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.user_id
                )))
            })?;

        info!(
            "User with id {} found, proceeding with topup creation",
            input.user_id
        );

        let topup = self
            .topup_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        info!(
            "Topup created for user with id {}: topup amount {}",
            input.user_id, topup.topup_amount
        );

        match self.saldo_repository.find_by_user_id(input.user_id).await {
            Ok(Some(current_saldo)) => {
                let new_balance = current_saldo.total_balance + topup.topup_amount;
                let request = UpdateSaldoBalance {
                    user_id: input.user_id,
                    total_balance: new_balance,
                };

                if let Err(db_err) = self.saldo_repository.update_balance(&request).await {
                    error!(
                        "Failed to update saldo balance for user {}: {}",
                        input.user_id, db_err
                    );

                    if let Err(rb_err) = self.topup_repository.delete(topup.topup_id).await {
                        error!(
                            "Failed to rollback topup creation for user {}: {}",
                            input.user_id, rb_err
                        );
                    }

                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }

                info!(
                    "Saldo updated successfully for user {}. New balance: {}",
                    input.user_id, new_balance
                );
            }
            Ok(None) => {
                let create_saldo_request = CreateSaldoRequest {
                    user_id: input.user_id,
                    total_balance: topup.topup_amount,
                };

                if let Err(db_err) = self.saldo_repository.create(&create_saldo_request).await {
                    error!(
                        "Failed to create initial saldo for user {}: {}",
                        input.user_id, db_err
                    );

                    if let Err(rb_err) = self.topup_repository.delete(topup.topup_id).await {
                        error!(
                            "Failed to rollback topup creation for user {}: {}",
                            input.user_id, rb_err
                        );
                    }

                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }

                info!(
                    "Initial saldo created for user {} with balance {}",
                    input.user_id, topup.topup_amount
                );
            }
            Err(_) => {
                error!("Failed to retrieve saldo for user {}", input.user_id);

                if let Err(rb_err) = self.topup_repository.delete(topup.topup_id).await {
                    error!(
                        "Failed to rollback topup creation for user {}: {}",
                        input.user_id, rb_err
                    );
                }

                return Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                ))));
            }
        }

        info!(
            "Topup successfully created for user {}. Total balance updated.",
            input.user_id
        );
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

        info!(
            "Validation passed for topup update for user {} and topup_id {}",
            input.user_id, input.topup_id
        );

        let _user = self
            .user_repository
            .find_by_id(input.user_id)
            .await
            .map_err(|_| {
                error!("User with id {} not found", input.user_id);
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.user_id
                )))
            })?;

        info!(
            "User with id {} found, proceeding with topup update",
            input.user_id
        );

        let existing_topup = self
            .topup_repository
            .find_by_id(input.topup_id)
            .await
            .map_err(|_| {
                error!("Topup with id {} not found", input.topup_id);
                ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with id {} not found",
                    input.topup_id
                )))
            })?;

        let existing_topup = existing_topup.ok_or_else(|| {
            error!("Topup with id {} not found", input.topup_id);
            ErrorResponse::from(AppError::NotFound(format!(
                "Topup with id {} not found",
                input.topup_id
            )))
        })?;

        let topup_difference = input.topup_amount - existing_topup.topup_amount;

        info!(
            "Calculating topup difference: new amount {} - old amount {} = difference {}",
            input.topup_amount, existing_topup.topup_amount, topup_difference
        );

        let update_topup = UpdateTopupAmount {
            topup_id: input.topup_id,
            topup_amount: input.topup_amount,
        };

        self.topup_repository
            .update_amount(&update_topup)
            .await
            .map_err(|e| {
                error!("Failed to update topup amount: {}", e);
                ErrorResponse::from(AppError::from(e))
            })?;

        match self.saldo_repository.find_by_user_id(input.user_id).await {
            Ok(Some(current_saldo)) => {
                let new_balance = current_saldo.total_balance + topup_difference;

                info!(
                    "Updating saldo: current balance {} + topup difference {} = new balance {}",
                    current_saldo.total_balance, topup_difference, new_balance
                );

                let request = UpdateSaldoBalance {
                    user_id: input.user_id,
                    total_balance: new_balance,
                };

                // Update saldo balance
                if let Err(db_err) = self.saldo_repository.update_balance(&request).await {
                    error!(
                        "Failed to update saldo balance for user {}: {}",
                        input.user_id, db_err
                    );

                    // Rollback topup update on failure
                    let rollback = UpdateTopupAmount {
                        topup_id: existing_topup.topup_id,
                        topup_amount: existing_topup.topup_amount,
                    };

                    if let Err(rb_err) = self.topup_repository.update_amount(&rollback).await {
                        error!(
                            "Failed to rollback topup update for user {}: {}",
                            input.user_id, rb_err
                        );
                    }

                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }

                info!(
                    "Saldo updated successfully for user {}. New balance: {}",
                    input.user_id, new_balance
                );
            }
            Ok(None) => {
                error!("No saldo found for user {} to update", input.user_id);
                return Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo for user {} not found",
                    input.user_id
                ))));
            }
            Err(e) => {
                error!("Failed to retrieve saldo for user {}: {}", input.user_id, e);
                return Err(ErrorResponse::from(AppError::from(e)));
            }
        }

        let updated_topup = self
            .topup_repository
            .find_by_id(input.topup_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match updated_topup {
            Some(topup) => Ok(ApiResponse {
                status: "success".to_string(),
                message: "Topup updated successfully".to_string(),
                data: Some(TopupResponse::from(topup)),
            }),
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
