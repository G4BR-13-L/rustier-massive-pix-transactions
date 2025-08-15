use deadpool_postgres::{Client, Transaction};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::application::dto::customer_dto::CreateCustomerRequest;
use crate::domain::customer::Customer;
use crate::infraestructure::error::{map_db_error, InternalError, MyError};

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
    client_or_tx: &Transaction<'_>,
    req: CreateCustomerRequest,
) -> Result<Customer, MyError>{
    let raw_sql = include_str!("../../../sql/create_customer.sql");
    let sql = raw_sql.replace("$table_fields", &Customer::sql_table_fields());
    let stmt = client_or_tx.prepare(&sql).await.map_err(map_db_error)?;

    let rows = client_or_tx
        .query(&stmt, &[&req.full_name, &req.email, &req.cpf])
        .await
        .map_err(map_db_error)?;

    let row = rows.first().ok_or(MyError::Internal(InternalError { msg: "Nenhum registro retornado".into() }))?;
    Customer::from_row_ref(row).map_err(|e| MyError::Internal(InternalError { msg: e.to_string() }))
}

