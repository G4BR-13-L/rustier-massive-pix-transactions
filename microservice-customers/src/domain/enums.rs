use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    CHECKING,
    SAVINGS,
    PAYMENT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PixKeyType {
    CPF,
    CNPJ,
    EMAIL,
    PHONE,
    EVP,
}
