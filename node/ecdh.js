import secp256k1 from "secp256k1/elliptic.js";
import { randomBytes } from "crypto";

function getPrivateKey() {
  while (true) {
    const privKey = randomBytes(32);
    if (secp256k1.privateKeyVerify(privKey)) return privKey;
  }
}

let privKey = getPrivateKey();

// get the public key in a compressed format
let pubKey = secp256k1.publicKeyCreate(privKey);

// let pubKey =
// compressed public key from X and Y
function hashfn(x, y) {
  const pubKey = new Uint8Array(33);
  pubKey[0] = (y[31] & 1) === 0 ? 0x02 : 0x03;
  pubKey.set(x, 1);
  return pubKey;
}

// get X point of ecdh
const ecdhPointX = secp256k1.ecdh(
  pubKey,
  privKey,
  { hashfn },
  Buffer.alloc(33)
);
console.log("pub: ", pubKey);
console.log("priv: ", Array.from(privKey));
console.log("shared: ", Array.from(ecdhPointX));
