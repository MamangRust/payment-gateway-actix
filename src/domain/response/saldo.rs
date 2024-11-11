use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::saldo;

#[derive(Debug, Deserialize, Serialize)]
pub struct SaldoResponse{
    pub id: i32,
    pub user_id: i32,
    pub total_balance: i32,
    pub withdraw_amount: Option<i32>,
    pub withdraw_time: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<saldo::Model> for SaldoResponse {
    fn from(value: saldo::Model) -> Self {
        SaldoResponse{
            id: value.saldo_id,
            user_id: value.user_id,
            total_balance: value.total_balance,
            withdraw_amount: value.withdraw_amount,
            withdraw_time: value.withdraw_time.map(|dt| Utc.from_utc_datetime(&dt)),
            created_at: value.created_at.map(|dt| Utc.from_utc_datetime(&dt)),
            updated_at: value.updated_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}