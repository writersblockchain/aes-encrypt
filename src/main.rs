use aes_siv::aead::generic_array::GenericArray;
use aes_siv::siv::Aes128Siv;
use log::*;
use secret_template::CryptoError;
fn main() {
    let ciphertext = vec![
        201, 202, 215, 190, 169, 13, 70, 53, 7, 190, 148, 202, 89, 132, 141, 9, 194, 94, 23, 68,
        145, 33, 11, 166, 204, 110,
    ];

    let key = vec![
        95, 98, 72, 63, 36, 62, 8, 90, 193, 87, 172, 132, 201, 103, 159, 142, 42, 137, 127, 241,
        26, 122, 173, 122, 239, 105, 7, 213, 108, 212, 242, 58,
    ];

    let decrypted_data = aes_siv_decrypt(&ciphertext, None, &key);
    match decrypted_data {
        Ok(data) => println!("Decrypted data: {:?}", data),
        Err(e) => println!("Decryption failed: {:?}", e),
    }
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
