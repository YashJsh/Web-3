import { useConnection, useWallet } from "@solana/wallet-adapter-react";

import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { type FC, useState } from "react";

import {
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    getMint,
    getAccountLenForMint,
    createInitializeAccountInstruction,
} from "@solana/spl-token";

export const CreateTokenAccount: FC = () => {
    const [txSig, setTxSig] = useState("");
    const [tokenAccount, setTokenAccount] = useState("");
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();
    const link = () => {
        return txSig
            ? `https://explorer.solana.com/tx/${txSig}?cluster=devnet`
            : "";
    };

    const createTokenAccount = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        if (!connection || !publicKey) {
            return;
        };
        const form = event.currentTarget;
        const formData = new FormData(form);
        const tokenMint = formData.get("mint") as string;
        const owner = formData.get("owner") as string;
        const ownerPublicKey = new PublicKey(owner);

        //Get the data associated with the mint;
        const mintState = await getMint(connection, new PublicKey(tokenMint));
        const accountKeyPair = await Keypair.generate();
        const space = getAccountLenForMint(mintState);
        const lamports = await connection.getMinimumBalanceForRentExemption(space);

        const transaction = new Transaction().add(
            SystemProgram.createAccount(
                {
                    fromPubkey: publicKey,
                    lamports: lamports,
                    programId: TOKEN_PROGRAM_ID,
                    newAccountPubkey: accountKeyPair.publicKey,
                    space: space
                }
            ),
            createInitializeAccountInstruction(
                accountKeyPair.publicKey,
                new PublicKey(tokenMint),
                ownerPublicKey,
                TOKEN_PROGRAM_ID
            ),
        );
        transaction.feePayer = publicKey;

        transaction.recentBlockhash = (
            await connection.getLatestBlockhash()
        ).blockhash;
        transaction.partialSign(accountKeyPair);
        const signature = await sendTransaction(transaction, connection);
        setTxSig(signature);
        setTokenAccount(accountKeyPair.publicKey.toBase58());
    };

    return (
        <div className="flex flex-col w-1/2">
            {publicKey ? (
                <form onSubmit={createTokenAccount} className="flex flex-col px-2 py-1 rounded-xl gap-2 border-1 border-slate-500">
                    <div className="flex flex-col">
                        <label htmlFor="owner" className="">Token Mint:</label>
                        <input
                            name="mint" 
                            id="mint" 
                            type="text"
                            placeholder="Enter Token Mint"
                            required
                            className="border rounded-xl px-2 py-1"
                        />
                    </div>

                    <div className="flex flex-col">
                        <label htmlFor="owner">Token Account Owner:</label>
                        <input
                            name="owner" 
                            id="owner"
                            type="text"
                            placeholder="Enter Token Account Owner PublicKey"
                            required
                            className="border rounded-xl px-2 py-1"
                        />
                    </div>


                    <button type="submit" className="bg-teal-600 rounded-xl px-3 py-2 text-white hover:bg-teal-700 cursor-pointer">
                        Create Token Account
                    </button>
                </form>
            ) : (
                <span></span>
            )}
            {txSig ? (
                <div className="shadow-2xl bg-slate-100 rounded-xl px-2 py-1 font-semibold text-xs">
                    <p>Token Account Address: {tokenAccount}</p>
                    <div className="flex gap-2">
                        <p>View your transaction on :</p>
                        <a href={link()} className="cursor-pointer">Solana Explorer</a>
                    </div>
                </div>
            ) : null}
        </div>
    );
};