
import './App.css'
import { RequestAirdrop } from './component/request-airdrop'
import { SendSolana } from './component/sending-solana'
import { SignMessage } from './component/sign-message'

function App() {
  return (
    <div style={{
      display : 'flex',
      flexDirection : 'column',
      gap : "1rem"
    }}>
     <h1>Request Airdrop</h1>
     <RequestAirdrop/>
     <SignMessage/>
     <SendSolana/>
    </div>
  )
}

export default App
