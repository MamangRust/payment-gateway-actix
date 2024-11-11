use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::transfers;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferResponse {
    pub transfer_id: i32,
    pub transfer_from: i32,
    pub transfer_to: i32,
    pub transfer_amount: i32,
    pub transfer_time: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<transfers::Model> for TransferResponse {
    fn from(value: transfers::Model) -> Self {
        TransferResponse {
            transfer_id: value.transfer_id,
            transfer_from: value.transfer_from,
            transfer_to: value.transfer_to,
            transfer_amount: value.transfer_amount,
            transfer_time: Utc.from_utc_datetime(&value.transfer_time),
            created_at: value.created_at.map(|dt| Utc.from_utc_datetime(&dt)),
            updated_at: value.updated_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}
