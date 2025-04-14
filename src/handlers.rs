use crate::error::WalletError;
use crate::models::{CreateWalletRequest, CreateWalletResponse};
use crate::services;
use actix_web::{web, HttpResponse};

pub async fn create_wallet(
    req: web::Json<CreateWalletRequest>,
) -> Result<HttpResponse, WalletError> {
    match req.blockchain_name.to_lowercase().as_str() {
        "bitcoin" => {
            let address = services::bitcoin::create_wallet(&req.user_id)?;
            Ok(HttpResponse::Ok().json(CreateWalletResponse {
                blockchain_name: "bitcoin".to_string(),
                wallet_address: address,
                user_id: req.user_id.clone(),
            }))
        }
        // Add other blockchains here as you implement them
        _ => Err(WalletError::UnsupportedBlockchain(req.blockchain_name.clone())),
    }
}