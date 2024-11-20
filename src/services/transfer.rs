use async_trait::async_trait;
use tracing::{error, info};

use crate::{
    abstract_trait::{
        saldo::DynSaldoRepository,
        transfer::{DynTransferRepository, TransferServiceTrait},
        user::DynUserRepository,
    },
    domain::{
        request::{
            saldo::UpdateSaldoBalance,
            transfer::{CreateTransferRequest, UpdateTransferRequest},
        },
        response::{transfer::TransferResponse, ApiResponse, ErrorResponse},
    },
    utils::errors::AppError,
};

pub struct TransferService {
    transfer_repository: DynTransferRepository,
    saldo_repository: DynSaldoRepository,
    user_repository: DynUserRepository,
}

impl TransferService {
    pub fn new(
        transfer_repository: DynTransferRepository,
        saldo_repository: DynSaldoRepository,
        user_repository: DynUserRepository,
    ) -> Self {
        Self {
            transfer_repository,
            saldo_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl TransferServiceTrait for TransferService {
    async fn get_transfers(&self) -> Result<ApiResponse<Vec<TransferResponse>>, ErrorResponse> {
        let transfer = self
            .transfer_repository
            .find_all()
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let transfer_response: Vec<TransferResponse> = transfer
            .into_iter()
            .map(|transfer| TransferResponse::from(transfer))
            .collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfer retrieved successfully".to_string(),
            data: transfer_response,
        })
    }

    async fn get_transfer(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse> {
        let transfer = self
            .transfer_repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if let Some(transfer) = transfer {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: "Transfer retrieved successfully".to_string(),
                data: Some(TransferResponse::from(transfer)),
            })
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!(
                "Transfer with id {} not found",
                id
            ))))
        }
    }

    async fn get_transfer_users(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<Vec<TransferResponse>>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let transfer = self
            .transfer_repository
            .find_by_users(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let transfer_response: Option<Vec<TransferResponse>> =
            transfer.map(|transfers| transfers.into_iter().map(TransferResponse::from).collect());

        let response = ApiResponse {
            status: "success".to_string(),
            data: transfer_response,
            message: "Success".to_string(),
        };

        Ok(response)
    }

    async fn get_transfer_user(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let transfer: Option<TransferResponse> = self
            .transfer_repository
            .find_by_user(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .map(TransferResponse::from);

        let response = ApiResponse {
            status: "success".to_string(),
            data: transfer,
            message: "Success".to_string(),
        };

        Ok(response)
    }

    async fn create_transfer(
        &self,
        input: &CreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for transfer create: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        // Check if sender and receiver exist
        self.user_repository
            .find_by_id(input.transfer_from)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.transfer_from
                )))
            })?;

        self.user_repository
            .find_by_id(input.transfer_to)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.transfer_to
                )))
            })?;

        // Create the transfer
        let transfer = self
            .transfer_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        // Sender's saldo adjustment
        let sender_saldo = self
            .saldo_repository
            .find_by_user_id(input.transfer_from)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with User id {} not found",
                    input.transfer_from
                )))
            })?;

        let sender_balance = sender_saldo.unwrap().total_balance - input.transfer_amount;

        let request_sender_balance = UpdateSaldoBalance {
            user_id: input.transfer_from,
            total_balance: sender_balance,
        };

        if let Err(db_err) = self
            .saldo_repository
            .update_balance(&request_sender_balance)
            .await
        {
            error!("Failed to update saldo balance for sender: {}", db_err);
            self.transfer_repository
                .delete(transfer.transfer_id)
                .await
                .map_err(AppError::from)
                .map_err(ErrorResponse::from)?;

            return Err(ErrorResponse::from(AppError::from(db_err)));
        }

        let receiver_saldo = self
            .saldo_repository
            .find_by_user_id(input.transfer_to)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with User id {} not found",
                    input.transfer_to
                )))
            })?;

        let receiver_balance = receiver_saldo.unwrap().total_balance + input.transfer_amount;

        let request_receiver_balance = UpdateSaldoBalance {
            user_id: input.transfer_to,
            total_balance: receiver_balance,
        };

        if let Err(db_err) = self
            .saldo_repository
            .update_balance(&request_receiver_balance)
            .await
        {
            error!("Failed to update saldo balance for receiver: {}", db_err);
            self.transfer_repository
                .delete(transfer.transfer_id) // Corrected rollback
                .await
                .map_err(AppError::from)
                .map_err(ErrorResponse::from)?;

            return Err(ErrorResponse::from(AppError::from(db_err)));
        }

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfer created successfully".to_string(),
            data: TransferResponse::from(transfer),
        })
    }

    async fn update_transfer(
        &self,
        input: &UpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ErrorResponse> {
        // Validate input
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for transfer update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        // Retrieve the existing transfer
        let transfer = self
            .transfer_repository
            .find_by_id(input.transfer_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Transfer with id {} not found",
                    input.transfer_id
                )))
            })?
            .ok_or_else(|| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Transfer with id {} not found",
                    input.transfer_id
                )))
            })?;

        // Calculate the difference in transfer amount
        let amount_difference = input.transfer_amount as i64 - transfer.transfer_amount as i64;

        // Update sender's saldo
        let sender_saldo = self
            .saldo_repository
            .find_by_user_id(transfer.transfer_from)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with User id {} not found",
                    transfer.transfer_from
                )))
            })?
            .ok_or_else(|| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with User id {} not found",
                    transfer.transfer_from
                )))
            })?;

        let new_sender_balance = sender_saldo.total_balance - amount_difference as i32;

        if new_sender_balance < 0 {
            return Err(ErrorResponse::from(AppError::ValidationError(
                "Insufficient balance for sender".to_string(),
            )));
        }

        let update_sender_balance = UpdateSaldoBalance {
            user_id: transfer.transfer_from,
            total_balance: new_sender_balance,
        };

        if let Err(db_err) = self
            .saldo_repository
            .update_balance(&update_sender_balance)
            .await
        {
            error!("Failed to update sender's saldo: {}", db_err);
            return Err(ErrorResponse::from(AppError::from(db_err)));
        }

        // Update receiver's saldo
        let receiver_saldo = self
            .saldo_repository
            .find_by_user_id(transfer.transfer_to)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with User id {} not found",
                    transfer.transfer_to
                )))
            })?
            .ok_or_else(|| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with User id {} not found",
                    transfer.transfer_to
                )))
            })?;

        let new_receiver_balance = receiver_saldo.total_balance + amount_difference as i32;

        let update_receiver_balance = UpdateSaldoBalance {
            user_id: transfer.transfer_to,
            total_balance: new_receiver_balance,
        };

        if let Err(db_err) = self
            .saldo_repository
            .update_balance(&update_receiver_balance)
            .await
        {
            error!("Failed to update receiver's saldo: {}", db_err);

            // Rollback sender's saldo update
            let rollback_sender_balance = UpdateSaldoBalance {
                user_id: transfer.transfer_from,
                total_balance: sender_saldo.total_balance,
            };

            self.saldo_repository
                .update_balance(&rollback_sender_balance)
                .await
                .map_err(|rollback_err| {
                    error!("Failed to rollback sender's saldo update: {}", rollback_err);
                })
                .ok();

            return Err(ErrorResponse::from(AppError::from(db_err)));
        }

        // Update the transfer record
        let updated_transfer = self
            .transfer_repository
            .update(input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfer updated successfully".to_string(),
            data: TransferResponse::from(updated_transfer),
        })
    }

    async fn delete_transfer(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        let user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let existing_transfer = self
            .transfer_repository
            .find_by_user(user.unwrap().user_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match existing_transfer {
            Some(_) => {
                self.transfer_repository
                    .delete(existing_transfer.unwrap().transfer_id)
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
