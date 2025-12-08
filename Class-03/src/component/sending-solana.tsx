import { useConnection, useWallet } from "@solana/wallet-adapter-react"
import { SystemProgram, Transaction, PublicKey, LAMPORTS_PER_SOL} from "@solana/web3.js";
import { useState } from "react";

export const SendSolana = ()=>{
    const {sendTransaction , publicKey, } = useWallet();
    const { connection } = useConnection();
    const [pblKey, setPblKey] = useState<string|null>(null);
    const [amount, setAmount] = useState<number>(0);

    const sendSol = async ()=>{
        if (!pblKey || amount == 0){
            return;
        }
        if (!sendTransaction || !publicKey){
            return;
        }
        const balance = await connection.getBalance(publicKey);
        const bal = balance / LAMPORTS_PER_SOL

        if(bal < amount){
            alert("You can't send more than you actually have");
            return;
        }
        try {
            const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
            const transaction = new Transaction({
                blockhash,
                lastValidBlockHeight,
                feePayer : publicKey
            });
            transaction.add(SystemProgram.transfer({
                fromPubkey : publicKey,
                toPubkey : new PublicKey(pblKey),
                lamports : amount * LAMPORTS_PER_SOL
             }));
            await sendTransaction(transaction, connection);
            alert(`Sent ${amount}` + "SOL to " + pblKey);
        } catch (error) {
            console.error(error);
            alert(error);
        }
    }   
    
    return (
        <div>
            <input type="text" placeholder="To" onChange={(e)=>{setPblKey(e.target.value)}} />
            <input type="number" placeholder="Amount" onChange={(e)=>{setAmount(e.target.valueAsNumber)}} />
            <button onClick={sendSol}>Send Transaction</button>
        </div>
    )
}