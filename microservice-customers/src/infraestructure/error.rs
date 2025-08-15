use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, Error, From};
use serde::Serialize;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::{Error as PGError, SqlState};

#[derive(Debug, Display, Error, Serialize)]
pub struct ApiError {
    pub msg: String,
}

#[derive(Debug, Display, Error, Serialize)]
pub struct ConflictError {
    pub msg: String,
}

#[derive(Debug, Display, Error, Serialize)]
pub struct InternalError {
    pub msg: String,
}

#[derive(Debug, Display, Error, From)]
pub enum MyError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
    Conflict(ConflictError),
    Internal(InternalError),
    ApiError(ApiError),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: String, message: String) -> Self {
        Self {
            error,
            message,
            details: None,
        }
    }

    pub fn with_details(self, error: String, message: String, details: String) -> Self {
        Self {
            error,
            message,
            details: Some(details),
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::NotFound => HttpResponse::NotFound().json(ErrorResponse::new(
                "not_found".to_string(),
                "The requested resource was not found".to_string(),
            )),
            MyError::PoolError(ref err) => HttpResponse::InternalServerError().json(
                ErrorResponse::new(
                    "database_error".to_string(),
                    "Database connection error".to_string(),
                )
                .with_details("pool_error".to_string(), err.to_string(), "".to_string()),
            ),
            MyError::Conflict(conflict_error) => HttpResponse::Conflict().json(
                ErrorResponse::new("conflict".to_string(), conflict_error.msg.clone()),
            ),
            MyError::Internal(internal_error) => HttpResponse::InternalServerError().json(
                ErrorResponse::new("internal_error".to_string(), internal_error.msg.clone()),
            ),
            MyError::ApiError(api_error) => HttpResponse::BadRequest().json(
                ErrorResponse::new("api_error".to_string(), api_error.msg.clone()),
            ),
            MyError::PGError(pg_error) => {
                // Map PostgreSQL errors to appropriate responses
                if let Some(db_err) = pg_error.as_db_error() {
                    match db_err.code() {
                        &SqlState::UNIQUE_VIOLATION => HttpResponse::Conflict().json(
                            ErrorResponse::new(
                                "unique_violation".to_string(),
                                "Unique constraint violation".to_string(),
                            )
                            .with_details(
                                "database_error".to_string(),
                                db_err.message().to_string(),
                                db_err.detail().unwrap_or("").to_string(),
                            ),
                        ),
                        &SqlState::FOREIGN_KEY_VIOLATION => HttpResponse::BadRequest().json(
                            ErrorResponse::new(
                                "foreign_key_violation".to_string(),
                                "Foreign key violation".to_string(),
                            )
                            .with_details(
                                "database_error".to_string(),
                                db_err.message().to_string(),
                                db_err.detail().unwrap_or("").to_string(),
                            ),
                        ),
                        _ => HttpResponse::InternalServerError().json(
                            ErrorResponse::new(
                                "database_error".to_string(),
                                "Database error".to_string(),
                            )
                            .with_details(
                                "database_error".to_string(),
                                db_err.message().to_string(),
                                db_err.detail().unwrap_or("").to_string(),
                            ),
                        ),
                    }
                } else {
                    HttpResponse::InternalServerError().json(ErrorResponse::new(
                        "database_error".to_string(),
                        "Database communication error".to_string(),
                    ))
                }
            }
            MyError::PGMError(pgm_error) => HttpResponse::InternalServerError().json(
                ErrorResponse::new(
                    "mapping_error".to_string(),
                    "Data mapping error".to_string(),
                )
                .with_details("pg_mapper_error".to_string(), pgm_error.to_string(), "".to_string()),
            ),
        }
    }
}

pub fn map_db_error(e: tokio_postgres::Error) -> MyError {
    if let Some(db_err) = e.as_db_error() {
        match db_err.code() {
            &SqlState::UNIQUE_VIOLATION => MyError::Conflict(ConflictError {
                msg: format!("Unique constraint violation: {}", db_err.message()),
            }),
            &SqlState::FOREIGN_KEY_VIOLATION => MyError::Conflict(ConflictError {
                msg: format!("Foreign key violation: {}", db_err.message()),
            }),
            _ => MyError::Internal(InternalError {
                msg: format!("Database error: {}", db_err.message()),
            }),
        }
    } else {
        MyError::ApiError(ApiError {
            msg: e.to_string(),
        })
    }
}