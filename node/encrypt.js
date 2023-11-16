import * as miscreant from "miscreant";
import { fromBase64, fromHex, toUtf8 } from "@cosmjs/encoding";
import { PrivateKey, utils } from "eciesjs";

let provider = new miscreant.PolyfillCryptoProvider();

// const keyData = new Uint8Array(32).fill(1);
const keyData = Uint8Array.from([
  60, 184, 13, 235, 55, 172, 25, 14, 189, 123, 114, 89, 244, 238, 150, 142, 149,
  137, 118, 127, 54, 180, 131, 31, 171, 74, 134, 162, 194, 82, 197, 183,
]);

let encrypt = async (msg, associatedData = []) => {
  const siv = await miscreant.SIV.importKey(keyData, "AES-SIV", provider);
  const plaintext = toUtf8(JSON.stringify(msg));

  try {
    const ciphertext = await siv.seal(plaintext, associatedData);
    console.log("Encrypted data:", ciphertext);
    return ciphertext;
  } catch (e) {
    console.warn("Error encrypting data:", e);
    throw e;
  }
};

let decrypt = async (ciphertext, associatedData = []) => {
  const siv = await miscreant.SIV.importKey(keyData, "AES-SIV", provider);

  try {
    let decrypted = await siv.open(ciphertext, associatedData);
    const convertedString = String.fromCharCode(...decrypted);
    console.log("Decrypted data:", convertedString);
    return convertedString;
  } catch (e) {
    console.warn("Error decrypting data:", e);
    throw e;
  }
};

// Usage example
let msg = { i_like_turtles: "I like turtles!" };
encrypt(msg).then((ciphertext) => {
  decrypt(ciphertext);
});
