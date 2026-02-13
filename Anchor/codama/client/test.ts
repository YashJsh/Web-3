import { PublicKey, SystemProgram } from "@solana/web3.js";

import { getIncrementCounterInstruction } from "./clients/js/src/generated/instructions/incrementCounter";
import { getInitializeInstruction } from "./clients/js/src/generated/instructions/initialize";
import { createSolanaRpc, generateKeyPairSigner, address, createTransactionMessage, appendTransactionMessageInstruction, setTransactionMessageFeePayerSigner, setTransactionMessageLifetimeUsingBlockhash, signAndSendTransactionMessageWithSigners, lamports, pipe, signTransaction, signTransactionMessageWithSigners, sendAndConfirmTransactionFactory, createSolanaRpcSubscriptions, getSignatureFromTransaction, } from "@solana/kit";

const ProgramId = new PublicKey(
    "tEstkZyM2FqJBECcFNw4VcuEib3r2kAqavF83q2oXY2"
)

const rpc = createSolanaRpc('http://127.0.0.1:8899');
const rpcSubscriptions = createSolanaRpcSubscriptions('ws://127.0.0.1:8900');

async function main() {
    const payer = await generateKeyPairSigner();
    await rpc.requestAirdrop(payer.address, lamports(1_000_000_000n)).send();

    const { value: latestBlockHash } = await rpc.getLatestBlockhash().send();

    const deriveCounterAddress = PublicKey.findProgramAddressSync(
        [Buffer.from("counter"), new PublicKey(payer.address).toBytes()],
        ProgramId
    );

    const initIx = getInitializeInstruction({
        counter: address(deriveCounterAddress[0].toBase58()),
        payer,
        systemProgram: address(SystemProgram.programId.toBase58())
    });


    const transactionMessage = pipe(
        createTransactionMessage({ version: 0 }),
        tx => setTransactionMessageFeePayerSigner(payer, tx),
        tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockHash, tx),
        tx => appendTransactionMessageInstruction(initIx, tx),
    )

    const signedTransaction = await signAndSendTransactionMessageWithSigners(transactionMessage);
    console.log("Counter Initialized");

    console.log("Signature : ", signedTransaction.toString());

    //Update the transaction
    const updateIx = getIncrementCounterInstruction({
        counter: address(deriveCounterAddress[0].toBase58()),
        payer,
        input: 12
    });

    const updateTransactionMessage = pipe(
        createTransactionMessage({ version: 0 }),
        tx => setTransactionMessageFeePayerSigner(payer, tx),
        tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockHash, tx),
        tx => appendTransactionMessageInstruction(updateIx, tx)
    );
    const txnSign = await signTransactionMessageWithSigners(updateTransactionMessage);



    console.log(
        "Counter updated"
    );
    console.log("Signature : ", txnSign.toString());
}

main();