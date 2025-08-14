
use configuration::db::connect_to_db;
use configuration::migrations::{check_table_exists, create_migration_table, run_migrations};
use confik::{Configuration, EnvSource};
use tokio_postgres::NoTls;
use std::fs;
use std::path::Path;
use dotenvy::dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use infraestructure::http::routes::config as routes_config;

use crate::configuration::db::ExampleConfig;

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
    pub mod db {
        pub mod customer_repo;
        pub mod account_repo;
        pub mod pix_key_repo;
    }
    pub mod error;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        
    dotenv().ok();

    let client = expect_or_exit(connect_to_db().await, "Failed to connect to database");
    
    if !expect_or_exit(check_table_exists(&client).await, "Failed to check table") {
        expect_or_exit(
            create_migration_table(&client).await,
            "Failed to create table",
        );
    }

    expect_or_exit(run_migrations(&client).await, "Error in migrations");

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone()))
        .configure(routes_config)
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await

    // HttpServer::new(|| {
    //     App::new()
    //         .configure(routes_config)
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
}

fn expect_or_exit<T, E: std::fmt::Display>(result: Result<T, E>, msg: &str) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("{}: {}", msg, e);
        std::process::exit(1);
    })
}
