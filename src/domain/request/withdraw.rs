use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWithdrawRequest {
    pub user_id: i32,
    pub withdraw_amount: i32,
    pub withdraw_time: NaiveDateTime,
}

impl CreateWithdrawRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.user_id <= 0 {
            return Err("User ID must be positive".to_string());
        }

        if self.withdraw_amount <= 0 {
            return Err("Withdraw amount must be positive".to_string());
        }

        if self.withdraw_time > Utc::now().naive_utc() {
            return Err("Withdraw time cannot be in the future".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWithdrawRequest {
    pub user_id: i32,
    pub withdraw_id: i32,
    pub withdraw_amount: i32,
    pub withdraw_time: NaiveDateTime,
}

impl UpdateWithdrawRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.user_id <= 0 {
            return Err("User ID must be positive".to_string());
        }

      
        if self.withdraw_id <= 0 {
            return Err("Withdraw ID must be positive".to_string());
        }

       
        if self.withdraw_amount <= 0 {
            return Err("Withdraw amount must be positive".to_string());
        }

        if self.withdraw_time > Utc::now().naive_utc() {
            return Err("Withdraw time cannot be in the future".to_string());
        }

        Ok(())
    }
}
