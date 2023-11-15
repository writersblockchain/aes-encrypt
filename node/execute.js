import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://lcd.pulsar-3.secretsaturn.net",
  wallet: wallet,
  walletAddress: wallet.address,
});

// secret contract info
let contractCodeHash =
  "f27f1455e562a815b7295d7f5fb798676ef83f8e0aba6a616cbae55ebf37bc51";
let contractAddress = "secret1n6xpp4n8lp0qgd8acy68fnpwpkg0jk2zywaah9";
let encrypted_data;

let try_create_keys = async () => {
  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        create_keys: {},
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 2_000_000 }
  );

  console.log(tx);
};

// try_create_keys();

// decrypt the stored encrypted data sent from EVM
let try_decrypt = async () => {
  let encrypted_data = [
    111, 132, 2, 64, 126, 8, 29, 41, 195, 8, 65, 247, 235, 97, 40, 176, 249,
    239, 63, 82, 3, 34, 22, 23, 250, 142, 67, 110, 15, 198, 248, 191, 31, 128,
    3, 181, 97,
  ];

  const other_public_key = [
    2, 75, 166, 234, 147, 181, 26, 63, 160, 127, 107, 150, 184, 241, 160, 147,
    193, 94, 232, 161, 184, 86, 15, 114, 93, 212, 54, 199, 202, 125, 124, 121,
    249,
  ];

  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        try_decrypt: {
          ciphertext: encrypted_data,
          public_key: other_public_key,
        },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(tx);
};

try_decrypt();
