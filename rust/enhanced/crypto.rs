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
use cbc::cipher::Cipher;
use hex;
use rand::{Rng, thread_rng};
use rsa::{
    Pkcs1v15Encrypt, RsaPublicKey, pkcs8::DecodePublicKey,
};
use serde::Serialize;
use std::collections::HashMap;

// Define the type alias correctly using aes::Aes128 and cbc::Encryptor
type Aes128CbcEnc = Encryptor<Aes128>;

const IV: &[u8] = b"0102030405060708";
const PRESET_KEY: &[u8] = b"0CoJUm6Qyw8W8jud";
const BASE62: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDgtQn2JZ34ZC28NWYpAUd98iZ37BUrX/aKzmFbt7clFSs6sXqHauqKWqdtLkF2KexO40H1YTX8z2lSgBBOAxLsvaklV8k4cBFK9snQXE9/DDaFt6Rr7iVZMldczhC0JNgTz+SHXT6CBHuX3e9SdB1Ua44oncaTWz7OBGLbCiK45wIDAQAB\n-----END PUBLIC KEY-----";

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
    let params = aes_encrypt(&params, &secret_key, IV)?;

    let mut reversed_key = secret_key.to_vec();
    reversed_key.reverse();

    let public_key =
        RsaPublicKey::from_public_key_pem(PUBLIC_KEY_PEM)
            .map_err(|e| {
            anyhow!("Failed to load public key: {}", e)
        })?;

    let enc_sec_key = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &reversed_key)
        .map_err(|e| {
            anyhow!("RSA encryption failed: {}", e)
        })?;

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
    let mut cipher = Aes128CbcEnc::new_from_slices(key, iv)
        .map_err(|e| {
            anyhow!("Failed to create AES cipher: {}", e)
        })?;

    // Allocate a buffer large enough for plaintext + padding (at least 1 block size larger).
    let mut buffer =
        vec![0u8; data.len() + Aes128CbcEnc::block_size()];
    // Copy plaintext into the buffer.
    buffer[..data.len()].copy_from_slice(data);

    // Call encrypt_padded_mut.
    // The function returns the length of the ciphertext.
    let ciphertext_len = cipher
        .encrypt_padded_mut::<Pkcs7>(
            &mut buffer,
            data.len(),
        )
        .map_err(|e| {
            anyhow!("AES encryption failed: {}", e)
        })?;

    // Truncate the buffer to the actual ciphertext length.
    buffer.truncate(ciphertext_len);

    Ok(buffer)
}
