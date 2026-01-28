import { WalletDisconnectButton, WalletMultiButton } from "@solana/wallet-adapter-react-ui"

export const Navbar = ()=>{
    return (
        <div className="container mx-auto p-4 bg-slate-100 rounded-xl">
            <div className="flex justify-between items-center text-center">
                <img src = {"https://static.vecteezy.com/system/resources/previews/044/626/788/non_2x/solana-logo-on-transparent-background-free-vector.jpg"} height={50} width={50}/>
                <span className="font-semibold uppercase tracking-tighter">Token Program</span>
                <div className="flex gap-2">
                     <WalletMultiButton />
                    <WalletDisconnectButton />
                </div>
            </div>
        </div>
    )
}