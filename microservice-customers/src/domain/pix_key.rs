use std::fmt;

use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "pix_keys")]
pub struct PixKey {
    pub id: Uuid,
    pub account_id: Uuid,
    pub key_type: PixKeyType,
    pub key_value: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub deactivated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "pix_key_type")]
pub enum PixKeyType {
    CPF,
    CNPJ,
    EMAIL,
    PHONE,
    RANDOM,
}

impl fmt::Display for PixKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixKeyType::CPF => write!(f, "CPF"),
            PixKeyType::CNPJ => write!(f, "CNPJ"),
            PixKeyType::EMAIL => write!(f, "EMAIL"),
            PixKeyType::PHONE => write!(f, "PHONE"),
            PixKeyType::RANDOM => write!(f, "RANDOM"),
        }
    }
}
