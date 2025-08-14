pub mod configuration;
pub mod product;
pub mod tests;
pub mod utils;

use configuration::db::connect_to_db;

#[macro_use]
extern crate rocket;

use configuration::migrations::{check_table_exists, create_migration_table, run_migrations};
use product::controller::product_routes;
use rocket::{get, routes, Build, Rocket};
use std::fs;
use std::path::Path;

#[launch]
async fn rocket() -> Rocket<Build> {
    let client = expect_or_exit(connect_to_db().await, "Failed to connect to database");

    if !expect_or_exit(check_table_exists(&client).await, "Failed to check table") {
        expect_or_exit(
            create_migration_table(&client).await,
            "Failed to create table",
        );
    }

    expect_or_exit(run_migrations(&client).await, "Error in migrations");

    rocket::build().mount("/", product_routes())
}

fn expect_or_exit<T, E: std::fmt::Display>(result: Result<T, E>, msg: &str) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("{}: {}", msg, e);
        std::process::exit(1);
    })
}
