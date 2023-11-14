import { PrivateKey, utils } from "eciesjs";
const sk = new PrivateKey();
const shared = new utils.getValidSecret(
  sk,
  [
    3, 127, 119, 51, 24, 246, 191, 179, 68, 149, 170, 44, 129, 90, 23, 226, 124,
    239, 23, 231, 14, 78, 136, 213, 91, 138, 104, 108, 115, 68, 0, 220, 74,
  ]
);
console.log("shared: ", shared);

console.log(sk);
