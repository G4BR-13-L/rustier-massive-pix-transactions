use actix_web::{Error, web, HttpResponse};
use deadpool_postgres::{Client, Pool};
use uuid::Uuid;

use crate::{application::account_service, infraestructure::error::MyError};
pub async fn list_accounts() -> HttpResponse {
    HttpResponse::Ok().json("List of accounts")
}


pub async fn get_account_by_customer_id(db_pool: web::Data<Pool>, path: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let account = account_service::get_account_by_customer_id(&client, id).await?;

    Ok(HttpResponse::Ok().json(account))
}