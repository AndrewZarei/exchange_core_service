use actix_web::web::Data;
use k256::ecdsa::SigningKey;
use k256::EncodedPoint;
use sha3::{Digest, Keccak256};
use hex;
use rand::rngs::OsRng;
use crate::{error::WalletError, models::StoredWallet};
use sqlx::MySqlPool;
use crate::database::{get_wallet, save_wallet};

pub struct BscWallet {
    private_key: SigningKey,
    address: String,
}

impl BscWallet {
    pub fn new() -> Result<Self, WalletError> {
        let mut rng = OsRng;
        let private_key = SigningKey::random(&mut rng);

        let public_key = private_key.verifying_key();
        let encoded = public_key.to_encoded_point(false); // 👈 Fix is here
        let uncompressed = encoded.as_bytes();

        let hash = Keccak256::digest(uncompressed);
        let address_bytes = &hash[12..]; // last 20 bytes
        let address = format!("0x{}", hex::encode(address_bytes));

        Ok(Self {
            private_key,
            address,
        })
    }


    pub(crate) fn address1(&self) -> String {

        self.address.clone()
    }

    pub fn private_key_hex(&self) -> String {
        format!("0x{}", hex::encode(self.private_key.to_bytes()))
    }

    pub(crate) async fn address(&self) -> String {
        self.address.clone()
    }
}

pub(crate) async fn BscWallet(_p0: String) {
    BscWallet::new().expect("Error while creating wallet");
}