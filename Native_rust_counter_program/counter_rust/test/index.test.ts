import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction, VersionedTransaction } from "@solana/web3.js";
import {expect, test} from "bun:test";
import { counter_size, schema } from "./types";
import * as borsh from 'borsh';

let counterAccountKeypair : Keypair;
let adminKeypair : Keypair;

test("Account Initialized", async ()=>{
    adminKeypair = Keypair.generate();
    counterAccountKeypair = new Keypair();

    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    // await connection.requestAirdrop(adminKeypair.publicKey, 2*LAMPORTS_PER_SOL);
    // console.log("Airdrop done");

    const data = await connection.getAccountInfo(adminKeypair.publicKey);
    console.log("data is : ",data);

    const programId = new PublicKey("HFKwi65ibRDVScWFyWM2yKvhj8Na2cGw4TyiLUWk4Hee");

    const lamports = await connection.getMinimumBalanceForRentExemption(counter_size);

    const createTransInst = SystemProgram.createAccount({
        fromPubkey : adminKeypair.publicKey,
        lamports : lamports,
        newAccountPubkey : counterAccountKeypair.publicKey,
        programId : programId,
        space : counter_size
    });
    let tx = new Transaction();
    tx.add(createTransInst)
    await sendAndConfirmTransaction(connection, tx, [adminKeypair, counterAccountKeypair,]);
    console.log("Pub Key of data account : ",counterAccountKeypair.publicKey.toBase58());

    const counterAccount = await connection.getAccountInfo(counterAccountKeypair.publicKey);
    console.log("Data Account Info : ", counterAccount);
    if (!counterAccount){
        throw new Error("Account not found");
    }

    const counter = borsh.deserialize(schema, counterAccount?.data);
    console.log("Counter is : ", counter);
    console.log(counter?.count);
    expect(counter.count).toBe(0);
});