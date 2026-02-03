use aes::Aes128;
use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{
    BlockEncryptMut, BlockSizeUser, KeyIvInit,
};
use anyhow::{Result, anyhow};
use base64::{
    Engine as _,
    engine::general_purpose::STANDARD as BASE64_STANDARD,
};
use cbc::Encryptor;
use hex;
use rand::{Rng, thread_rng};
use rsa::{
    BigUint, RsaPublicKey, pkcs8::DecodePublicKey,
    traits::PublicKeyParts,
};
use serde::Serialize;
use std::collections::HashMap;

// Define the type alias correctly using aes::Aes128 and cbc::Encryptor
type Aes128CbcEnc = Encryptor<Aes128>;

const IV: &[u8] = b"0102030405060708";
const PRESET_KEY: &[u8] = b"0CoJUm6Qyw8W8jud";
const BASE62: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ3
7BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvakl
V8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44o
ncaTWz7OBGLbCiK45wIDAQAB
-----END PUBLIC KEY-----"#;

/// Implements the NetEase weapi encryption.
pub fn weapi(
    data: &impl Serialize,
) -> Result<HashMap<String, String>> {
    let text = serde_json::to_string(data)?;

    let mut rng = thread_rng();
    let mut secret_key = [0u8; 16];
    for i in 0..16 {
        secret_key[i] =
            BASE62[rng.gen_range(0..BASE62.len())];
    }

    let params =
        aes_encrypt(text.as_bytes(), PRESET_KEY, IV)?;
    let params_b64 = BASE64_STANDARD.encode(params);
    let params = aes_encrypt(
        params_b64.as_bytes(),
        &secret_key,
        IV,
    )?;

    let mut reversed_key = secret_key.to_vec();
    reversed_key.reverse();

    let public_key =
        RsaPublicKey::from_public_key_pem(PUBLIC_KEY_PEM)
            .map_err(|e| {
            anyhow!("Failed to load public key: {}", e)
        })?;

    // RSA Encryption (NoPadding / Manual modular exponentiation)
    let n = public_key.n();
    let e = public_key.e();

    let mut padded_key = vec![0u8; 128];
    let start = 128 - reversed_key.len();
    padded_key[start..].copy_from_slice(&reversed_key);

    let m = BigUint::from_bytes_be(&padded_key);
    let c = m.modpow(e, n);
    let enc_sec_key_bytes = c.to_bytes_be();

    // Pad to 128 bytes to ensure 256 hex chars
    let mut enc_sec_key = vec![0u8; 128];
    let offset = 128 - enc_sec_key_bytes.len();
    enc_sec_key[offset..]
        .copy_from_slice(&enc_sec_key_bytes);

    let mut map = HashMap::new();
    map.insert(
        "params".to_string(),
        BASE64_STANDARD.encode(params),
    );
    map.insert(
        "encSecKey".to_string(),
        hex::encode(enc_sec_key),
    );

    Ok(map)
}

fn aes_encrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>> {
    let cipher = Aes128CbcEnc::new_from_slices(key, iv)
        .map_err(|e| {
            anyhow!("Failed to create AES cipher: {}", e)
        })?;

    // Allocate a buffer large enough for plaintext + padding (at least 1 block size larger).
    let mut buffer =
        vec![0u8; data.len() + Aes128CbcEnc::block_size()];
    // Copy plaintext into the buffer.
    buffer[..data.len()].copy_from_slice(data);

    // Call encrypt_padded_mut.
    // The function returns the ciphertext slice.
    let ciphertext = cipher
        .encrypt_padded_mut::<Pkcs7>(
            &mut buffer,
            data.len(),
        )
        .map_err(|e| {
            anyhow!("AES encryption failed: {}", e)
        })?;

    Ok(ciphertext.to_vec())
}
