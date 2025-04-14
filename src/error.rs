use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};


#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Unsupported blockchain: {0}")]
    UnsupportedBlockchain(String),

    #[error("Bitcoin wallet creation failed: {0}")]
    BitcoinCreationError(String),

    #[error("Internal server error")]
    InternalError,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl ResponseError for WalletError {
    fn error_response(&self) -> HttpResponse {
        match self {
            WalletError::UnsupportedBlockchain(chain) => HttpResponse::BadRequest()
                .body(format!("Unsupported blockchain: {}", chain)),

            WalletError::BitcoinCreationError(msg) => HttpResponse::InternalServerError()
                .body(format!("Failed to create Bitcoin wallet: {}", msg)),

            WalletError::InternalError => HttpResponse::InternalServerError()
                .body("Internal server error"),

            WalletError::DatabaseError(e) => HttpResponse::InternalServerError()
                .body(format!("Database error: {}", e)),
        }
    }
}