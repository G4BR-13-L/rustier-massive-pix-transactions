use deadpool_postgres::{Client, Transaction};
use uuid::Uuid;

use crate::{application::dto::account_dto::AccountCreateRequest, domain::{account::Account, enums::AccountType}, infraestructure::{db::account_repo, error::MyError}};



pub async fn create_account(
    client_or_tx: &Transaction<'_>,
    customer_id: &Uuid
) -> Result<Account, MyError> {

    let new_account = AccountCreateRequest {
        customer_id: *customer_id,
        account_type: AccountType::PAYMENT,
        currency: String::from("R$")
    };

    return account_repo::create_account(client_or_tx, new_account).await;
}