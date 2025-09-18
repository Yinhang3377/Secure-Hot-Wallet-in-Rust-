mod config;
mod security;
mod tools;
/// 主入口：集成配置、安全、错误等模块，实现 wallet create 命令生成加密账户

// 直接 use 子模块，无需 mod 声明

use rand::RngCore;
use hex;

use crate::config::config::WalletConfig; // 钱包配置加载
use crate::security::encryption::WalletSecurity; // 加密/解密操作
use secp256k1::SecretKey;
use secp256k1::PublicKey;
use std::env;
use secp256k1::Secp256k1;
/// 程序主入口
fn main() {
    // 解析命令行参数，要求 wallet create
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] != "create" {
        eprintln!("用法: wallet create");
        return;
    }

    // 加载环境变量配置（如 ENCRYPTION_KEY、NETWORK）
    let config = WalletConfig::from_env();
    println!("[配置] network: {}", config.network);

    // 生成 secp256k1 密钥对（私钥+公钥）
    let secp = Secp256k1::new();
    let mut rng = rand::rng();
    let mut sk_bytes = [0u8; 32];
    rng.fill_bytes(&mut sk_bytes);
    let secret_key = SecretKey::from_byte_array(sk_bytes).expect("生成私钥失败");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    println!("[生成] 公钥: {}", public_key);

    // 用配置中的加密密钥加密私钥
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
    // 实际项目可将加密私钥和公钥存储到文件或数据库，便于后续恢复和使用
}
