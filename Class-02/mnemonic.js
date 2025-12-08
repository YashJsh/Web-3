import nacl from "tweetnacl";
import { generateMnemonic, mnemonicToSeedSync } from "bip39";
import { derivePath } from "ed25519-hd-key";
import { Keypair } from "@solana/web3.js";

const mnemonic = generateMnemonic(256);
const mnemonic2 = "destroy retire candy cotton asset field wheel gate auction borrow join predict"

console.log(mnemonic);

const seed = mnemonicToSeedSync(mnemonic2);

console.log(seed.toString("base64"));

for (let i = 0; i<4; i++){
    const path = `m/44'/501'/${i}'/0'`;
    const derivedSeed = derivePath(path, seed.toString("hex")).key;
    const secret = nacl.sign.keyPair.fromSeed(derivedSeed).secretKey;
    console.log(Keypair.fromSecretKey(secret).publicKey.toBase58());
}

//clap office mixed month dad surge cargo manual jar bone cool process aspect soccer tide maple bargain ring another rapid square retire peace jungle


// A derivation path is typically expressed in a format like m / purpose' / coin_type' / account' / change / address_index