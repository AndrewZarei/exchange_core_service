use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySqlPool, migrate::Migrator};
use std::env;
use std::path::Path;
use crate::{error::WalletError, models::StoredWallet};

static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

pub async fn create_pool() -> Result<MySqlPool, WalletError> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| WalletError::ConfigError("DATABASE_URL not set".into()))?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| WalletError::DatabaseError(e.to_string()))?;

    // Run migrations
    MIGRATOR.run(&pool).await
        .map_err(|e| WalletError::DatabaseError(e.to_string()))?;

    Ok(pool)
}

pub async fn ensure_user_exists(
    pool: &MySqlPool,
    user_id: &str
) -> Result<(), WalletError> {
    sqlx::query(
        r#"INSERT IGNORE INTO users (user_id) VALUES (?)"#
    )
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn save_wallet(
    pool: &MySqlPool,
    wallet: &StoredWallet
) -> Result<(), WalletError> {
    ensure_user_exists(pool, &wallet.user_id).await?;

    sqlx::query(
        r#"INSERT INTO wallets
           (user_id, blockchain, address, mnemonic, xpriv, derivation_path)
           VALUES (?, ?, ?, ?, ?, ?)"#
    )
        .bind(&wallet.user_id)
        .bind(&wallet.blockchain)
        .bind(&wallet.address)
        .bind(&wallet.mnemonic)
        .bind(&wallet.xpriv)
        .bind(&wallet.derivation_path)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_wallet(
    pool: &MySqlPool,
    user_id: &str,
    blockchain: &str
) -> Result<Option<StoredWallet>, WalletError> {
    sqlx::query_as(
        r#"SELECT user_id, blockchain, address, mnemonic, xpriv, derivation_path
           FROM wallets
           WHERE user_id = ? AND blockchain = ?"#
    )
        .bind(user_id)
        .bind(blockchain)
        .fetch_optional(pool)
        .await
        .map_err(Into::into)
}