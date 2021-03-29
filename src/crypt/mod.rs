use crate::{config::NONCE_LENGTH, utils::generate_random_bytes};
use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, NewAead},
    Aes256GcmSiv,
};

pub fn encrypt(key: &[u8], plain_txt_msg: &[u8]) -> crate::Result<Vec<u8>> {
    if key.len() < 32 {
        return Err("key in valid".into());
    }
    let key = GenericArray::from_slice(key);
    let cipher = Aes256GcmSiv::new(key);
    let random_bytes = generate_random_bytes();
    let nonce = GenericArray::from_slice(&random_bytes);
    let encrypt_msg = cipher
        .encrypt(nonce, plain_txt_msg)
        .map_err(|e| e.to_string())?;
    let mut cipher_msg = Vec::new();
    cipher_msg.extend_from_slice(&random_bytes);
    cipher_msg.extend(encrypt_msg);
    Ok(cipher_msg)
}

pub fn decrypt(key: &[u8], cipher_msg: &[u8]) -> crate::Result<Vec<u8>> {
    if cipher_msg.len() <= NONCE_LENGTH {
        return Err("msg is invalid".into());
    }
    let key = GenericArray::from_slice(key);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = GenericArray::from_slice(&cipher_msg[..NONCE_LENGTH]);
    let plain_txt_msg = cipher
        .decrypt(nonce, &cipher_msg[NONCE_LENGTH..])
        .map_err(|e| e.to_string())?;
    Ok(plain_txt_msg)
}
