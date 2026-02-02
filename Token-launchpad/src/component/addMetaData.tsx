import { useWallet } from "@solana/wallet-adapter-react";
import { useState } from "react";
import { addMetadataToken } from "../helper/addMetaData";

export const AddData = () => {
    const wallet = useWallet();
    const [name, setName] = useState("");
    const [symbol, setSymbol] = useState("");
    const [image, setImageUrl] = useState("");
    const [tokenMint, setTokenMint] = useState("");
    const [txSig, setTxSig] = useState("");

    const addMetaData = async () => {
        if (
            !name.trim() ||
            !symbol.trim() ||
            !image.trim() ||
            tokenMint.length === 0
        ) {
            return;
        }
        const txSignature = await addMetadataToken(wallet, tokenMint, name, symbol, image);
        setTxSig(txSignature);
    };

    return (
        <div className='flex flex-col gap-1 w-1/3 border rounded-xl px-3 py-5 justify-center shadow-2xl  text-center'>
            <h1 className='font-semibold text-xl mb-2'>Solana Add Metadata</h1>
            <input className="border rounded-2xl px-2 py-1" type='text' placeholder='Name' onChange={(e) => {
                setName(e.target.value);
            }}></input> <br />
            <input className="border rounded-2xl px-2 py-1" type='text' placeholder='Symbol' onChange={(e) => {
                setSymbol(e.target.value);
            }}></input> <br />
            <input className="border rounded-2xl px-2 py-1" type='text' placeholder='Image URL' onChange={(e) => {
                setImageUrl(e.target.value);
            }}></input> <br />
            <input
                name="mint"
                id="mint"
                type="text"
                placeholder="Enter Token Mint"
                required
                className="border rounded-xl px-2 py-1"
                onChange={(e) => {
                    setTokenMint(e.target.value);
                }}
            />
            <button className='roudned-xl bg-blue-600 text-white font-semibold px-2 py-1 rounded-2xl uppercase hover:bg-blue-700' onClick={addMetaData} >Add Meta Data</button>
            {txSig ? (
                <div className="shadow-2xl bg-slate-100 rounded-xl px-2 py-1 font-semibold text-xs">
                    <div className="flex gap-2">
                        <p>View your transaction on :</p>
                        <a href={txSig} className="cursor-pointer">Solana Explorer</a>
                    </div>
                </div>
            ) : null}
        </div>
    )
};