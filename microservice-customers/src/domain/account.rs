use super::enums::AccountType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "accounts")]
pub struct Account {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub account_type: AccountType,
    pub currency: String,
    pub available_balance: Decimal,
    pub ledger_balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
