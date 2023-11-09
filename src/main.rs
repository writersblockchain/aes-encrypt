use aes_siv::aead::generic_array::GenericArray;
use aes_siv::siv::Aes128Siv;
use log::*;
use secret_encryption::CryptoError;

fn main() {
    // Hardcoded encrypted data
    let encrypted_data = [
        175, 69, 13, 254, 115, 57, 164, 144, 25, 224, 155, 72, 84, 30, 25, 216, 1, 55, 116, 100,
        59, 95, 104, 233, 87, 54, 35, 96, 207, 192, 26, 48, 218, 47, 103, 20, 91, 85, 29, 118, 33,
        146, 136, 194, 101, 52, 168, 61, 9, 98, 237, 5,
    ];

    let key = [1; 32]; // Fixed key

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
