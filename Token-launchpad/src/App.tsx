import { useState } from 'react';
import './App.css'
import { Navbar } from './component/navbar';
import { BalanceDisplay } from './component/balanceForm';
import { CreateMint } from './component/createMint';
import { CreateTokenAccount } from './component/createTokenAccount';
import { MintToken } from './component/mintToken';

function App() {
  const [name, setName] = useState<string | null>(null);
  const [symbol, setSymbol] = useState<string | null>(null);
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const [supply, setSupply] = useState<string | null>(null);

  async function createToken() {
    if (!name || !symbol || !imageUrl || !supply) {
      alert("All fields are required");
      return;
    };
  }

  return (
    <div className='h-screen w-screen'>
      <Navbar />
      <div className='flex flex-col items-center justify-center gap-2'>
          <BalanceDisplay/>
          <CreateMint/>
          <CreateTokenAccount/>
          <MintToken/>
      </div>
    </div>
  )
}

export default App


//https://petal-estimate-4e9.notion.site/Web-3-Cohort-d1b49c992dbf4648b185f974523d127c
