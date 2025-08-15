use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, post, web};
use chrono::{Duration, Utc};
use env_logger::Env;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
struct LoginRequest {
    customer_id: Uuid,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

#[post("/login")]
async fn login(pool: web::Data<PgPool>, req: web::Json<LoginRequest>) -> impl Responder {
    let exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT * FROM customers WHERE id = $1::uuid)")
            .bind(req.customer_id.to_string())
            .fetch_one(pool.get_ref())
            .await;

    match exists {
        Ok(true) => {
            let secret =
                env::var("JWT_SECRET").unwrap_or_else(|_| "minha_chave_super_secreta".into());
            let header = Header::new(jsonwebtoken::Algorithm::HS256);
            let exp = (Utc::now() + Duration::hours(1)).timestamp() as usize;
            let claims = Claims {
                sub: req.customer_id.to_string(),
                exp,
            };

            let token = encode(
                &header,
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .unwrap();

            HttpResponse::Ok().json(TokenResponse { token })
        }
        Ok(false) => HttpResponse::Unauthorized().body("UUID não encontrado"),
        Err(e) => {
            eprintln!("Erro ao consultar banco: {:?}", e);
            HttpResponse::InternalServerError().body("Erro interno no servidor")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não configurada");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Erro ao conectar no banco");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(login)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
