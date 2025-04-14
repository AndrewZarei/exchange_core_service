use std::str::FromStr;
use bip39::{Mnemonic, Language};
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    Network,
    secp256k1::Secp256k1,
};
use crate::error::WalletError;

pub struct BitcoinHDWallet {
    mnemonic: Mnemonic,
    pub(crate) xpriv: Xpriv,
}

impl BitcoinHDWallet {

    pub fn new(network: Network) -> Result<Self, WalletError> {
        let mnemonic = Mnemonic::generate_in(Language::English, 12)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;
        let seed = mnemonic.to_seed("");

        let xpriv = Xpriv::new_master(network, &seed)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;

        Ok(Self { mnemonic, xpriv })
    }

    pub fn phrase(&self) -> String {
        self.mnemonic.to_string()
    }

    pub fn seed(&self) -> [u8; 64] {
        self.mnemonic.to_seed("")
    }

    pub fn derive_key(&self, path: &str) -> Result<Xpriv, WalletError> {
        let derivation_path = DerivationPath::from_str(path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;
        let secp = Secp256k1::new();
        self.xpriv.derive_priv(&secp, &derivation_path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))
    }

    pub fn generate_receive_address(&self, index: u32) -> Result<String, WalletError> {
        let path = format!("m/84h/0h/0h/0/{}", index);
        let derived_key = self.derive_key(&path)?;

        // Convert to address (this is simplified - you'll need to implement proper address generation)
        let address = format!("bc1q{}...", &derived_key.to_string()[..10]);
        Ok(address)
    }
}

pub fn create_wallet(user_id: &str) -> Result<String, WalletError> {
    let wallet = BitcoinHDWallet::new(Network::Bitcoin)?;

    // Generate first receive address
    let address = wallet.generate_receive_address(0)?;

    // In a real implementation, you would:
    // 1. Store the mnemonic/keys securely (encrypted) in your database
    // 2. Associate them with the user_id
    // 3. Return only the public address

    Ok(address)
}