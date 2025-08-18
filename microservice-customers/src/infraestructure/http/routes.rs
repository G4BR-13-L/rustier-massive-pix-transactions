use actix_web::web;

use crate::infraestructure::auth::auth::JwtMiddleware;

use super::handlers::{account_handler, customer_handler, pix_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customers")
            .wrap(JwtMiddleware)
            .route("", web::get().to(customer_handler::get_customers))
            .route("", web::post().to(customer_handler::create_customer))
            .route("/{id}", web::get().to(customer_handler::get_customer_by_id)),
    )
    .service(
        web::scope("/accounts")
            .route("", web::get().to(account_handler::list_accounts))
            .route(
                "/customer/{id}",
                web::get().to(account_handler::get_account_by_customer_id),
            ),
    )
    .service(
        web::scope("/pix-keys")
            .wrap(JwtMiddleware)
            .route("", web::post().to(pix_handler::create_pix_key))
            .route("", web::get().to(pix_handler::list_pix_keys)),
    );
}
