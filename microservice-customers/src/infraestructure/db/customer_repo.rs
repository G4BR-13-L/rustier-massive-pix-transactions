use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::domain::customer::Customer;
use crate::infraestructure::error::MyError;

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
