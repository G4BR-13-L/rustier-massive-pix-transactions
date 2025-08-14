use actix_web::{HttpResponse};
pub async fn list_accounts() -> HttpResponse {
    HttpResponse::Ok().json("List of accounts")
}
