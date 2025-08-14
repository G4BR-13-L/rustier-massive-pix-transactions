use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "customers")]
pub struct Customer {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub cpf: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
