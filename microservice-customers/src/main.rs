
use configuration::db::connect_to_db;
use configuration::migrations::{check_table_exists, create_migration_table, run_migrations};
use std::fs;
use std::path::Path;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use infraestructure::http::routes::config;

pub mod configuration;
pub mod utils;

mod domain {
    pub mod enums;
    pub mod customer;
    pub mod account;
    pub mod pix_key;
}

mod infraestructure {
    pub mod http {
        pub mod handlers;
        pub mod routes;
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        
    let client = expect_or_exit(connect_to_db().await, "Failed to connect to database");
    
    if !expect_or_exit(check_table_exists(&client).await, "Failed to check table") {
        expect_or_exit(
            create_migration_table(&client).await,
            "Failed to create table",
        );
    }

    expect_or_exit(run_migrations(&client).await, "Error in migrations");

    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn expect_or_exit<T, E: std::fmt::Display>(result: Result<T, E>, msg: &str) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("{}: {}", msg, e);
        std::process::exit(1);
    })
}
