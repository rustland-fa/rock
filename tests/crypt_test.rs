
use rock::crypt::{decrypt, encrypt};


#[test]
pub fn crypt_test() {
    let key = "this-is-example-very-secret-keys".as_bytes();
    let msg = "Hello World!!!";
    let encrypt_msg = encrypt(key, msg.as_bytes()).unwrap();
    let decrypt_msg = decrypt(key, &encrypt_msg).unwrap();
    let decrypt_msg = std::str::from_utf8(&decrypt_msg).unwrap();
    assert_eq!(msg, decrypt_msg);
}
