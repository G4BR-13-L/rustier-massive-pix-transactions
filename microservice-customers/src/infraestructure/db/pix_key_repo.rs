use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{application::dto::pix_key_dto::CreatePixKeyRequest, domain::pix_key::PixKey, infraestructure::error::{map_db_error, InternalError, MyError}};




pub async fn create_pix_key(
    client: &Client,
    req: &CreatePixKeyRequest,
    account_id: Uuid,
) -> Result<PixKey, MyError>{
    let raw_sql = include_str!("../../../sql/create_pix_key.sql");
    let sql = raw_sql.replace("$table_fields", &PixKey::sql_table_fields());
    let stmt = client.prepare(&sql).await.map_err(map_db_error)?;

    let rows = client
        .query(&stmt, &[&req.key_value, &req.key_type, &account_id])
        .await
        .map_err(map_db_error)?;

    let row = rows.first().ok_or(MyError::Internal(InternalError { msg: "Nenhum registro retornado".into() }))?;

    PixKey::from_row_ref(row).map_err(|e| MyError::Internal(InternalError { msg: e.to_string() }))
}