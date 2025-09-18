/// 加密模块：负责钱包核心数据的加解密逻辑
/// 支持对称加密（如 AES-GCM）、非对称加密（如 secp256k1）等

use aes_gcm::{ Aes256Gcm, Key, Nonce };
use aes_gcm::aead::{ Aead };
use aes_gcm::KeyInit;
use zeroize::Zeroize;
use crate::tools::error::WalletError;

/// WalletSecurity handles cryptographic operations for the hot wallet.
/// This module provides secure encryption and decryption of sensitive data, such as private keys.
pub struct WalletSecurity;

impl WalletSecurity {
    /// 使用 AES-256-GCM 加密私钥
    ///
    /// # 参数
    /// * `private_key` - 待加密的私钥（如 secp256k1，32字节）
    /// * `encryption_key` - 加密用的密钥（必须为32字节）
    ///
    /// # 返回值
    /// 返回包含随机nonce的密文（nonce前置），或 WalletError 错误
    ///
    /// # 安全说明
    /// - 使用随机12字节nonce防止重放攻击
    /// - nonce前置于密文，便于解密时提取
    /// - 加密密钥用完自动清零，防止内存泄漏
    /// - 生产环境建议用KDF安全派生encryption_key
    pub fn encrypt_private_key(
        private_key: &[u8],
        encryption_key: &str
    ) -> Result<Vec<u8>, WalletError> {
        // 检查加密密钥长度（AES-256要求32字节）
        if encryption_key.len() < 32 {
            return Err(WalletError::EncryptionError("加密密钥长度必须至少32字节".to_string()));
        }

        // 用密钥初始化加密器
        let mut key = Key::<Aes256Gcm>::from_slice(encryption_key.as_bytes()).clone();
        let cipher = Aes256Gcm::new(&key);

        // 生成随机12字节nonce
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 加密私钥
        let ciphertext = cipher
            .encrypt(nonce, private_key)
            .map_err(|e| WalletError::EncryptionError(format!("加密失败: {}", e)))?;

        // nonce前置于密文，便于解密
        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        // 用完自动清零密钥
        key.zeroize();

        Ok(result)
    }

    /// 解密用 AES-256-GCM 加密的私钥
    ///
    /// # 参数
    /// * `ciphertext` - 加密数据（前12字节为nonce）
    /// * `encryption_key` - 解密用密钥（需与加密密钥一致）
    ///
    /// # 返回值
    /// 返回解密后的私钥明文，或 WalletError 错误
    ///
    /// # 安全说明
    /// - 默认前12字节为nonce
    /// - 密钥用完自动清零
    /// - 生产环境需校验密钥派生与nonce管理
    #[allow(dead_code)]
    pub fn decrypt_private_key(
        ciphertext: &[u8],
        encryption_key: &str
    ) -> Result<Vec<u8>, WalletError> {
        // 检查密文长度（至少12字节nonce+密文）
        if ciphertext.len() < 12 {
            return Err(WalletError::EncryptionError("密文无效：长度过短".to_string()));
        }

        // 提取nonce和实际密文
        let nonce = Nonce::from_slice(&ciphertext[..12]);
        let actual_ciphertext = &ciphertext[12..];

        // 用密钥初始化解密器
        let mut key = Key::<Aes256Gcm>::from_slice(encryption_key.as_bytes()).clone();
        let cipher = Aes256Gcm::new(&key);

        // 解密数据
        let plaintext = cipher
            .decrypt(nonce, actual_ciphertext)
            .map_err(|e| WalletError::EncryptionError(format!("解密失败: {}", e)))?;

        // 用完自动清零密钥
        key.zeroize();

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_cycle() {
        // 测试加密解密流程
        let private_key = b"32_byte_private_key_1234567890ab";
        let encryption_key = "32_byte_encryption_key_1234567890";

        let encrypted = WalletSecurity::encrypt_private_key(private_key, encryption_key).unwrap();
        let decrypted = WalletSecurity::decrypt_private_key(&encrypted, encryption_key).unwrap();

        assert_eq!(private_key.to_vec(), decrypted);
    }

    #[test]
    fn test_invalid_key_length() {
        // 测试密钥长度不足的情况
        let private_key = b"test_key";
        let short_key = "too_short";
        let result = WalletSecurity::encrypt_private_key(private_key, short_key);
        assert!(matches!(result, Err(WalletError::EncryptionError(_))));
    }

    #[test]
    fn test_invalid_ciphertext() {
        // 测试密文长度不足的情况
        let encryption_key = "32_byte_encryption_key_1234567890";
        let invalid_ciphertext = b"too_short";
        let result = WalletSecurity::decrypt_private_key(invalid_ciphertext, encryption_key);
        assert!(matches!(result, Err(WalletError::EncryptionError(_))));
    }
}
