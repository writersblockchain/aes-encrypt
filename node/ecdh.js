import secp256k1 from "secp256k1/elliptic.js";
import { randomBytes } from "crypto";

function getPrivateKey() {
  while (true) {
    const privKey = randomBytes(32);
    if (secp256k1.privateKeyVerify(privKey)) return privKey;
  }
}

let privKey = getPrivateKey();

let secret_pubKey = new Uint8Array([
  2, 251, 140, 169, 155, 86, 119, 60, 69, 229, 123, 9, 125, 166, 206, 17, 236,
  150, 79, 147, 252, 105, 217, 217, 252, 250, 106, 248, 84, 219, 173, 55, 16,
]);

// // get the public key in a compressed format
let my_pubKey = secp256k1.publicKeyCreate(privKey);

const ecdhPointX = secp256k1.ecdh(secret_pubKey, privKey);

console.log("pub: ", pubKey);
console.log("priv: ", Array.from(privKey));
console.log("Shared secret (32 bytes):", Array.from(ecdhPointX));
