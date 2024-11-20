use crate::{
    abstract_trait::{
        saldo::DynSaldoRepository,
        user::DynUserRepository,
        withdraw::{DynWithdrawRepository, WithdrawServiceTrait},
    },
    domain::{
        request::{
            saldo::UpdateSaldoBalance,
            withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest},
        },
        response::{withdraw::WithdrawResponse, ApiResponse, ErrorResponse},
    },
    utils::errors::AppError,
};
use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};

pub struct WithdrawService {
    withdraw_repository: DynWithdrawRepository,
    saldo_repository: DynSaldoRepository,
    user_repository: DynUserRepository,
}

impl WithdrawService {
    pub fn new(
        withdraw_repository: DynWithdrawRepository,
        saldo_repository: DynSaldoRepository,
        user_repository: DynUserRepository,
    ) -> Self {
        Self {
            withdraw_repository,
            saldo_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl WithdrawServiceTrait for WithdrawService {
    async fn get_withdraws(&self) -> Result<ApiResponse<Vec<WithdrawResponse>>, ErrorResponse> {
        let withdraw = self
            .withdraw_repository
            .find_all()
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let withdraw_response: Vec<WithdrawResponse> = withdraw
            .into_iter()
            .map(|withdraw| WithdrawResponse::from(withdraw))
            .collect();

        info!(
            "Successfully fetched {} withdrawals",
            withdraw_response.len()
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Withdraw retrieved successfully".to_string(),
            data: withdraw_response,
        })
    }

    async fn get_withdraw(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> {
        let withdraw = self
            .withdraw_repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if let Some(withdraw) = withdraw {
            info!("Successfully retrieved withdraw with ID: {}", id);
            Ok(ApiResponse {
                status: "success".to_string(),
                message: "Withdraw retrieved successfully".to_string(),
                data: Some(WithdrawResponse::from(withdraw)),
            })
        } else {
            error!("Withdraw with ID {} not found", id);
            Err(ErrorResponse::from(AppError::NotFound(format!(
                "Saldo with id {} not found",
                id
            ))))
        }
    }

    async fn get_withdraw_users(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<Vec<WithdrawResponse>>>, ErrorResponse> {
        // Check if the user exists
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let withdraw = self
            .withdraw_repository
            .find_by_users(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let withdraw_response: Option<Vec<WithdrawResponse>> = match withdraw {
            Some(withdrawals) if !withdrawals.is_empty() => Some(
                withdrawals
                    .into_iter()
                    .map(WithdrawResponse::from)
                    .collect(),
            ),
            _ => None,
        };

        if withdraw_response.is_none() {
            let response = ApiResponse {
                status: "success".to_string(),
                data: None,
                message: format!("No withdraw found for user with id {}", id),
            };

            return Ok(response);
        }

        let response = ApiResponse {
            status: "success".to_string(),
            data: withdraw_response,
            message: "Success".to_string(),
        };

        Ok(response)
    }

    async fn get_withdraw_user(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let withdraw: Option<WithdrawResponse> = self
            .withdraw_repository
            .find_by_user(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .map(WithdrawResponse::from);

        match withdraw {
            Some(withdraw) => {
                info!("Successfully retrieved withdraw for user with id {}", id);

                Ok(ApiResponse {
                    status: "success".to_string(),
                    data: Some(withdraw),
                    message: "Success".to_string(),
                })
            }
            None => {
                info!("No withdraw found for user with id {}", id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with user id {} not found",
                    id
                ))))
            }
        }
    }

    async fn create_withdraw(
        &self,
        input: &CreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ErrorResponse> {
        info!("Creating withdraw for user_id: {}", input.user_id);

        if let Err(validation_err) = input.validate() {
            error!("Validation failed for withdraw create: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }
        info!("Validation passed for withdraw creation");

        let saldo = self
            .saldo_repository
            .find_by_user_id(input.user_id)
            .await
            .map_err(|_| {
                error!("Saldo with user_id {} not found", input.user_id);
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                )))
            })?;

        let saldo_ref = saldo.as_ref().ok_or_else(|| {
            error!("Saldo not found for user_id: {}", input.user_id);
            ErrorResponse::from(AppError::NotFound("Saldo not found".to_string()))
        })?;

        info!(
            "Saldo found for user_id: {}. Current balance: {}",
            input.user_id, saldo_ref.total_balance
        );

        if saldo_ref.total_balance < input.withdraw_amount {
            error!(
                "Insufficient balance for user_id: {}. Attempted withdrawal: {}",
                input.user_id, input.withdraw_amount
            );
            return Err(ErrorResponse::from(AppError::ValidationError(
                "Insufficient balance".to_string(),
            )));
        }
        info!("User has sufficient balance for withdrawal");

        let new_total_balance = saldo_ref.total_balance - input.withdraw_amount;

        let _update_saldo_balance = self
            .saldo_repository
            .update_saldo_withdraw(&UpdateSaldoWithdraw {
                user_id: input.user_id,
                withdraw_amount: Some(input.withdraw_amount),
                withdraw_time: Some(Utc::now().naive_utc()),
                total_balance: new_total_balance,
            })
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        info!(
            "Saldo balance updated for user_id: {}. New balance: {}",
            input.user_id, new_total_balance
        );

        let withdraw_create_result = self
            .withdraw_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        info!(
            "Withdraw created successfully for user_id: {}",
            input.user_id
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Withdraw created successfully".to_string(),
            data: withdraw_create_result.into(),
        })
    }

    async fn update_withdraw(
        &self,
        input: &UpdateWithdrawRequest,
    ) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for withdraw update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        let _withdraw = self
            .withdraw_repository
            .find_by_id(input.withdraw_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Withdraw with id {} not found",
                    input.withdraw_id
                )))
            })?;

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

        let saldo_ref = saldo.as_ref().ok_or_else(|| {
            ErrorResponse::from(AppError::NotFound("Saldo not found".to_string()))
        })?;

        let new_total_balance = saldo_ref.total_balance - input.withdraw_amount;

        let updated_withdraw = self
            .withdraw_repository
            .update(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from);

        if let Err(err) = updated_withdraw {
            let _rollback_saldo = self
                .saldo_repository
                .update_saldo_withdraw(&UpdateSaldoWithdraw {
                    user_id: input.user_id,
                    withdraw_amount: None,
                    withdraw_time: None,
                    total_balance: saldo_ref.total_balance,
                })
                .await
                .map_err(AppError::from)
                .map_err(ErrorResponse::from)?;

            error!("Rollback: Saldo reverted due to withdraw update failure");

            return Err(err);
        }

        let _update_saldo = self
            .saldo_repository
            .update_saldo_withdraw(&UpdateSaldoWithdraw {
                user_id: input.user_id,
                withdraw_amount: Some(input.withdraw_amount),
                withdraw_time: Some(Utc::now().naive_utc()),
                total_balance: new_total_balance,
            })
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Withdraw updated successfully".to_string(),
            data: Some(updated_withdraw.unwrap().into()),
        })
    }

    async fn delete_withdraw(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        let user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let existing = self
            .withdraw_repository
            .find_by_user(user.unwrap().user_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match existing {
            Some(_) => {
                self.withdraw_repository
                    .delete(existing.unwrap().withdraw_id)
                    .await
                    .map_err(AppError::from)
                    .map_err(ErrorResponse::from)?;

                info!("Withdraw deleted successfully for id: {}", id);

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Withdraw deleted successfully".to_string(),
                    data: (),
                })
            }
            None => {
                error!("Withdraw with id {} not found", id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Withdraw with id {} not found",
                    id
                ))))
            }
        }
    }
}
