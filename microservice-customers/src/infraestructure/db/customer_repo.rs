use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::error::SqlState;

use crate::application::dto::customer_dto::CreateCustomerRequest;
use crate::domain::customer::Customer;
use crate::infraestructure::error::{ApiError, MyError};

pub async fn get_customers(client: &Client) -> Result<Vec<Customer>, MyError> {
    let stmt = include_str!("../../../sql/get_customers.sql");
    let stmt = stmt.replace("$table_fields", &Customer::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Customer::from_row_ref(row).unwrap())
        .collect::<Vec<Customer>>();

    Ok(results)
}

pub async fn create_customer(
    client: &Client,
    customer_create_request: CreateCustomerRequest,
) -> Result<Customer, MyError> {
    let raw_sql = include_str!("../../../sql/create_customer.sql");
    let sql = raw_sql.replace("$table_fields", &Customer::sql_table_fields());

    let stmt = client.prepare(&sql).await.map_err(MyError::from)?;

    println!("SQL: {}", sql);
    println!(
        "Params: {:?} {:?} {:?}",
        customer_create_request.full_name,
        customer_create_request.email,
        customer_create_request.cpf
    );

    let result = client
        .query(
            &stmt,
            &[
                &customer_create_request.full_name,
                &customer_create_request.email,
                &customer_create_request.cpf,
            ],
        )
        .await;


        
    match result {
        Ok(rows) => {
            let row = rows.first().ok_or(MyError::NotFound)?;
            Ok(Customer::from_row_ref(row)?)
        }
        Err(e) => {
            if let Some(db_err) = e.as_db_error() {
                match db_err.code() {
                    &SqlState::UNIQUE_VIOLATION => {
                        return Err(MyError::ApiError(
                            ApiError {
                                msg: format!(
                            "Email ou CPF já existe: {}",
                            db_err.detail().unwrap_or("sem detalhe")
                        )
                    }));
                    }
                    _ => {}
                }
            }
            Err(MyError::from(e))
        }
    }
}
