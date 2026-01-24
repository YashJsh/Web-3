
import './App.css'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useAccount, useBalance, useConnect, useConnectors, useSendTransaction, WagmiProvider } from 'wagmi'
import { config } from "./config"
import { useState } from 'react'
import { parseEther } from 'viem'

const queryClient = new QueryClient()

function App() {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
          <WalletConnector/>
          <SendEth/>
          <MyData/>
      </QueryClientProvider>
    </WagmiProvider>
  )
}

function WalletConnector() {
  const { connect, isPending} = useConnect();
  const connectors = useConnectors();
  return connectors.map((connector)=>(
    <button key={connector.uid} onClick={() => connect({ connector })}>
      {isPending ? "Connecting..." : connector.name}
    </button>
  ))
}

function MyData(){
  const { address,  } = useAccount();
  if (!address){
    return <div>Please connect your wallet</div>
  }
  const balance = useBalance({address});
  return <div>
    <h2>Account Address :   {address}</h2>
    <h3>Balance : {balance?.data?.value ? balance.data.value : 0}</h3>
  </div>
}

function SendEth() {
  const [money, setMoney] = useState("");
  const [address, setAddress] = useState("");

  const { data: hash, sendTransaction } = useSendTransaction();

  const handleSendTransaction = () => {
    const to = address as `0x${string}`;
    sendTransaction({
      to,
      value: parseEther(money),
    });
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "10px" }}>
      <h1>Send the money to other</h1>

      <input
        placeholder="Enter the money"
        type="number"
        onChange={(e) => setMoney(e.target.value)}
      />

      <input
        placeholder="Enter Address"
        type="text"
        onChange={(e) => setAddress(e.target.value)}
      />

      <button onClick={handleSendTransaction}>Send Eth</button>
    </div>
  );
}


export default App;

