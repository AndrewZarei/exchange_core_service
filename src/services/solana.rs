// // src/services/solana.rs
// use bip39::{Language, Mnemonic, MnemonicType, Seed};
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     pubkey::Pubkey,
//     signature::{keypair_from_seed, Keypair},
//     signer::Signer,
// };
// use crate::{error::WalletError, models::StoredSolanaWallet};
// use sqlx::MySqlPool;
//
// const SOLANA_DERIVATION_PATH: &str = "m/44'/501'/0'/0'"; // Standard Solana derivation path
//
// pub struct SolanaWallet {
//     mnemonic: Mnemonic,
//     keypair: Keypair,
// }
//
// impl SolanaWallet {
//     /// Generates a new Solana HD wallet
//     pub fn new() -> Result<Self, WalletError> {
//         let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English)
//             .map_err(|e| WalletError::SolanaCreationError(e.to_string()))?;
//
//         let seed = Seed::new(&mnemonic, "");
//         let keypair = keypair_from_seed(seed.as_bytes())
//             .map_err(|e| WalletError::SolanaCreationError(e.to_string()))?;
//
//         Ok(Self { mnemonic, keypair })
//     }
//
//     /// Gets the mnemonic phrase
//     pub fn phrase(&self) -> String {
//         self.mnemonic.to_string()
//     }
//
//     /// Gets the public address
//     pub fn address(&self) -> String {
//         self.keypair.pubkey().to_string()
//     }
// }
//
// /// Creates a new Solana wallet and stores it in the database
// pub async fn create_wallet(
//     pool: &MySqlPool,
//     user_id: &str,
// ) -> Result<String, WalletError> {
//     let wallet = SolanaWallet::new()?;
//
//     // Store wallet securely
//     let stored_wallet = StoredSolanaWallet {
//         user_id: user_id.to_string(),
//         blockchain: "solana".to_string(),
//         address: wallet.address(),
//         mnemonic: wallet.phrase(),
//         derivation_path: SOLANA_DERIVATION_PATH.to_string(),
//     };
//
//     store_wallet(pool, stored_wallet).await?;
//
//     Ok(wallet.address())
// }
//
// async fn store_wallet(pool: &MySqlPool, wallet: StoredSolanaWallet) -> Result<(), WalletError> {
//     sqlx::query!(
//         r#"INSERT INTO solana_wallets
//            (user_id, blockchain, address, mnemonic, derivation_path)
//            VALUES (?, ?, ?, ?, ?)"#,
//         wallet.user_id,
//         wallet.blockchain,
//         wallet.address,
//         wallet.mnemonic,
//         wallet.derivation_path
//     )
//         .execute(pool)
//         .await
//         .map_err(|e| WalletError::DatabaseError(e.to_string()))?;
//
//     Ok(())
// }
//
// /// RPC Client implementation
// pub struct SolanaClient {
//     client: RpcClient,
// }
//
// impl SolanaClient {
//     pub fn new(rpc_url: &str) -> Self {
//         Self {
//             client: RpcClient::new(rpc_url.to_string()),
//         }
//     }
//
//     pub async fn get_balance(&self, address: &str) -> Result<f64, WalletError> {
//         let pubkey = Pubkey::from_str(address)
//             .map_err(|e| WalletError::SolanaRpcError(e.to_string()))?;
//
//         let balance = self.client.get_balance(&pubkey)
//             .map_err(|e| WalletError::SolanaRpcError(e.to_string()))?;
//
//         Ok(balance as f64 / 1_000_000_000.0) // Convert lamports to SOL
//     }
//
//     pub async fn airdrop(
//         &self,
//         address: &str,
//         sol: f64,
//     ) -> Result<String, WalletError> {
//         let pubkey = Pubkey::from_str(address)
//             .map_err(|e| WalletError::SolanaRpcError(e.to_string()))?;
//
//         let lamports = (sol * 1_000_000_000.0) as u64;
//         let signature = self.client.request_airdrop(&pubkey, lamports)
//             .map_err(|e| WalletError::SolanaRpcError(e.to_string()))?;
//
//         Ok(signature.to_string())
//     }
// }