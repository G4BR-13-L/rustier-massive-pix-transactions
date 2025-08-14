use actix_web::web;

use super::handlers::{
    customer_handler,
    account_handler,
    pix_handler,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // Customer routes
        .service(
            web::scope("/customers")
                .route("", web::get().to(customer_handler::list_customers))
                .route("", web::post().to(customer_handler::create_customer))
                .route("/{id}", web::get().to(customer_handler::get_customer_by_id))
        )
        // Accounts routes
        .service(
            web::scope("/accounts")
                .route("", web::get().to(account_handler::list_accounts))
        )
        // Pix routes
        .service(
            web::scope("/pix-keys")
                .route("", web::get().to(pix_handler::list_pix_keys))
        );
}
