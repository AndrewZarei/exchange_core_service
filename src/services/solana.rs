use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use bip32::Seed;
use bip39::{Language, Mnemonic};
use bitcoin::{
    bip32::{ExtendedPrivKey, ExtendedPubKey},
    Network,
};
use bitcoin::secp256k1::Secp256k1;

//-------------------------- get address -------------------------------------

pub fn get_solana_address() -> Result<String, String> {
    let output = Command::new("solana")
        .arg("address")
        .output()
        .map_err(|e| format!("Failed to run solana command: {}", e))?;

    if output.status.success() {
        let address = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(address)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command failed: {}", error))
    }
}

//-------------------------- get new address -------------------------------------
fn generate_and_store_keys() -> Result<String, Box<dyn std::error::Error>> {
    // Generate a new 12-word mnemonic phrase
    let mnemonic = Mnemonic::generate_in(Language::English, 12)?;
    let phrase = mnemonic.language();

    // Derive the seed from the mnemonic
    let seed = mnemonic.to_seed("");

    // Initialize Secp256k1 context
    let secp = Secp256k1::new();

    // Derive the extended private key (xprv) from the seed
    let xprv = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)?;

    // Derive the extended public key (xpub) from the xprv
    let xpub = ExtendedPubKey::from_priv(&secp, &xprv);

    // Define the directory to store the keys
    let base_path = Path::new("/home/hosein/.config/solana/keys");
    fs::create_dir_all(base_path)?;

    // Save the mnemonic phrase
    let mut mnemonic_file = File::create(base_path.join("mnemonic.txt"))?;
    mnemonic_file.write_all(phrase.to_string().as_ref())?;

    // Save the extended private key
    let mut xprv_file = File::create(base_path.join("xprv.txt"))?;
    xprv_file.write_all(xprv.to_string().as_bytes())?;

    // Save the extended public key
    let mut xpub_file = File::create(base_path.join("xpub.txt"))?;
    xpub_file.write_all(xpub.to_string().as_bytes())?;

    // Return the extended public key as the address (or some other string-based address)
    Ok(xpub.to_string())  // Returning xpub as the Solana address (for example)
}

// ------------------------ ***func for solana -----------

use crate::{error::WalletError, models::StoredWallet};
use sqlx::MySqlPool;
use crate::database::{get_wallet, save_wallet};

pub async fn create_solana_wallet(
    pool: &MySqlPool,
    user_id: &str,
) -> Result<String, WalletError> {
    if let Some(wallet) = get_wallet(pool, user_id, "solana").await? {
        return Ok(wallet.address);
    }

    let address = generate_and_store_keys().unwrap();

    let stored_wallet = StoredWallet {
        user_id: user_id.to_string(),
        blockchain: "solana".to_string(),
        address: address.clone(),
        mnemonic: "".to_string(),
        xpriv: "".to_string(),
        derivation_path: "".to_string(),
    };

    save_wallet(pool, &stored_wallet).await?;

    Ok(address)
}
