use actix_web::{web, HttpResponse, Error};
use chrono::Utc;
use uuid::Uuid;

use crate::domain::customer;
use crate::infraestructure::error::MyError;
use crate::{
    domain::customer::Customer, infraestructure::db::customer_repo,
    infraestructure::http::handlers::customer_handler,
};
use deadpool_postgres::{Client, Pool};

pub async fn list_customers() -> HttpResponse {
    HttpResponse::Ok().json(vec![Customer {
        id: Uuid::new_v4(),
        full_name: "João da Silva".into(),
        email: "joao@example.com".into(),
        cpf: "12345678901".into(),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }])
}

pub async fn create_customer(item: web::Json<Customer>) -> HttpResponse {
    HttpResponse::Created().json(item.into_inner())
}

pub async fn get_customer_by_id(path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();
    let customer = Customer {
        id,
        full_name: "Maria Souza".into(),
        email: "maria@example.com".into(),
        cpf: "98765432100".into(),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    HttpResponse::Ok().json(customer)
}

pub async fn get_customers(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let customers = customer_repo::get_customers(&client).await?;

    Ok(HttpResponse::Ok().json(customers))
}
