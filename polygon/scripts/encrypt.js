const miscreant = require("miscreant");
const { fromBase64, fromHex, toUtf8 } = require("@cosmjs/encoding");
const { ethers } = require("hardhat");

let provider = new miscreant.PolyfillCryptoProvider();
let ciphertext;

let keyData = Uint8Array.from([
  188, 131, 212, 28, 13, 250, 169, 192, 183, 66, 222, 180, 252, 243, 131, 8,
  242, 65, 77, 117, 36, 229, 79, 91, 29, 225, 105, 180, 30, 15, 195, 177,
]);

let encrypt = async (msg, associatedData = []) => {
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
  const destinationAddress = "secret1n6xpp4n8lp0qgd8acy68fnpwpkg0jk2zywaah9"; // Replace with your desired destination address

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
