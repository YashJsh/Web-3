import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { useEffect, useState } from "react";

export const BalanceDisplay = ()=>{
    const [balance, setBalance] = useState<number>(0);
    const {publicKey} = useWallet();
    const connection = useConnection();
    useEffect(()=>{
        if (!connection || !publicKey){
            return;
        }
        connection.connection.getAccountInfo(publicKey).then((e)=>{
            setBalance(e?.lamports!);
        });
    }, [connection, publicKey]);
    return (
        <div className="flex items-center gap-2 justify-center bg-purple-800 rounded-xl p-3 text-white uppercase mt-2">
            <h1 className="font-semibold text-xl">Balance : </h1>
            <span className="font-xs">{balance/LAMPORTS_PER_SOL}</span>
        </div>
    )
};
