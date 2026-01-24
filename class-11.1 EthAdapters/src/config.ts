import { createConfig, http, injected } from 'wagmi'
import { mainnet, base  } from 'wagmi/chains'
// import { walletConnect } from 'wagmi/connectors'

// const projectId = '<WALLETCONNECT_PROJECT_ID>'

export const config = createConfig({
  chains: [mainnet, base],
    connectors: [
    injected(),
  ],
  transports: {
    [mainnet.id]: http(),
    [base.id]: http(),
  },
})