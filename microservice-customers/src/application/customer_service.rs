use deadpool_postgres::Client;
use uuid::Uuid;

use crate::{
    application::{account_service, dto::customer_dto::CreateCustomerRequest},
    domain::customer::Customer,
    infraestructure::{
        db::customer_repo,
        error::{ApiError, MyError},
    },
    shared::cpf,
};

pub async fn create_customer(
    client: &mut Client,
    mut create_customer_request: CreateCustomerRequest,
) -> Result<Customer, MyError> {

    create_customer_request.sanitize_fields();

    let cpf_validation_result = cpf::validate_cpf(&create_customer_request.cpf);

    match cpf_validation_result {
        Ok(()) => println!("CPF valido"),
        Err(error) => {
            return Err(MyError::ApiError(ApiError {
                msg: error.to_string(),
            }))
        }
    }

    let transaction = client.build_transaction().start().await?;

    let new_customer = customer_repo::create_customer(&transaction, create_customer_request).await?;

    // let new_account = account_service::create_account(&transaction, &new_customer.id).await?;

    transaction.commit().await?;
    Ok(new_customer)

}


pub async fn get_customer_by_id(client: &Client, id: Uuid) -> Result<Customer, MyError> {

    let customer = customer_repo::get_customer_by_id(&client, id).await?;

    Ok(customer)

}