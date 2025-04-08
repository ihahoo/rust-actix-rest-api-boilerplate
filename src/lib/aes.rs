use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes128Gcm,
    Nonce,
};
use rand::Rng;
use base64::{Engine as _, engine::general_purpose::STANDARD};

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn gen_string(size: usize) -> String {
    let mut rng = rand::rng();
    (0..size)
        .map(|_| {
            let idx = rng.random_range(0..BASE_STR.len());
            BASE_STR.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn encrypt(data: &str, key: &str) -> String {
    // 确保密钥长度为16字节
    let key_bytes = if key.len() >= 16 {
        key.as_bytes()[0..16].to_vec()
    } else {
        let mut padded = key.as_bytes().to_vec();
        padded.resize(16, 0);
        padded
    };

    let cipher = Aes128Gcm::new_from_slice(&key_bytes).unwrap();
    let nonce_str = gen_string(12);  // AES-GCM 使用12字节的 nonce
    let nonce = Nonce::from_slice(nonce_str.as_bytes());
    
    let ciphertext = cipher.encrypt(nonce, data.as_bytes()).unwrap();
    let mut result = nonce_str.as_bytes().to_vec();
    result.extend_from_slice(&ciphertext);
    STANDARD.encode(result)
}

pub fn decrypt(data: &str, key: &str) -> Option<String> {
    // 确保密钥长度为16字节
    let key_bytes = if key.len() >= 16 {
        key.as_bytes()[0..16].to_vec()
    } else {
        let mut padded = key.as_bytes().to_vec();
        padded.resize(16, 0);
        padded
    };

    let bytes = STANDARD.decode(data).ok()?;
    if bytes.len() <= 12 {
        return None;
    }

    let cipher = Aes128Gcm::new_from_slice(&key_bytes).ok()?;
    let nonce = Nonce::from_slice(&bytes[0..12]);
    
    let plaintext = cipher.decrypt(nonce, &bytes[12..]).ok()?;
    String::from_utf8(plaintext).ok()
}