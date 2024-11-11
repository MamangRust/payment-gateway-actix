use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::topups;

#[derive(Debug, Deserialize, Serialize)]
pub struct TopupResponse {
    pub topup_id: i32,
    pub user_id: i32,
    pub topup_no: String,
    pub topup_amount: i32,
    pub topup_method: String,
    pub topup_time: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<topups::Model> for TopupResponse {
    fn from(value: topups::Model) -> Self {
        TopupResponse {
            topup_id: value.topup_id,
            user_id: value.user_id,
            topup_no: value.topup_no,
            topup_amount: value.topup_amount,
            topup_method: value.topup_method,
            topup_time: Utc.from_utc_datetime(&value.topup_time),
            created_at: value.created_at.map(|dt| Utc.from_utc_datetime(&dt)),
            updated_at: value.updated_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}
