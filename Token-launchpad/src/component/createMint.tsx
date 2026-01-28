import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { Keypair, SystemProgram, Transaction} from "@solana/web3.js";
import { useState, type FC } from "react";

import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  getMinimumBalanceForRentExemptMint,
  createInitializeMintInstruction,
} from "@solana/spl-token";

export const CreateMint: FC = () => {
  const [txSig, setTxSig] = useState("");
  const [mint, setMint] = useState("");

  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();
  const link = () => {
    return txSig
      ? `https://explorer.solana.com/tx/${txSig}?cluster=devnet`
      : "";
  };

  const createMint = async (event : React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!connection || !publicKey) {
      return;
    }
    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    const accountKeyPair = Keypair.generate();
    
    const transaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey : publicKey,
            newAccountPubkey : accountKeyPair.publicKey,
            lamports : lamports,
            programId : TOKEN_PROGRAM_ID,
            space : MINT_SIZE
        }),
        createInitializeMintInstruction(
            accountKeyPair.publicKey,
            5,
            publicKey,
            null,
            TOKEN_PROGRAM_ID
        )
    );
    transaction.feePayer = publicKey;

    transaction.recentBlockhash = (
        await connection.getLatestBlockhash()
    ).blockhash;
    
    transaction.partialSign(accountKeyPair);
    const signature = await sendTransaction(transaction, connection);
    setTxSig(signature);
    setMint(accountKeyPair.publicKey.toBase58());
    return transaction;    
  };

  return (
    <div className="flex flex-col items-center gap-2">
      {publicKey ? (
        <form onSubmit={createMint} >
          <button type="submit" className="bg-teal-600 rounded-xl px-3 py-2 text-white hover:bg-teal-700 cursor-pointer">
            Create Mint
          </button>
        </form>
      ) : (
        <span>Connect Your Wallet</span>
      )}
      {txSig ? (
        <div className="shadow-2xl bg-slate-100 rounded-xl px-2 py-1 font-semibold text-xs">
          <p className="font-semibold text-xs">Token Mint Address: {mint}</p>
          <div className="flex items-center gap-2">
            <p>View your transaction on </p>
            <a href={link()} className="bg-pink-700 rounded-xl px-2 py-1 text-white hover:bg-pink-700">Solana Explorer</a>
          </div>
        </div>
      ) : null}
    </div>
  );
};