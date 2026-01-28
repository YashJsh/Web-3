import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL, PublicKey, Transaction } from "@solana/web3.js";
import { type FC, useState } from "react";
import {
    createMintToInstruction,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAccount,
    getMint,
} from "@solana/spl-token";

export const MintToken: FC = () => {
    const [txSig, setTxSig] = useState("");
    const [balance, setBalance] = useState<number | null>();
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();
    const link = () => {
        return txSig
            ? `https://explorer.solana.com/tx/${txSig}?cluster=devnet`
            : "";
    };


    const mintTo = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        if (!connection || !publicKey) {
            return;
        }
        const form = event.currentTarget;
        const formData = new FormData(form);
        const tokenMint = formData.get("mint") as string;
        const tokenAccount = formData.get("recipient") as string;
        const tokenAccountPubKey = new PublicKey(tokenAccount);
        const mintInfo = await getMint(connection, new PublicKey(tokenMint));
        const decimals = mintInfo.decimals;

        const amount = Number(formData.get("amount") as string);
        const rawAmount = amount * Math.pow(10, decimals);

        const transaction = new Transaction().add(
            createMintToInstruction(
                new PublicKey(tokenMint),
                tokenAccountPubKey,
                publicKey,
                rawAmount
            )
        );
        transaction.feePayer = publicKey;
        transaction.recentBlockhash = (
            await connection.getLatestBlockhash()
        ).blockhash;

        const signature = await sendTransaction(transaction, connection);
        setTxSig(signature);

        const balanceResponse = await connection.getTokenAccountBalance(tokenAccountPubKey);
        setBalance(balanceResponse.value.uiAmount || 0);
    };

    return (
        <div className="w-1/2 ">
            {publicKey ? (
                <form onSubmit={mintTo} className="flex flex-col gap-2  border-1 border-slate-500 rounded-xl px-3 py-2">
                    <div className="flex flex-col">
                        <label htmlFor="mint">Token Mint:</label>
                        <input
                            id="mint"
                            name="mint"
                            type="text"
                            placeholder="Enter Token Mint"
                            required
                            className="border rounded-xl px-2 py-1"
                        />
                    </div>
                    <div className="flex flex-col">
                        <label htmlFor="recipient">Recipient:</label>
                        <input
                            id="recipient"
                            name="recipient"
                            type="text"
                            placeholder="Enter Recipient PublicKey"
                            required
                            className="border rounded-xl px-2 py-1"
                        />
                    </div>
                    <div className="flex flex-col">
                        <label htmlFor="amount">Amount Tokens to Mint:</label>
                        <input
                            id="amount"
                            name="amount"
                            type="text"
                            placeholder="e.g. 100"
                            required
                            className="border rounded-xl px-2 py-1"
                        />
                    </div>
                    <button type="submit" className="bg-teal-600 rounded-xl px-3 py-2 text-white hover:bg-teal-700 cursor-pointer">
                        Mint Tokens
                    </button>
                </form>
            ) : (
                <span></span>
            )}
            {txSig ? (
                <div className="shadow-2xl bg-slate-100 rounded-xl px-2 py-1 font-semibold text-xs mt-2">
                    <p>Token Balance: {balance} </p>
                    <div className="flex gap-2">
                        <p>View your transaction on :</p>
                        <a href={link()} className="cursor-pointer">Solana Explorer</a>
                    </div>
                </div>
            ) : null}
        </div>
    );
};