import {
  address,
  addSignersToTransactionMessage,
  airdropFactory,
  appendTransactionMessageInstruction,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  generateKeyPair,
  generateKeyPairSigner,
  getBase64EncodedWireTransaction,
  getProgramDerivedAddress,
  getSignatureFromTransaction,
  lamports,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type FullySignedTransaction,
  type Instruction,
  type TransactionWithBlockhashLifetime,
} from "@solana/kit";
import { getStructCodec, getU8Codec, getUtf8Encoder } from "@solana/codecs";
import { getAddressEncoder } from "@solana/addresses";
import bs58 from "bs58";


const PROGRAM_ID = address("DDkSqdNoZXAoXDWb6Xd5Yo7sJEi4EVusQmD4RKNr67Uj");

const Initialize = async () => {
    const rpc = createSolanaRpc("https://api.devnet.solana.com");
    const rpcSubscriptions = createSolanaRpcSubscriptions("wss://api.devnet.solana.com");
    const privateKeyBase58 = "Privatekey"; //This is for raw testing
    const privateKeyBytes = bs58.decode(privateKeyBase58);
    const signer = await createKeyPairSignerFromBytes(privateKeyBytes);

    try {
        const initializeVaultCodec = getStructCodec([
            ["discriminator", getU8Codec()],
            ["bump", getU8Codec()],
        ]);
        const [vault, bump] = await getProgramDerivedAddress({
            programAddress: PROGRAM_ID,
            seeds: [
                getUtf8Encoder().encode("vault"),
                getAddressEncoder().encode(signer.address)
            ]
        });
        console.log("Vault address is : ", vault);
        const instructionData = initializeVaultCodec.encode({
            discriminator: 0,
            bump: bump
        });
        const initializeInstruction: Instruction = {
            programAddress: PROGRAM_ID,
            accounts: [
                {
                    address: signer.address,
                    role: 3

                },
                {
                    address: vault,
                    role: 1
                },
                {
                    address: address("11111111111111111111111111111111"),
                    role: 0
                }
            ],
            data: instructionData
        };
        const { value: { blockhash, lastValidBlockHeight } } = await rpc.getLatestBlockhash().send();
        const transactionMessage = pipe(
            createTransactionMessage({ version: "legacy" }),
            tx => setTransactionMessageFeePayer(signer.address, tx),
            tx => appendTransactionMessageInstruction(initializeInstruction, tx),
            tx => setTransactionMessageLifetimeUsingBlockhash({ blockhash, lastValidBlockHeight }, tx),
            tx => addSignersToTransactionMessage([signer], tx)
        );

        const signedTransaction = await signTransactionMessageWithSigners(transactionMessage);

        const simulate = await rpc
            .simulateTransaction(getBase64EncodedWireTransaction(signedTransaction), { encoding: "base64" })
            .send();
        console.log("Signature:", simulate);

        if (simulate.value.err) {
            return;
        };
        // const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });

        // await sendAndConfirmTransaction(signedTransaction, { commitment: "confirmed" });

        // const sx = getSignatureFromTransaction(signedTransaction);
        // console.log(sx);
    } catch (error) {
        console.log("Error in instruction");
        console.error(error);
    }
}


Initialize();