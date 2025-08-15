use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use configuration::db::connect_to_db;
use configuration::migrations::{check_table_exists, create_migration_table, run_migrations};
use confik::{Configuration, EnvSource};
use dotenvy::dotenv;
use env_logger::Env;
use infraestructure::http::routes::config as routes_config;
use tokio_postgres::NoTls;

use crate::configuration::db::ExampleConfig;

pub mod configuration;
pub mod shared;

mod application {
    pub mod account_service;
    pub mod customer_service;
    pub mod dto;
    pub mod pix_service;
}

mod domain {
    pub mod account;
    pub mod customer;
    pub mod enums;
    pub mod pix_key;
}

mod infraestructure {
    pub mod http {
        pub mod handlers;
        pub mod routes;
    }
    pub mod db {
        pub mod account_repo;
        pub mod customer_repo;
        pub mod pix_key_repo;
    }

    pub mod auth{
        pub mod auth;
    }
    pub mod error;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes_config)
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}

fn expect_or_exit<T, E: std::fmt::Display>(result: Result<T, E>, msg: &str) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("{}: {}", msg, e);
        std::process::exit(1);
    })
}
