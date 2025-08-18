use std::sync::Arc;

use actix_web::{Error, web, HttpRequest, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{application::{dto::pix_key_dto::{CreatePixKeyRequest, PixKeyResponse}, jwt_service::JwtService, pix_service}, infraestructure::error::{ApiError, MyError}};


pub async fn list_pix_keys() -> HttpResponse {
    HttpResponse::Ok().json("List of PIX keys")
}

pub async fn create_pix_key(
    req: HttpRequest,
    db_pool: web::Data<Pool>,
    pix_key: web::Json<CreatePixKeyRequest>,
    jwt_service: web::Data<Arc<JwtService>>,
) -> Result<HttpResponse, Error> {

    println!("\n\n\nCreating PIX key...\n\n\n");
    let mut pix_key_info: CreatePixKeyRequest = pix_key.into_inner();

    let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    println!("Auth Header: {:?}", auth_header.clone());

    let customer_uuid = jwt_service
        .extract_uuid_from_header(auth_header)
        .map_err(|_| {
            MyError::ApiError(ApiError {
                msg: "Invalid or missing JWT token".to_string(),
            })
        })?;

    println!("\n\n\nExtracted UUID: {}\n\n\n\n", customer_uuid);

    match pix_service::create_pix_key(&mut client, &mut pix_key_info, customer_uuid).await {
        Ok(new_pix_key) => {
            log::info!("PIX key created successfully: {:?}", new_pix_key.id);
            Ok(HttpResponse::Created()
                .append_header(("Location", format!("/pix-keys/{}", new_pix_key.id)))
                .json(PixKeyResponse::from(new_pix_key)))
        }
        Err(e) => {
            log::error!("Error creating PIX key: {:?}", e);
            Err(e.into())
        }
    }
}