import { Keypair } from "@solana/web3.js";
import nacl from "tweetnacl";

const keyPair = Keypair.generate(); // Generating Key Pair

const publicKey = keyPair.publicKey;
const privateKey = keyPair.secretKey;

const message = new TextEncoder().encode("Fuck you");
console.log(message);

const sign = nacl.sign.detached(message, privateKey);
const verify = nacl.sign.detached.verify(message, sign, publicKey.toBytes());

console.log(verify);
console.log("PrivateKey : ", privateKey.toString("hex"));
console.log("Public Key : ", publicKey.toString("hex"));
