use aes_siv::aead::generic_array::GenericArray;
use aes_siv::siv::Aes128Siv;
use log::*;
use secp256k1::ecdh::SharedSecret;
use secp256k1::{PublicKey, SecretKey};
use secret_encryption::{ContractError, CryptoError};

fn main() {
    // Hardcoded encrypted data
    let encrypted_data = [
        111, 132, 2, 64, 126, 8, 29, 41, 195, 8, 65, 247, 235, 97, 40, 176, 249, 239, 63, 82, 3,
        34, 22, 23, 250, 142, 67, 110, 15, 198, 248, 191, 31, 128, 3, 181, 97,
    ];

    // let other_public_key = [
    //     2, 75, 166, 234, 147, 181, 26, 63, 160, 127, 107, 150, 184, 241, 160, 147, 193, 94, 232,
    //     161, 184, 86, 15, 114, 93, 212, 54, 199, 202, 125, 124, 121, 249,
    // ];

    // let my_private_key = [
    //     5, 235, 171, 38, 245, 27, 57, 248, 115, 21, 12, 246, 106, 234, 193, 17, 90, 215, 108, 97,
    //     60, 56, 16, 193, 146, 150, 243, 104, 236, 18, 197, 134,
    // ];

    // let my_private_key = SecretKey::from_slice(my_private_key.as_slice()).unwrap();

    // let my_public_key = PublicKey::from_slice(other_public_key.as_slice()).unwrap();

    // let key = SharedSecret::new(&my_public_key, &my_private_key);
    // println!("SharedSecret: {:?}", key);
    let key = [
        17, 236, 81, 217, 126, 148, 50, 226, 141, 52, 161, 169, 48, 30, 138, 7, 50, 240, 246, 101,
        20, 1, 41, 13, 20, 198, 83, 230, 245, 50, 63, 32,
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
