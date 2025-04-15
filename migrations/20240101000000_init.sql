-- Create users table
CREATE TABLE IF NOT EXISTS users (
                                     id INT AUTO_INCREMENT PRIMARY KEY,
                                     user_id VARCHAR(255) NOT NULL UNIQUE,
                                     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create wallets table
CREATE TABLE IF NOT EXISTS wallets (
                                       id INT AUTO_INCREMENT PRIMARY KEY,
                                       user_id VARCHAR(255) NOT NULL,
                                       blockchain VARCHAR(50) NOT NULL,
                                       address VARCHAR(255) NOT NULL UNIQUE,
                                       mnemonic TEXT NOT NULL,
                                       xpriv TEXT NOT NULL,
                                       derivation_path VARCHAR(255) NOT NULL,
                                       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                       FOREIGN KEY (user_id) REFERENCES users(user_id),
                                       INDEX (user_id, blockchain)
);