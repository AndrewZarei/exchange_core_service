use std::str::FromStr;
use bip39::{Mnemonic, Language};
use bitcoin::{
    Network,
    Address,
    bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
    secp256k1::Secp256k1,
};
use crate::error::WalletError;

pub struct BitcoinHDWallet {
    mnemonic: Mnemonic,
    xpriv: ExtendedPrivKey,
}

impl BitcoinHDWallet {
    /// Generates a new random HD wallet
    pub fn new(network: Network) -> Result<Self, WalletError> {
        let mnemonic = Mnemonic::generate_in(Language::English, 12)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;
        let seed = mnemonic.to_seed("");

        let xpriv = ExtendedPrivKey::new_master(network, &seed)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;

        Ok(Self { mnemonic, xpriv })
    }

    /// Gets the mnemonic phrase
    pub fn phrase(&self) -> String {
        self.mnemonic.to_string()
    }

    /// Gets the seed bytes
    pub fn seed(&self) -> [u8; 64] {
        self.mnemonic.to_seed("")
    }

    /// Derives a child key
    pub fn derive_key(&self, path: &str) -> Result<ExtendedPrivKey, WalletError> {
        let derivation_path = DerivationPath::from_str(path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;
        let secp = Secp256k1::new();
        self.xpriv.derive_priv(&secp, &derivation_path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))
    }

    /// Generates a proper BIP-84 native SegWit receive address
    pub fn generate_receive_address(&self, index: u32) -> Result<String, WalletError> {
        let secp = Secp256k1::new();

        // BIP-84 derivation path: m/84'/0'/0'/0/{index}
        let path = format!("m/84h/0h/0h/0/{}", index);
        let derivation_path = DerivationPath::from_str(&path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;

        // Derive the child private key
        let child_xpriv = self.xpriv.derive_priv(&secp, &derivation_path)
            .map_err(|e| WalletError::BitcoinCreationError(e.to_string()))?;

        // Get the extended public key
        let child_xpub = ExtendedPubKey::from_priv(&secp, &child_xpriv);

        // Generate native SegWit v0 address (P2WPKH)
        let address = Address::p2wpkh(&child_xpub.to_pub(), Network::Bitcoin);
        Ok(address.to_string())
    }
}

/// Creates a new Bitcoin wallet and returns the first receive address
pub fn create_wallet(user_id: &str) -> Result<String, WalletError> {
    let wallet = BitcoinHDWallet::new(Network::Bitcoin)?;
    let address = wallet.generate_receive_address(0)?;

    // In production:
    // 1. Store the mnemonic/xpriv securely (encrypted)
    // 2. Associate with user_id
    // 3. Return only the public address

    Ok(address)
}