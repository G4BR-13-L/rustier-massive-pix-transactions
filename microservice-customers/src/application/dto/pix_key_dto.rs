use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::pix_key::PixKeyType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePixKeyRequest {
    pub key_type: PixKeyType,
    pub key_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PixKeyResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub key_type: PixKeyType,
    pub key_value: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub deactivated_at: Option<DateTime<Utc>>,
}

impl From<crate::domain::pix_key::PixKey> for PixKeyResponse {
    fn from(pix_key: crate::domain::pix_key::PixKey) -> Self {
        PixKeyResponse {
            id: pix_key.id,
            account_id: pix_key.account_id,
            key_type: pix_key.key_type,
            key_value: pix_key.key_value,
            is_active: pix_key.is_active,
            created_at: pix_key.created_at,
            deactivated_at: pix_key.deactivated_at,
        }
    }
}