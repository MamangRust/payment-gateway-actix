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
            withdraw_amount: None,
            withdraw_time: None,
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
            withdraw_amount: None,
            withdraw_time: None,
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
    ) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse> {
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
                    "Sender with id {} not found",
                    input.transfer_from
                )))
            })?;
    
        self.user_repository
            .find_by_id(input.transfer_to)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Receiver with id {} not found",
                    input.transfer_to
                )))
            })?;
    
        // Get the existing transfer record
        let existing_transfer = self
            .transfer_repository
            .find_by_id(input.transfer_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Transfer with id {} not found",
                    input.transfer_id
                )))
            })?;
    
       
        let existing_transfer_value = existing_transfer
            .clone()
            .expect("Transfer not found");
    
        // Calculate the amount difference for updating balances
        let amount_difference = input.transfer_amount - existing_transfer_value.transfer_amount;
    
        // Update the transfer record
        let updated_transfer = self
            .transfer_repository
            .update(input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;
    
        // Update sender's balance
        let sender_saldo = self
            .saldo_repository
            .find_by_user_id(input.transfer_from)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo for Sender with User id {} not found",
                    input.transfer_from
                )))
            })?
            .unwrap(); // Handle unwrap properly
    
        let sender_new_balance = sender_saldo.total_balance - amount_difference;
    
        let sender_balance_update = UpdateSaldoBalance {
            withdraw_amount: None,
            withdraw_time: None,
            user_id: input.transfer_from,
            total_balance: sender_new_balance,
        };
    
        if let Err(db_err) = self
            .saldo_repository
            .update_balance(&sender_balance_update)
            .await
        {
            error!("Failed to update sender saldo balance: {}", db_err);
    
            // Rollback transfer update
            let existing_transfer_update = UpdateTransferRequest {
                transfer_id: input.transfer_id,
                transfer_from: existing_transfer_value.transfer_from.clone(),
                transfer_to: existing_transfer_value.transfer_to.clone(),
                transfer_amount: existing_transfer_value.transfer_amount.clone(),
            };
    
            self.transfer_repository
                .update(&existing_transfer_update) // Rollback to original transfer details
                .await
                .map_err(AppError::from)
                .map_err(ErrorResponse::from)?;
    
            return Err(ErrorResponse::from(AppError::from(db_err)));
        }
    
        // Update receiver's balance
        let receiver_saldo = self
            .saldo_repository
            .find_by_user_id(input.transfer_to)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo for Receiver with User id {} not found",
                    input.transfer_to
                )))
            })?
            .unwrap(); // Handle unwrap properly
    
        let receiver_new_balance = receiver_saldo.total_balance + amount_difference;
    
        let receiver_balance_update = UpdateSaldoBalance {
            withdraw_amount: None,
            withdraw_time: None,
            user_id: input.transfer_to,
            total_balance: receiver_new_balance,
        };
    
        if let Err(db_err) = self
            .saldo_repository
            .update_balance(&receiver_balance_update)
            .await
        {
            error!("Failed to update receiver saldo balance: {}", db_err);
    
            // Rollback transfer update
            let existing_transfer_update = UpdateTransferRequest {
                transfer_id: input.transfer_id,
                transfer_from: existing_transfer_value.transfer_from.clone(),
                transfer_to: existing_transfer_value.transfer_to.clone(),
                transfer_amount: existing_transfer_value.transfer_amount.clone(),
            };
    
            self.transfer_repository
                .update(&existing_transfer_update) // Rollback to original transfer details
                .await
                .map_err(AppError::from)
                .map_err(ErrorResponse::from)?;
    
            return Err(ErrorResponse::from(AppError::from(db_err)));
        }
    
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfer updated successfully".to_string(),
            data: Some(TransferResponse::from(updated_transfer)),
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
