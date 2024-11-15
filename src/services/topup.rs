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
    
        // Create topup record
        let topup = self
            .topup_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;
    
        // Attempt to get user's current saldo
        match self.saldo_repository.find_by_user_id(input.user_id).await {
            Ok(Some(current_saldo)) => {
                // Calculate new balance
                let request = UpdateSaldoBalance {
                    user_id: input.user_id,
                    withdraw_amount: None,
                    withdraw_time: None,
                    total_balance: current_saldo.total_balance + topup.topup_amount,
                };
    
                // Update saldo
                if let Err(db_err) = self.saldo_repository.update_balance(&request).await {
                    error!("Failed to update saldo balance: {}", db_err);
                    
                    // Rollback topup creation on failure
                    if let Err(rb_err) = self.topup_repository.delete(topup.topup_id).await {
                        error!("Failed to rollback topup: {}", rb_err);
                    }
    
                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }
            }
            Ok(None) => {
                // Create initial saldo if none exists
                let create_saldo_request = CreateSaldoRequest {
                    user_id: input.user_id,
                    total_balance: topup.topup_amount,
                };
    
                if let Err(db_err) = self.saldo_repository.create(&create_saldo_request).await {
                    error!("Failed to create initial saldo: {}", db_err);
    
                    // Rollback topup creation on failure
                    if let Err(rb_err) = self.topup_repository.delete(topup.topup_id).await {
                        error!("Failed to rollback topup: {}", rb_err);
                    }
    
                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }
            }
            Err(_) => {
                error!("Failed to retrieve saldo for user {}", input.user_id);
                
                // Rollback topup creation on failure
                if let Err(rb_err) = self.topup_repository.delete(topup.topup_id).await {
                    error!("Failed to rollback topup: {}", rb_err);
                }
    
                return Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                ))));
            }
        }
    
        // Return success response
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Topup created successfully".to_string(),
            data: TopupResponse::from(topup),
        })
    }
    

    async fn update_topup(
        &self,
        input: &UpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ErrorResponse> {
        // Validate input
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for topup update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(validation_err)));
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
    
       
        let topup = self
            .topup_repository
            .find_by_id(input.topup_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Topup with id {} not found",
                    input.topup_id
                )))
            })?;
    
        // Calculate the new topup amount (if necessary)
        let topup_amount_diff = input.topup_amount - topup.clone().unwrap().topup_amount;
    
        // Attempt to get user's current saldo
        match self.saldo_repository.find_by_user_id(input.user_id).await {
            Ok(Some(current_saldo)) => {
             
                let new_balance = current_saldo.total_balance + topup_amount_diff;
                let request = UpdateSaldoBalance {
                    user_id: input.user_id,
                    withdraw_amount: None,
                    withdraw_time: None,
                    total_balance: new_balance,
                };
    
                // Update saldo
                if let Err(db_err) = self.saldo_repository.update_balance(&request).await {
                    let update = UpdateTopupAmount{
                        topup_id: topup.unwrap().topup_id,
                        topup_amount: topup_amount_diff
                    };
    

                    error!("Failed to update saldo balance: {}", db_err);
    
    
                    // Rollback topup update on failure
                    if let Err(rb_err) = self.topup_repository.update_amount(&update).await {
                        error!("Failed to rollback topup update: {}", rb_err);
                    }
    
                    return Err(ErrorResponse::from(AppError::from(db_err)));
                }
            }
            Ok(None) => {
                error!("No saldo found for user {} to update", input.user_id);
    
                let update = UpdateTopupAmount{
                    topup_id: topup.unwrap().topup_id,
                    topup_amount: topup_amount_diff
                };

                // Rollback topup update on failure
                if let Err(rb_err) = self.topup_repository.update_amount(&update).await {
                    error!("Failed to rollback topup update: {}", rb_err);
                }
    
                return Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                ))));
            }
            Err(_) => {
                error!("Failed to retrieve saldo for user {}", input.user_id);
    
                // Rollback topup update on failure
                let update = UpdateTopupAmount{
                    topup_id: topup.unwrap().topup_id,
                    topup_amount: topup_amount_diff
                };

                if let Err(rb_err) = self.topup_repository.update_amount(&update).await {
                    error!("Failed to rollback topup update: {}", rb_err);
                }
    
                return Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                ))));
            }
        }
    
      
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Topup updated successfully".to_string(),
            data: TopupResponse::from(topup.unwrap()),
        })
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
