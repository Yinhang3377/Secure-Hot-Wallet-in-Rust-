//! 集成测试：验证加密/解密循环和内存清理

use hot_wallet::security::encryption::WalletSecurity;
use hot_wallet::tools::error::WalletError;

#[test]
fn test_encrypt_decrypt_cycle() {
    let private_key = b"32_byte_private_key_1234567890ab";
    let encryption_key = "32_byte_encryption_key_1234567890";
    let encrypted = WalletSecurity::encrypt_private_key(private_key, encryption_key).unwrap();
    let decrypted = WalletSecurity::decrypt_private_key(&encrypted, encryption_key).unwrap();
    assert_eq!(private_key.to_vec(), decrypted);
}

#[test]
fn test_invalid_key_length() {
    let private_key = b"test_key";
    let short_key = "too_short";
    let result = WalletSecurity::encrypt_private_key(private_key, short_key);
    assert!(matches!(result, Err(WalletError::EncryptionError(_))));
}

#[test]
fn test_invalid_ciphertext() {
    let encryption_key = "32_byte_encryption_key_1234567890";
    let invalid_ciphertext = b"too_short";
    let result = WalletSecurity::decrypt_private_key(invalid_ciphertext, encryption_key);
    assert!(matches!(result, Err(WalletError::EncryptionError(_))));
}
