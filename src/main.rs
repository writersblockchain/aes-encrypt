use aes_siv::aead::generic_array::GenericArray;
use aes_siv::siv::Aes128Siv;
use log::*;
use secp256k1::ecdh::SharedSecret;
use secp256k1::{PublicKey, SecretKey};
use secret_encryption::{ContractError, CryptoError};

fn main() {
    // Hardcoded encrypted data
    let encrypted_data = [
        141, 67, 69, 100, 172, 91, 58, 7, 0, 82, 215, 142, 28, 252, 121, 100, 96, 97, 28, 108, 39,
        5, 216, 98, 140, 163, 194, 79, 153, 171, 89, 184, 58, 215, 19, 156, 36, 250, 251, 25, 177,
        139, 167, 171, 182, 58, 9, 249, 93, 82, 141, 123,
    ];

    let other_public_key = [
        2, 15, 59, 161, 106, 5, 104, 87, 221, 54, 12, 86, 168, 126, 144, 126, 162, 146, 92, 80, 95,
        60, 160, 98, 199, 229, 156, 54, 55, 252, 47, 144, 157,
    ];

    let my_private_key = [
        5, 235, 171, 38, 245, 27, 57, 248, 115, 21, 12, 246, 106, 234, 193, 17, 90, 215, 108, 97,
        60, 56, 16, 193, 146, 150, 243, 104, 236, 18, 197, 134,
    ];

    let my_private_key = SecretKey::from_slice(my_private_key.as_slice()).unwrap();

    let my_public_key = PublicKey::from_slice(other_public_key.as_slice()).unwrap();

    // let key = SharedSecret::new(&my_public_key, &my_private_key);
    // println!("SharedSecret: {:?}", key);
    let key = [
        60, 184, 13, 235, 55, 172, 25, 14, 189, 123, 114, 89, 244, 238, 150, 142, 149, 137, 118,
        127, 54, 180, 131, 31, 171, 74, 134, 162, 194, 82, 197, 183,
    ];

    // Convert associated data to the correct type
    let ad_data: &[&[u8]] = &[];
    let ad = Some(ad_data);

    // Decrypt
    match aes_siv_decrypt(&encrypted_data, ad, &key) {
        Ok(decrypted_data) => {
            println!(
                "Decrypted data: {:?}",
                String::from_utf8(decrypted_data).unwrap()
            );
        }
        Err(e) => {
            warn!("Error decrypting data: {:?}", e);
        }
    }
}

fn aes_siv_encrypt(
    plaintext: &[u8],
    ad: Option<&[&[u8]]>,
    key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let ad = ad.unwrap_or(&[&[]]);

    let mut cipher = Aes128Siv::new(GenericArray::clone_from_slice(key));
    cipher.encrypt(ad, plaintext).map_err(|e| {
        warn!("aes_siv_encrypt error: {:?}", e);
        CryptoError::EncryptionError
    })
}

fn aes_siv_decrypt(
    ciphertext: &[u8],
    ad: Option<&[&[u8]]>,
    key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let ad = ad.unwrap_or(&[&[]]);

    let mut cipher = Aes128Siv::new(GenericArray::clone_from_slice(key));
    cipher.decrypt(ad, ciphertext).map_err(|e| {
        warn!("aes_siv_decrypt error: {:?}", e);
        CryptoError::DecryptionError
    })
}

fn hex_to_byte_array(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_str)
}
