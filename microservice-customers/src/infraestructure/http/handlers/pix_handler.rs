use actix_web::{HttpResponse};
pub async fn list_pix_keys() -> HttpResponse {
    HttpResponse::Ok().json("List of PIX keys")
}
