use crate::{domain::customer::Customer};
use serde::{Deserialize, Serialize};
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
impl CreateCustomerRequest {
    pub fn sanitize_fields(&mut self) {
        self.sanitize_cpf();
        self.full_name = self.full_name.trim().to_string();
        self.email = self.email.trim().to_string();
    }
    fn sanitize_cpf(&mut self) {
        self.cpf = self.cpf.chars().filter(|c| c.is_ascii_digit()).collect();
    }
}
