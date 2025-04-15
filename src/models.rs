use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub blockchain: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WalletResponse {
    pub address: String,
    pub blockchain: String,
    pub user_id: String,
}

#[derive(Debug, FromRow)]
pub struct StoredWallet {
    pub user_id: String,
    pub blockchain: String,
    pub address: String,
    pub mnemonic: String,
    pub xpriv: String,
    pub derivation_path: String,
}