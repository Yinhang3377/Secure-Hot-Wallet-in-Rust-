use thiserror::Error;

/// 钱包全局错误类型定义模块
/// 用于集中管理所有与钱包相关的错误，便于统一处理和扩展

/// 钱包通用错误类型
#[derive(Debug, Error)]
pub enum WalletError {
    /// 加密相关错误，携带详细信息
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    /// 解密相关错误，携带详细信息
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    /// 其他未知错误
    #[error("Other error: {0}")]
    Other(String),
}

/// 实现 Display trait，使 WalletError 支持中文错误信息输出
/// 便于日志、终端等场景下直观展示错误内容
impl std::fmt::Display for WalletError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletError::EncryptionError(msg) => write!(f, "加密失败: {}", msg),
            WalletError::DecryptionError(msg) => write!(f, "解密失败: {}", msg),
            WalletError::Other(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}
// ...existing code from error.rs...
