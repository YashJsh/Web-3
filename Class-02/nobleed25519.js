import * as ed from "@noble/ed25519"

async function main(){
    const privateKey = ed.utils.randomSecretKey(); //Generating Random private key
    const message = "Hello world";
    const encoded = new TextEncoder().encode(message); // Encoding into UInt8Array

    //Generate public key from private key;
    const publicKey = await ed.getPublicKeyAsync(privateKey);
    const sign = await ed.signAsync(encoded, privateKey);

    const isValid = await ed.verifyAsync(sign, encoded, publicKey);

    console.log(isValid);
    console.log("PrivateKey : ", privateKey.toString());
    console.log("Public Key : ", publicKey.toString("hex"));
    console.log("Message : ", encoded.toString("hex"));
}

main();