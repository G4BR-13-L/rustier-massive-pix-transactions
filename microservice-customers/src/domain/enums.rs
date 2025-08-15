
use std::fmt;

use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "account_type")]
pub enum AccountType {
    CHECKING,
    SAVINGS,
    PAYMENT,
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountType::CHECKING => write!(f, "CHECKING"),
            AccountType::SAVINGS => write!(f, "SAVINGS"),
            AccountType::PAYMENT => write!(f, "PAYMENT"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PixKeyType {
    CPF,
    CNPJ,
    EMAIL,
    PHONE,
    EVP,
}
