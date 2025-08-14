use crate::product;
use crate::product::product::{Product, ProductInput};
use crate::product::service;
use chrono::NaiveDateTime;
use rocket::{get, http::Status, post, response::status, routes, serde::json::Json, Route};

#[get("/products")]
async fn list() -> Result<Json<Vec<Product>>, Status> {
    match service::list_products().await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/products/<uuid>")]
async fn find_one(uuid: &str) -> Result<Json<Product>, Status> {
    match service::find_by_uuid(&uuid).await {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/products/<uuid>")]
async fn delete_one(uuid: &str) -> Result<Json<&str>, Status> {
    match service::delete_by_uuid(&uuid).await {
        Ok(()) => Ok(Json(uuid)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/products", format = "json", data = "<input>")]
async fn create(input: Json<ProductInput>) -> Result<Json<Product>, Status> {
    let input = input.into_inner();

    match service::create(input).await {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn product_routes() -> Vec<Route> {
    routes![list, create, find_one, delete_one]
}
