import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction, TransactionInstruction, VersionedTransaction } from "@solana/web3.js";
import { expect, test } from "bun:test";
import { counter_size, schema } from "./types";
import * as borsh from 'borsh';

let counterAccountKeypair: Keypair;
let adminKeypair: Keypair;

test("Account Initialized", async () => {
    adminKeypair = Keypair.generate();
    counterAccountKeypair = new Keypair();

    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    // await connection.requestAirdrop(adminKeypair.publicKey, 2*LAMPORTS_PER_SOL);
    // console.log("Airdrop done");

    const data = await connection.getAccountInfo(adminKeypair.publicKey);
    console.log("data is : ", data);

    const programId = new PublicKey("HFKwi65ibRDVScWFyWM2yKvhj8Na2cGw4TyiLUWk4Hee");

    const lamports = await connection.getMinimumBalanceForRentExemption(counter_size);

    const createTransInst = SystemProgram.createAccount({
        fromPubkey: adminKeypair.publicKey,
        lamports: lamports,
        newAccountPubkey: counterAccountKeypair.publicKey,
        programId: programId,
        space: counter_size
    });
    let tx = new Transaction();
    tx.add(createTransInst)
    await sendAndConfirmTransaction(connection, tx, [adminKeypair, counterAccountKeypair,]);
    console.log("Pub Key of data account : ", counterAccountKeypair.publicKey.toBase58());

    const counterAccount = await connection.getAccountInfo(counterAccountKeypair.publicKey);
    console.log("Data Account Info : ", counterAccount);
    if (!counterAccount) {
        throw new Error("Account not found");
    }

    const counter = borsh.deserialize(schema, counterAccount?.data);
    console.log("Counter is : ", counter);
    //@ts-ignore
    console.log(counter?.count);
    //@ts-ignore
    expect(counter.count).toBe(0);
});

test("multiple Increment", async () => {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const programId = new PublicKey("HFKwi65ibRDVScWFyWM2yKvhj8Na2cGw4TyiLUWk4Hee");

    for (let i = 0; i < 5; i++) {
        const incrementIx = new TransactionInstruction({
            programId: programId,
            keys: [
                {
                    pubkey: counterAccountKeypair.publicKey,
                    isWritable: true,
                    isSigner: true
                }
            ],
            data: Buffer.from([1])
        });

        const tx = new Transaction().add(incrementIx);
        await sendAndConfirmTransaction(connection, tx, [adminKeypair]);
    }

    const counterAccount = await connection.getAccountInfo(counterAccountKeypair.publicKey);
    if (!counterAccount) throw new Error("Account not found");

    const counter = borsh.deserialize(schema, counterAccount.data);
    console.log(
        "Counter is : ", counter
    );
    //@ts-ignore
    expect(counter.count).toBe(6);
});


test("counter increment", async () => {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const programId = new PublicKey("HFKwi65ibRDVScWFyWM2yKvhj8Na2cGw4TyiLUWk4Hee");

    //Build increment instruction data
    const incrementIx = new TransactionInstruction({
        programId: programId,
        keys: [
            {
                pubkey: counterAccountKeypair.publicKey,
                isWritable: true,
                isSigner: true
            }
        ],
        data: Buffer.from([1])
    });

    const tx = new Transaction().add(incrementIx);
    await sendAndConfirmTransaction(connection, tx, [adminKeypair]);

    const counterAccount = await connection.getAccountInfo(counterAccountKeypair.publicKey);
    if (!counterAccount) throw new Error("Account not found");

    const counter = borsh.deserialize(schema, counterAccount.data);
    console.log(
        "Counter is : ", counter
    );
    //@ts-ignore
    expect(counter.count).toBe(1);
})