//! 配置模块：负责加载和管理环境变量（如 ENCRYPTION_KEY、NETWORK）
use std::env;

/// 钱包配置结构体
#[derive(Debug, Clone)]
pub struct WalletConfig {
    /// 加密密钥（建议32字节）
    pub encryption_key: String,
    /// 网络类型（如 mainnet/testnet）
    pub network: String,
}

impl WalletConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let encryption_key = env
            ::var("ENCRYPTION_KEY")
            .unwrap_or_else(|_| "default_key_please_set_env".to_string());
        let network = env::var("NETWORK").unwrap_or_else(|_| "mainnet".to_string());
        WalletConfig { encryption_key, network }
    }
}
