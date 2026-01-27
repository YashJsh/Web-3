import { useState } from 'react';
import './App.css'
import { WalletDisconnectButton, WalletModalButton, WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Keypair, SystemProgram, Transaction } from '@solana/web3.js';
import { MINT_SIZE, TOKEN_PROGRAM_ID, createInitializeMint2Instruction, createMint, getMinimumBalanceForRentExemptMint } from "@solana/spl-token"

//https://petal-estimate-4e9.notion.site/Web-3-Cohort-d1b49c992dbf4648b185f974523d127c
function App() {
  const {connection} = useConnection();
  const wallet = useWallet();

  const [name, setName] = useState<string | null>(null);
  const [symbol, setSymbol] = useState<string | null>(null);
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const [supply, setSupply] = useState<string | null>(null);

  async function createToken(){
    if (!name || !symbol || !imageUrl || supply){
      alert("All fields are required");
      return;
    };
    if (!wallet){
      return;
    }
    const mintKeyPair = Keypair.generate();
    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey : wallet.publicKey!,
        lamports,
        newAccountPubkey : mintKeyPair.publicKey,
        programId : TOKEN_PROGRAM_ID,
        space : MINT_SIZE
      }),
      createInitializeMint2Instruction(mintKeyPair.publicKey, 9, wallet.publicKey!, wallet.publicKey, TOKEN_PROGRAM_ID)
    );
    transaction.feePayer = wallet.publicKey!;
    transaction.recentBlockhash = ((await connection.getLatestBlockhash()).blockhash);
    transaction.partialSign(mintKeyPair);
    await wallet.sendTransaction(transaction, connection);
    console.log(`Token mint created at ${mintKeyPair.publicKey.toBase58()}`);
  }

  return (
    <div className='h-screen w-screen items-center flex flex-col gap-2 justify-center'>
      <div className='inset-0 flex gap-2'>
          <WalletMultiButton/>
          <WalletDisconnectButton/>
      </div>
      <div className='flex flex-col gap-1 w-1/3 border rounded-xl px-3 py-5 justify-center shadow-2xl  text-center'>
        <h1 className='font-semibold text-xl mb-2'>Solana Token Launchpad</h1>
        <input className="border rounded-2xl px-2 py-1" type='text' placeholder='Name' onChange={(e)=>{
          setName(e.target.value);
        }}></input> <br />
        <input className="border rounded-2xl px-2 py-1" type='text' placeholder='Symbol'  onChange={(e)=>{
          setSymbol(e.target.value);
        }}></input> <br />
        <input className="border rounded-2xl px-2 py-1" type='text' placeholder='Image URL'  onChange={(e)=>{
          setImageUrl(e.target.value);
        }}></input> <br />
        <input className="border rounded-2xl px-2 py-1" type='text' placeholder='Initial Supply'  onChange={(e)=>{
          setSupply(e.target.value);
        }}></input> <br />
        <button className='roudned-xl bg-blue-600 text-white font-semibold px-2 py-1 rounded-2xl uppercase hover:bg-blue-700' onClick={createToken} >Create token</button>
      </div>
    </div>

  )
}

export default App
