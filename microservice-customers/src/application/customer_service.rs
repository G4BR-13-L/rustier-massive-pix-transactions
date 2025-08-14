use deadpool_postgres::Client;

use crate::{
    application::dto::customer_dto::CreateCustomerRequest,
    domain::customer::Customer,
    infraestructure::{
        db::customer_repo,
        error::{ApiError, MyError},
    },
    shared::cpf,
};

pub async fn create_customer(
    client: &Client,
    create_customer_request: CreateCustomerRequest,
) -> Result<Customer, MyError> {
    let cpf_validation_result = cpf::validate_cpf(&create_customer_request.cpf);
    match cpf_validation_result {
        Ok(()) => println!("CPF valido"),
        Err(error) => {
            return Err(MyError::ApiError(ApiError {
                msg: error.to_string(),
            }))
        }
    }

    return customer_repo::create_customer(client, create_customer_request).await;
}
