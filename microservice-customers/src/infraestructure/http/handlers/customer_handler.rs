use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::application::customer_service;
use crate::application::dto::customer_dto::{CreateCustomerRequest, CustomerResponse};
use crate::infraestructure::error::MyError;
use crate::{domain::customer::Customer, infraestructure::db::customer_repo};
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

pub async fn create_customer(
    customer: web::Json<CreateCustomerRequest>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let customer_info: CreateCustomerRequest = customer.into_inner();

    let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    match customer_service::create_customer(&mut client, customer_info).await {
        Ok(new_customer) => {
            log::info!("Customer created successfully: {:?}", new_customer.id);
            Ok(HttpResponse::Created()
                .append_header(("Location", format!("/customers/{}", new_customer.id)))
                .json(CustomerResponse::from(new_customer)))
        }
        Err(e) => {
            log::error!("Error creating customer: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn get_customer_by_id(db_pool: web::Data<Pool>, path: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let customer = customer_service::get_customer_by_id(&client, id).await?;

    Ok(HttpResponse::Ok().json(customer))
}

pub async fn get_customers(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let customers = customer_repo::get_customers(&client).await?;

    Ok(HttpResponse::Ok().json(customers))
}
