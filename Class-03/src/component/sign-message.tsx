import { useWallet } from "@solana/wallet-adapter-react";
import { ed25519 } from '@noble/curves/ed25519.js';
import bs58  from 'bs58';
import { useState } from "react";

export const SignMessage = () => {
    const { signMessage, publicKey} = useWallet();
    const [message, setMessage] = useState("");
    
    const sign = async ()=>{
        if(!signMessage || !publicKey)return;

        if (message == ""){
            alert("Message is empty")
        };

        const encodeMessage = new TextEncoder().encode(message);
        const signature = await signMessage(encodeMessage);

        const ok = ed25519.verify(signature, encodeMessage, publicKey.toBytes());
        if (!ok) throw new Error("Invalid signature");

        alert(`Success!\nMessage Signature: ${bs58.encode(signature)}`);
    }


    return (
        <div>
            <input type="text" id = "message" placeholder="message" onChange={(e)=>{
                setMessage(e.target.value)
            }}/>
            <button onClick={sign}>Sign Message</button>
        </div>
    )
}