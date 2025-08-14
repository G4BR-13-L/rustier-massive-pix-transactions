use super::enums::PixKeyType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixKey {
    pub id: Uuid,
    pub account_id: Uuid,
    pub key_type: PixKeyType,
    pub key_value: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub deactivated_at: Option<DateTime<Utc>>,
}
