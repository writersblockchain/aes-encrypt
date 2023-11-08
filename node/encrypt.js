import * as miscreant from "miscreant";
import { randomFillSync } from "crypto";
const cryptoProvider = new miscreant.PolyfillCryptoProvider();
async function encryptData(
  plaintextArray,
  keyData,
  nonceArray,
  associatedData = ""
) {
  // Import the key
  let key = await miscreant.AEAD.importKey(keyData, "AES-SIV", cryptoProvider);

  // Encrypt the plaintext
  let ciphertext = await key.seal(plaintextArray, nonceArray, associatedData);

  return ciphertext;
}

// Usage example
(async () => {
  let keyData = new Uint8Array(32); // Key must be either 32 bytes (AES-128) or 64 bytes (AES-256)
  randomFillSync(keyData); // Random key for the example

  let nonce = new Uint8Array(16); // Nonce should be unique for each message
  randomFillSync(nonce);

  let plaintext = new Uint8Array([2, 3, 5, 7, 11, 13, 17, 19, 23, 29]); // Example plaintext data

  let ciphertext = await encryptData(plaintext, keyData, nonce);

  console.log("Ciphertext:", ciphertext);
  console.log("Key:", keyData);
  console.log("Nonce:", nonce);
})();
