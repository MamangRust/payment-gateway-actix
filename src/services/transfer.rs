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
            transfer::{CreateTransferRequest, UpdateTransferAmountRequest, UpdateTransferRequest},
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
    ) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse>{
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
    
        
        let transfer_response: Option<Vec<TransferResponse>> = transfer.map(|transfers| {
            transfers
                .into_iter()
                .map(TransferResponse::from)
                .collect()
        });
    
        
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

        let transfer = self
            .transfer_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        // sender Saldo
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

        // sender balance
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
            error!("Failed to update saldo balance sender: {}", db_err);
            self.transfer_repository
                .delete(input.transfer_from)
                .await
                .map_err(AppError::from)
                .map_err(ErrorResponse::from)?;

            return Err(ErrorResponse::from(AppError::from(db_err)));
        }

        // receiver saldo
        let receiver_saldo = self
            .saldo_repository
            .find_by_user_id(input.transfer_from)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with User id {} not found",
                    input.transfer_to
                )))
            })?;

        // receiver saldo
        let receiver_balance = receiver_saldo.unwrap().total_balance + input.transfer_amount;

        // request receiver saldo
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
            error!("Failed to update saldo balance receiver: {}", db_err);
            self.transfer_repository
                .delete(input.transfer_to)
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
        // Validate input
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for transfer update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(validation_err)));
        }
    
        // Check if sender and receiver exist
        let _sender = self
            .user_repository
            .find_by_id(input.transfer_from)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.transfer_from
                )))
            })?;
    
        let _receiver = self
            .user_repository
            .find_by_id(input.transfer_to)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "User with id {} not found",
                    input.transfer_to
                )))
            })?;
    
        // Find and verify existing transfer
        let existing_transfer = self
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
                error!("Transfer with id {} not found", input.transfer_id);
                ErrorResponse::from(AppError::NotFound(format!(
                    "Transfer with id {} not found",
                    input.transfer_id
                )))
            })?;
    
        // Get sender's saldo
        let sender_saldo = self
            .saldo_repository
            .find_by_user_id(input.transfer_from)
            .await
            .map_err(|err| {
                error!("Failed to get sender saldo: {:?}", err);
                ErrorResponse::from(AppError::NotFound("Failed to get sender saldo".to_string()))
            })?
            .ok_or_else(|| {
                ErrorResponse::from(AppError::NotFound("Sender saldo not found".to_string()))
            })?;
    
      
        if sender_saldo.total_balance < input.transfer_amount {
            return Err(ErrorResponse::from(AppError::NotFound(
                "Insufficient balance for transfer".to_string(),
            )));
        }
    
        // Get receiver's saldo
        let receiver_saldo = self
            .saldo_repository
            .find_by_user_id(input.transfer_to)
            .await
            .map_err(|err| {
                error!("Failed to get receiver saldo: {:?}", err);
                ErrorResponse::from(AppError::NotFound("Failed to get receiver saldo".to_string()))
            })?
            .ok_or_else(|| {
                ErrorResponse::from(AppError::NotFound("Receiver saldo not found".to_string()))
            })?;
    
        // Calculate new balances
        let sender_new_balance = sender_saldo.total_balance - input.transfer_amount;
        let receiver_new_balance = receiver_saldo.total_balance + input.transfer_amount;
    
   
        let update_transfer = match self.transfer_repository.update(input).await {
            Ok(transfer) => transfer,
            Err(err) => {
                error!("Failed to update transfer: {:?}", err);
                return Err(ErrorResponse::from(AppError::from(err)));
            }
        };
    
      
        if let Err(err) = self
            .saldo_repository
            .update_balance(&UpdateSaldoBalance {
                withdraw_amount: None, 
                withdraw_time: None,  
                user_id: input.transfer_from,
                total_balance: sender_new_balance,
            })
            .await
        {
            error!("Failed to update sender saldo: {:?}", err);
            
            if let Err(rollback_err) = self
                .transfer_repository
                .update_amount(&UpdateTransferAmountRequest {
                    transfer_id: input.transfer_id,
                    transfer_amount: existing_transfer.transfer_amount,
                })
                .await
            {
                error!("Failed to rollback transfer: {:?}", rollback_err);
            }
            return Err(ErrorResponse::from(AppError::NotFound(
                "Failed to update sender saldo".to_string(),
            )));
        }
    
        
        if let Err(err) = self
            .saldo_repository
            .update_balance(&UpdateSaldoBalance {
                withdraw_amount: None, 
                withdraw_time: None,  
                user_id: input.transfer_to,
                total_balance: receiver_new_balance,
            })
            .await
        {
            error!("Failed to update receiver saldo: {:?}", err);
            
            if let Err(rollback_err) = self
                .transfer_repository
                .update_amount(&UpdateTransferAmountRequest {
                    transfer_id: input.transfer_id,
                    transfer_amount: existing_transfer.transfer_amount,
                })
                .await
            {
                error!("Failed to rollback transfer: {:?}", rollback_err);
            }
            if let Err(rollback_err) = self
                .saldo_repository
                .update_balance(&UpdateSaldoBalance {
                    withdraw_amount: None, 
                    withdraw_time: None,  
                    user_id: input.transfer_from,
                    total_balance: sender_saldo.total_balance,
                })
                .await
            {
                error!("Failed to rollback sender balance: {:?}", rollback_err);
            }
            return Err(ErrorResponse::from(AppError::NotFound(
                "Failed to update receiver saldo".to_string(),
            )));
        }
    
       
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Transfer and saldo updates successful".to_string(),
            data: Some(TransferResponse::from(update_transfer)),
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
