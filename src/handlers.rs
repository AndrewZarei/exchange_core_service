use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use crate::{
    models::{CreateWalletRequest, WalletResponse},
    error::WalletError,
};

pub async fn create_wallet(
    req: web::Json<CreateWalletRequest>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, WalletError> {
    match req.blockchain.to_lowercase().as_str() {
        "bitcoin" => {
            let address = crate::services::bitcoin::create_wallet(&pool, &req.user_id).await?;
            Ok(HttpResponse::Ok().json(WalletResponse {
                address,
                blockchain: "bitcoin".to_string(),
                user_id: req.user_id.clone(),
            }))
        }
        "bsc" => {
            let wallet = crate::services::bsc::BscWallet::new()?;
            let address = wallet.address1(); // or `.address()`, if renamed
            Ok(HttpResponse::Ok().json(WalletResponse {
                address,
                blockchain: "bsc".to_string(),
                user_id: req.user_id.clone(),
            }))
        }
        "solana" => {
            let address = crate::services::solana::create_solana_wallet(&pool, &req.user_id).await?;
            Ok(HttpResponse::Ok().json(WalletResponse {
                address,
                blockchain: "solana".to_string(),
                user_id: req.user_id.clone(),
            }))
        }
        _ => Err(WalletError::UnsupportedBlockchain(req.blockchain.clone())),
    }
}