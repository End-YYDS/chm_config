use aes::Aes256;
use block_padding::{Padding, Pkcs7};
use cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use generic_array::typenum::U16;
use hex::{decode, encode};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub path: String,
    pub password: String,
    pub kind: String,
    pub tools: Vec<String>,
}

#[allow(dead_code)]
impl Config {
    pub fn new(
        hostname: String,
        path: String,
        password: String,
        kind: String,
        tools: Vec<String>,
    ) -> Self {
        Self {
            hostname,
            path,
            password,
            kind,
            tools,
        }
    }
}
#[allow(dead_code)]
pub fn encrypt_password(password: &str, key: &[u8]) -> String {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut block: GenericArray<u8, U16> = GenericArray::clone_from_slice(&[0u8; 16]);
    let pos = password.len();
    block[..pos].copy_from_slice(password.as_bytes());
    Pkcs7::pad(&mut block, pos);
    let mut encrypted_block = block;
    cipher.encrypt_block(&mut encrypted_block);

    encode(encrypted_block)
}
#[allow(dead_code)]
pub fn decrypt_password(encrypted_password: &str, key: &[u8]) -> String {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let encrypted_bytes = decode(encrypted_password).expect("Decoding failed");

    let mut block: GenericArray<u8, U16> = GenericArray::clone_from_slice(&encrypted_bytes);
    cipher.decrypt_block(&mut block);

    let decrypted_bytes = Pkcs7::unpad(&block).unwrap();
    String::from_utf8(decrypted_bytes.to_vec()).expect("Invalid UTF-8")
}
