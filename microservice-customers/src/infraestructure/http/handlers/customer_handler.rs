use std::sync::Arc;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::application::dto::customer_dto::{CreateCustomerRequest, CustomerResponse};
use crate::application::jwt_service::JwtService;
use crate::application::{customer_service};
use crate::infraestructure::error::{ApiError, MyError};
use crate::{domain::customer::Customer, infraestructure::db::customer_repo};
use deadpool_postgres::{Client, Pool};

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

pub async fn get_customer_by_id(
    db_pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let customer = customer_service::get_customer_by_id(&client, id).await?;

    Ok(HttpResponse::Ok().json(customer))
}

pub async fn get_customers(
    req: HttpRequest,
    db_pool: web::Data<Pool>,
    jwt_service: web::Data<Arc<JwtService>>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let jwt_service = jwt_service.as_ref();

    // Extrair header quando necessário
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    println!("Auth Header: {:?}", auth_header.clone());

    let extracted_uuid = jwt_service
        .extract_uuid_from_header(auth_header)
        .map_err(|_| {
            MyError::ApiError(ApiError {
                msg: "Invalid or missing JWT token".to_string(),
            })
        })?;

    println!("\n\n\nExtracted UUID: {}\n\n\n\n", extracted_uuid);

    let customers = customer_repo::get_customers(&client).await?;

    Ok(HttpResponse::Ok().json(customers))
}
