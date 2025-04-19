use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as SqlxError;
use sqlx::migrate::MigrateError;
use serde_json::json;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Unsupported blockchain: {0}")]
    UnsupportedBlockchain(String),

    #[error("Bitcoin wallet creation failed: {0}")]
    BitcoinCreationError(String),

    #[error("Solana wallet creation failed: {0}")]
    BnbWalletCreationError(String),

    #[error("Ethereum wallet creation failed: {0}")]
    EthereumWalletCreationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error")]
    InternalError,

    #[error("Migration error: {0}")]
    MigrationError(String),

    // #[error("Solana wallet creation failed: {0}")]
    // SolanaCreationError(String),
    //
    // #[error("Solana RPC error: {0}")]
    // SolanaRpcError(String),
}


impl WalletError {
    pub(crate) fn EthereumCreationError(p0: String) -> Self {
        Self::EthereumCreationError(p0.clone())
    }
}

impl ResponseError for WalletError {
    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        let status_code = match self {
            WalletError::UnsupportedBlockchain(_) => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code)
            .json(json!({
                "error": error_message,
                "code": status_code.as_u16()
            }))
    }
}

impl From<SqlxError> for WalletError {
    fn from(err: SqlxError) -> Self {
        WalletError::DatabaseError(err.to_string())
    }
}

impl From<MigrateError> for WalletError {
    fn from(err: MigrateError) -> Self {
        WalletError::MigrationError(err.to_string())
    }
}


// impl From<solana_sdk::signature::ParseSignatureError> for WalletError {
//     fn from(err: solana_sdk::signature::ParseSignatureError) -> Self {
//         WalletError::SolanaRpcError(err.to_string())
//     }
