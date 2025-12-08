//Wallet Address Generation - 
//1. Generate a mnemonic
//2. Generate a seed from the mnemonic
//3. Generate a wallet address from the seed
import { generateMnemonic, mnemonicToSeedSync} from "bip39";

const mnemonic = generateMnemonic();
const seed = mnemonicToSeedSync(mnemonic);

console.log(seed.length);
console.log(seed.toString('hex'));


// Let's generate 3 different public private key pair from this 
for (let i = 0; i < 3; i++) {
    const path = `m/44'/60'/0'/0/${i}`;
    const child = root.derivePath(path);
    const publicKey = child.publicKey;
    const privateKey = child.privateKey;
    console.log(`Public Key ${i}:`, publicKey);
    console.log(`Private Key ${i}:`, privateKey);
}