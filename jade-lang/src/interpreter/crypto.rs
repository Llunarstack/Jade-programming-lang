//! Enigma-style cryptography: AEAD, signing, key exchange, secrets.
//! Lives in the interpreter so builtins and eval can use it without crate-level crypto.

use aes_gcm::{
    aead::{Aead as AesAead, KeyInit as AesKeyInit, Payload as AesPayload},
    Aes256Gcm, Nonce as AesNonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, Params,
};
use chacha20poly1305::{
    aead::Payload,
    ChaCha20Poly1305, Nonce,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use subtle::ConstantTimeEq;
use x25519_dalek::{PublicKey, StaticSecret};

/// Hybrid keypair combining Ed25519 (signing) and X25519 (key exchange)
#[allow(dead_code)]
#[derive(Clone)]
pub struct EnigmaKeypair {
    #[allow(dead_code)]
    pub signing_key: SigningKey,
    #[allow(dead_code)]
    pub exchange_secret: StaticSecret,
}

impl EnigmaKeypair {
    /// Generate a new master keypair
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let exchange_secret = StaticSecret::random_from_rng(csprng);

        Self {
            signing_key,
            exchange_secret,
        }
    }

    #[allow(dead_code)]
    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    #[allow(dead_code)]
    pub fn public_key(&self) -> PublicKey {
        PublicKey::from(&self.exchange_secret)
    }

    #[allow(dead_code)]
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    #[allow(dead_code)]
    pub fn derive_session_key(
        &self,
        peer_public: &PublicKey,
        context: &str,
        key_length: usize,
        nonce_length: usize,
    ) -> Result<(Vec<u8>, Vec<u8>), String> {
        let shared_secret = self.exchange_secret.diffie_hellman(peer_public);
        let hk = Hkdf::<Sha256>::new(Some(context.as_bytes()), shared_secret.as_bytes());
        let mut key = vec![0u8; key_length];
        let mut nonce = vec![0u8; nonce_length];
        hk.expand(b"enigma-key", &mut key)
            .map_err(|_| "Failed to derive key")?;
        hk.expand(b"enigma-nonce", &mut nonce)
            .map_err(|_| "Failed to derive nonce")?;
        Ok((key, nonce))
    }
}

pub fn enigma_encrypt(
    plaintext: &[u8],
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err(format!(
            "Key must be 32 bytes for 2026 security level, got {}",
            key.len()
        ));
    }
    if nonce.len() != 12 {
        return Err(format!("Nonce must be 12 bytes, got {}", nonce.len()));
    }
    let cipher =
        ChaCha20Poly1305::new_from_slice(key).map_err(|_| "Invalid key length".to_string())?;
    let nonce_array = Nonce::from_slice(nonce);
    let payload = Payload { msg: plaintext, aad };
    cipher
        .encrypt(nonce_array, payload)
        .map_err(|_| "Encryption failed".to_string())
}

pub fn enigma_decrypt(
    ciphertext: &[u8],
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }
    if nonce.len() != 12 {
        return Err("Nonce must be 12 bytes".to_string());
    }
    let cipher =
        ChaCha20Poly1305::new_from_slice(key).map_err(|_| "Invalid key length".to_string())?;
    let nonce_array = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: ciphertext,
        aad,
    };
    cipher
        .decrypt(nonce_array, payload)
        .map_err(|_| "Decryption failed - tampered or wrong key".to_string())
}

pub fn aes_encrypt(
    plaintext: &[u8],
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err(format!(
            "Key must be 32 bytes for AES-256, got {}",
            key.len()
        ));
    }
    if nonce.len() != 12 {
        return Err(format!(
            "Nonce must be 12 bytes for GCM, got {}",
            nonce.len()
        ));
    }
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "Invalid key length".to_string())?;
    let nonce_array = AesNonce::from_slice(nonce);
    let payload = AesPayload {
        msg: plaintext,
        aad,
    };
    cipher
        .encrypt(nonce_array, payload)
        .map_err(|_| "AES encryption failed".to_string())
}

pub fn aes_decrypt(
    ciphertext: &[u8],
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }
    if nonce.len() != 12 {
        return Err("Nonce must be 12 bytes".to_string());
    }
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| "Invalid key length".to_string())?;
    let nonce_array = AesNonce::from_slice(nonce);
    let payload = AesPayload {
        msg: ciphertext,
        aad,
    };
    cipher
        .decrypt(nonce_array, payload)
        .map_err(|_| "AES decryption failed - tampered or wrong key".to_string())
}

pub fn random_bytes(length: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut bytes = vec![0u8; length];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Result<Vec<u8>, String> {
    if a.len() != b.len() {
        return Err(format!(
            "XOR requires equal length inputs: {} != {}",
            a.len(),
            b.len()
        ));
    }
    Ok(a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect())
}

pub fn derive_password_key(
    password: &str,
    salt: &[u8],
    ops_limit: u32,
    mem_limit_kb: u32,
) -> Result<Vec<u8>, String> {
    if salt.len() < 16 {
        return Err("Salt must be at least 16 bytes".to_string());
    }
    let params = Params::new(
        mem_limit_kb,
        ops_limit,
        1,
        Some(32),
    )
    .map_err(|e| format!("Invalid Argon2 params: {}", e))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let salt_str =
        SaltString::encode_b64(salt).map_err(|e| format!("Failed to encode salt: {}", e))?;
    let hash = argon2
        .hash_password(password.as_bytes(), &salt_str)
        .map_err(|e| format!("Argon2 hashing failed: {}", e))?;
    let hash_bytes = hash.hash.ok_or("No hash output")?.as_bytes().to_vec();
    Ok(hash_bytes)
}

pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

pub fn generate_salt(length: usize) -> Vec<u8> {
    random_bytes(length)
}

pub fn generate_nonce(length: usize) -> Vec<u8> {
    random_bytes(length)
}
