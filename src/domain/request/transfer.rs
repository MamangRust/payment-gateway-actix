use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTransferRequest {
    pub transfer_from: i32,
    pub transfer_to: i32,
    pub transfer_amount: i32,
}

impl CreateTransferRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.transfer_from <= 0 {
            return Err("Transfer from must be a positive integer".to_string());
        }

        if self.transfer_to <= 0 {
            return Err("Transfer to must be a positive integer".to_string());
        }

        if self.transfer_amount < 50000 {
            return Err("Transfer amount must be at least 50,000".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTransferRequest {
    pub transfer_id: i32,
    pub transfer_from: i32,
    pub transfer_to: i32,
    pub transfer_amount: i32,
}

impl UpdateTransferRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.transfer_id <= 0 {
            return Err("Transfer ID must be a positive integer".to_string());
        }

        if self.transfer_from <= 0 {
            return Err("Transfer from must be a positive integer".to_string());
        }

        if self.transfer_to <= 0 {
            return Err("Transfer to must be a positive integer".to_string());
        }

        if self.transfer_amount < 50000 {
            return Err("Transfer amount must be at least 50,000".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTransferAmountRequest {
    pub transfer_id: i32,
    pub transfer_amount: i32,
}

impl UpdateTransferAmountRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.transfer_id <= 0 {
            return Err("Transfer ID must be a positive integer".to_string());
        }

        if self.transfer_amount <= 0 {
            return Err("Transfer amount must be greater than zero".to_string());
        }

        Ok(())
    }
}
