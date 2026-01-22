import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { WalletDisconnectButton } from "@solana/wallet-adapter-react-ui";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { useState } from "react";

export const RequestAirdrop = () => {
    const [balance, setbalance] = useState(0);
    const wallet = useWallet();
    const connection = useConnection();

    const request = async  () => {
        try {
            let inp = document.getElementById("amount") as HTMLInputElement;
            let amount = Number(inp.value);
            const response = await connection.connection.requestAirdrop(wallet.publicKey!, amount * LAMPORTS_PER_SOL);
            console.log(response);
            alert(`Sol send for ${amount} in public key ${wallet.publicKey}`);
        } catch (error) {
            console.warn(error);
        }
        
    }   
    
    const getBalance  = async ()=>{
        const balance = await connection.connection.getBalance(wallet.publicKey!);
        setbalance(balance / LAMPORTS_PER_SOL);
    };

    return (
        <div>
            <input id="amount" type="text" placeholder="Amount" />
            <button onClick = {()=>{
                request();      
            }}>Request Airdrop</button>
            <button onClick = {()=>{
                getBalance();
            }}>Get Balance</button>
            <p>Balance : {balance} sol</p>
            <div>
                <WalletDisconnectButton />
            </div>
        </div>
    )
}