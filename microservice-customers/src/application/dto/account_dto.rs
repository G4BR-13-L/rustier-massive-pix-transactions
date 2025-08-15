use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::AccountType;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreateRequest {
    pub customer_id: Uuid,
    pub account_type: AccountType,
    pub currency: String,
}