use serde::{Deserialize, Serialize};

use crate::utils::payment_method_validator::payment_method_validator;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTopupRequest {
    pub user_id: i32,
    pub topup_no: String,
    pub topup_amount: i32,
    pub topup_method: String,
}

impl CreateTopupRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.user_id <= 0 {
            return Err("User ID must be a positive integer".to_string());
        }

        if self.topup_no.is_empty() {
            return Err("Top-up number is required".to_string());
        }

        if self.topup_amount <= 50000 {
            return Err("Topup amount must be greater than or equal to 50000".to_string());
        }

        if self.topup_method.is_empty() {
            return Err("Top-up method is required".to_string());
        }

        if payment_method_validator(&self.topup_method.to_owned()) {
            return Err("Topup method not found".to_string()); 
        }
        

        Ok(())
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTopupRequest {
    pub user_id: i32,
    pub topup_id: i32,
    pub topup_amount: i32,
    pub topup_method: String,
}

impl UpdateTopupRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.user_id <= 0 {
            return Err("User ID must be a positive integer".to_string());
        }

        if self.topup_id <= 0 {
            return Err("Top-up ID must be a positive integer".to_string());
        }

        if self.topup_amount <= 50000 {
            return Err("Topup amount must be greater than or equal to 50000".to_string());
        }

        if self.topup_method.is_empty() {
            return Err("Top-up method is required".to_string());
        }

        if payment_method_validator(&self.topup_method.to_owned()) {
            return Err("Topup method not found".to_string()); 
        }

        Ok(())
    }
}
