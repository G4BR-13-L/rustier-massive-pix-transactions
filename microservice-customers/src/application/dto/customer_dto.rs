use serde::{Deserialize, Serialize};

use crate::domain::customer::Customer;

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub full_name: String,
    pub email: String,
    pub cpf: String,
}

#[derive(Debug, Serialize)]
pub struct CustomerResponse {
    pub id: uuid::Uuid,
    pub full_name: String,
    pub email: String,
    pub cpf: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Customer> for CustomerResponse {
    fn from(c: Customer) -> Self {
        Self { 
            id: c.id,
            full_name: c.full_name,
            email: c.email,
            cpf: c.cpf,
            is_active: c.is_active,
            created_at: c.created_at,
        }
    }
}