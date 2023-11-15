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
  "4824f41a114aaab5603abc5ce0397f4727acfe25219ab1aa5cd621af3b4a5921";
let contractAddress = "secret1dheq635zxrwug0w522anj5kv2gl04ht9zetsla";
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
    { gasLimit: 300_000 }
  );

  console.log(tx);
};

try_create_keys();

// get stored encrypted data sent from EVM
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
  encrypted_data = byteArray;
};

// decrypt the stored encrypted data sent from EVM
let try_decrypt = async () => {
  await get_stored_message();
  // let encrypted_data = [
  //   61, 58, 249, 158, 14, 64, 54, 38, 168, 229, 117, 233, 59, 229, 146, 155,
  //   147, 193, 54, 242, 105, 154, 140, 230, 49, 145, 75, 43, 94, 102, 223, 127,
  //   60, 76, 132, 197, 87, 41, 69, 156, 247, 39, 184, 249, 30, 199, 191, 240,
  //   145, 223, 111, 181,
  // ];

  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        try_decrypt: { ciphertext: await encrypted_data },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(tx);
};

// try_decrypt();
