use deadpool_postgres::{Client, Transaction};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{application::dto::account_dto::AccountCreateRequest, domain::account::Account, infraestructure::error::{map_db_error, InternalError, MyError}};

pub async fn create_account(
    client_or_tx: &Transaction<'_>, account: AccountCreateRequest) -> Result<Account, MyError> {
    let raw_sql = include_str!("../../../sql/create_account.sql");
    let sql = raw_sql.replace("$table_fields", &Account::sql_table_fields());
    let stmt = client_or_tx.prepare(&sql).await.map_err(map_db_error)?;

    let rows = client_or_tx
        .query(&stmt, &[
            &account.customer_id, 
            &account.account_type, 
            &account.currency
            ]
        )
        .await
        .map_err(map_db_error)?;

    let row = rows.first().ok_or(MyError::Internal(InternalError {
        msg: "Nenhum registro retornado".into(),
    }))?;
    Account::from_row_ref(row).map_err(|e| MyError::Internal(InternalError { msg: e.to_string() }))
}
