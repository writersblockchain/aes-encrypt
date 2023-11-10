use aes_siv::aead::generic_array::GenericArray;
use aes_siv::siv::Aes128Siv;
use log::*;
use secret_encryption::CryptoError;

fn main() {
    // Hardcoded encrypted data
    let encrypted_data = [
        218, 106, 140, 26, 40, 206, 183, 135, 65, 31, 50, 8, 165, 230, 224, 229, 80, 6, 213, 74,
        122, 50, 171, 110, 187, 166, 109, 114, 214, 249, 187, 97,
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
