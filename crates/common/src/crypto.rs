use crate::error::{KonquerorError, Result};
use aes_gcm::KeyInit as AesKeyInit;
use aes_gcm::{
    AeadCore, Aes256Gcm, Key,
    aead::{Aead, Nonce, OsRng},
};
#[cfg(feature = "password-hashing")]
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

fn validate_key(key: &[u8]) -> Result<()> {
    if key.len() != 32 {
        return Err(KonquerorError::Crypto("Key should be 32 bytes".to_string()));
    }

    Ok(())
}

#[cfg(feature = "password-hashing")]
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| KonquerorError::Crypto(e.to_string()))?;

    Ok(hash.to_string())
}

#[cfg(feature = "password-hashing")]
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed = argon2::password_hash::PasswordHash::new(hash)
        .map_err(|e| KonquerorError::Crypto(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub fn verify_hmac(data: &[u8], key: &[u8], signature: &[u8]) -> Result<bool> {
    let mut mac =
        HmacSha256::new_from_slice(key).map_err(|e| KonquerorError::Crypto(e.to_string()))?;

    mac.update(data);

    Ok(mac.verify_slice(signature).is_ok())
}

type HmacSha256 = Hmac<Sha256>;

pub fn compute_hmac(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut mac =
        HmacSha256::new_from_slice(key).map_err(|e| KonquerorError::Crypto(e.to_string()))?;

    mac.update(data);

    let result = mac.finalize().into_bytes().to_vec();
    Ok(result)
}

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    validate_key(key)?;
    let key: &Key<Aes256Gcm> = Key::<Aes256Gcm>::from_slice(key);

    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .map_err(|e| KonquerorError::Crypto(e.to_string()))?;

    let mut out = Vec::with_capacity(nonce.len() + ciphertext.len());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn decrypt(input: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    validate_key(key)?;
    if input.len() < 12 {
        return Err(KonquerorError::Crypto("ciphertext too short".to_string()));
    }

    let key: &Key<Aes256Gcm> = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let (nonce_bytes, ciphertext) = input.split_at(12);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce_bytes);

    cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|e| KonquerorError::Crypto(e.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;
    use aes_gcm::aead::rand_core::RngCore;

    #[test]
    fn test_encryption_decryption() {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        let plaintext = vec![1, 2, 3];
        let encrypted = encrypt(&plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_invalid_key_encryption() {
        let key = [1];

        let plaintext = vec![1, 2, 3];
        encrypt(&plaintext, &key)
            .expect_err(&KonquerorError::Crypto("Key should be 32 bytes".to_string()).to_string());
    }

    #[test]
    fn test_invalid_key_decryption() {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let plaintext = vec![1, 2, 3];
        let encrypted = encrypt(&plaintext, &key).unwrap();

        let invalid_key = [1];
        decrypt(&encrypted, &invalid_key)
            .expect_err(&KonquerorError::Crypto("Key should be 32 bytes".to_string()).to_string());
    }

    #[test]
    fn test_hmac() {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        let data = vec![1, 2, 3];

        let signature = compute_hmac(&data, &key).unwrap();
        let verified = verify_hmac(&data, &key, &signature).unwrap();

        assert_eq!(verified, true);
    }

    #[test]
    #[cfg(feature = "password-hashing")]
    fn test_password_hashing() {
        let password = "password";

        let hash = hash_password(password).unwrap();
        let verify_password = verify_password(password, &hash.to_string()).unwrap();

        assert_eq!(verify_password, true);
    }
}
