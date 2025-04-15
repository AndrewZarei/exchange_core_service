use std::str::FromStr;
use bip39::{Mnemonic, Language};
use bitcoin::{
    Network,
    Address,
    bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
    secp256k1::Secp256k1,
};
use crate::{error::WalletError, models::StoredWallet};
use sqlx::MySqlPool;
use crate::database::{get_wallet, save_wallet};

pub struct BitcoinHDWallet {
    mnemonic: Mnemonic,
    xpriv: ExtendedPrivKey,
}

impl BitcoinHDWallet {
    pub fn new(network: Network) -> Result<Self, WalletError> {
        let mnemonic = Mnemonic::generate_in(Language::English, 12)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;
        let seed = mnemonic.to_seed("");

        let xpriv = ExtendedPrivKey::new_master(network, &seed)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;

        Ok(Self { mnemonic, xpriv })
    }

    pub fn phrase(&self) -> String {
        self.mnemonic.to_string()
    }

    pub fn generate_receive_address(&self, index: u32) -> Result<String, WalletError> {
        let secp = Secp256k1::new();
        let path = format!("m/84h/0h/0h/0/{}", index);
        let derivation_path = DerivationPath::from_str(&path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;

        let child_xpriv = self.xpriv.derive_priv(&secp, &derivation_path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;

        let child_xpub = ExtendedPubKey::from_priv(&secp, &child_xpriv);
        let address = Address::p2wpkh(&child_xpub.to_pub(), Network::Bitcoin);
        Ok(address.to_string())
    }
}

pub async fn create_wallet(
    pool: &MySqlPool,
    user_id: &str,
) -> Result<String, WalletError> {
    if let Some(wallet) = get_wallet(pool, user_id, "bitcoin").await? {
        return Ok(wallet.address);
    }

    let wallet = BitcoinHDWallet::new(Network::Bitcoin)?;
    let address = wallet.generate_receive_address(0)?;
    let path = "m/84h/0h/0h/0/0";

    let stored_wallet = StoredWallet {
        user_id: user_id.to_string(),
        blockchain: "bitcoin".to_string(),
        address: address.clone(),
        mnemonic: wallet.phrase(),
        xpriv: wallet.xpriv.to_string(),
        derivation_path: path.to_string(),
    };

    save_wallet(pool, &stored_wallet).await?;

    Ok(address)
}