use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn gen_string(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect()
    ).unwrap()
}

pub fn encrypt(data: &str, key: &str) -> String {
    let iv_str = gen_string(16);
    let iv = iv_str.as_bytes();
    let cipher = Aes128Cbc::new_from_slices(key.as_bytes(), iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())
}

pub fn decrypt(data: &str, key: &str) -> Option<String> {
    let bytes = match base64::decode(data) {
        Ok(v) => v,
        Err(_) => return None
    };
    let cipher = match Aes128Cbc::new_from_slices(key.as_bytes(), &bytes[0..16]) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let decrypt_byte = match cipher.decrypt_vec(&bytes[16..]) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let decrypt_str = match String::from_utf8(decrypt_byte) {
        Ok(v) => v,
        Err(_) => return None,
    };
    
    Some(decrypt_str)
}