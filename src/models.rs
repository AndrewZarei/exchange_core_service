use serde::{Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateWalletRequest {
    pub blockchain_name: String,
    pub user_id: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateWalletResponse {
    pub blockchain_name: String,
    pub wallet_address: String,
    pub user_id: String,
}