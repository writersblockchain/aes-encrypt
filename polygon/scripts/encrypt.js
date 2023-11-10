const miscreant = require("miscreant");
const { fromBase64, fromHex, toUtf8 } = require("@cosmjs/encoding");
const { ethers } = require("hardhat");

let provider = new miscreant.PolyfillCryptoProvider();
let ciphertext;

let encrypt = async (msg, associatedData = []) => {
  const keyData = new Uint8Array(32).fill(1);
  const siv = await miscreant.SIV.importKey(keyData, "AES-SIV", provider);
  const plaintext = toUtf8(JSON.stringify(msg));

  try {
    ciphertext = await siv.seal(plaintext, associatedData);
    console.log("Encrypted data:", ciphertext);
    return ciphertext;
  } catch (e) {
    console.warn("Error encrypting data:", e);
    throw e;
  }
};

async function encrypt_evm() {
  const sendReceiveEncryptAddress =
    "0x0DC75cB5CE7335fa335b03F34d6f9a7697fA9336"; // Replace with your deployed contract's address
  const destinationChain = "secret"; // Replace with your desired destination chain
  const destinationAddress = "secret1w7m5kxz2sslmhdkf5nyx3hneq4e7fzd6luvppp"; // Replace with your desired destination address

  let msg = { seanrad: "seanrad" };
  let my_encrypted_message = await encrypt(msg);
  const SendReceiveEncrypt = await ethers.getContractFactory(
    "SendReceiveEncrypt"
  );
  const sendReceiveEncrypt = await SendReceiveEncrypt.attach(
    sendReceiveEncryptAddress
  );

  const tx = await sendReceiveEncrypt.send(
    destinationChain,
    destinationAddress,
    my_encrypted_message,
    {
      value: ethers.utils.parseEther("0.3"), // Adjust the amount as needed for gas
    }
  );

  console.log(`Transaction hash: ${tx.hash}`);
  await tx.wait();

  console.log("send function executed successfully!");
}
encrypt_evm();
