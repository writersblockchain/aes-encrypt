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

let get_decrypted = async () => {
  let query = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    query: {
      get_decrypted: {},
    },
    code_hash: contractCodeHash,
  });

  console.log(query);
};
get_decrypted();

// Query the contract for the stored message sent from Polygon
let get_stored_message = async () => {
  let query = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    query: {
      get_stored_message: {},
    },
    code_hash: contractCodeHash,
  });

  const hexString = query.message;

  // Convert the hex string to a byte array
  const byteArray = [];
  for (let i = 0; i < hexString.length; i += 2) {
    byteArray.push(parseInt(hexString.substring(i, i + 2), 16));
  }

  console.log(byteArray);
};

// get_stored_message();

let get_keys = async () => {
  let query = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    query: {
      get_keys: {},
    },
    code_hash: contractCodeHash,
  });

  console.log(query);
};

// get_keys();
