//! 主入口：集成配置、安全、错误等模块，实现 wallet create 命令生成加密账户

mod config;
mod security;
mod tools;

use config::config::WalletConfig;
use security::encryption::WalletSecurity;
use secp256k1::Secp256k1;
use secp256k1::SecretKey;
use secp256k1::PublicKey;
use std::env;
use tools::error::WalletError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] != "create" {
        eprintln!("用法: wallet create");
        return;
    }

    // 加载配置
    let config = WalletConfig::from_env();
    println!("[配置] network: {}", config.network);

    // 生成 secp256k1 密钥对
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
    println!("[生成] 公钥: {}", public_key);

    // 加密私钥
    let encrypted = match
        WalletSecurity::encrypt_private_key(&secret_key.secret_bytes(), &config.encryption_key)
    {
        Ok(data) => data,
        Err(e) => {
            eprintln!("[错误] 加密失败: {}", e);
            return;
        }
    };
    println!("[加密] 加密私钥(hex): {}", hex::encode(&encrypted));
    // 实际项目可将加密私钥和公钥存储到文件或数据库
}
