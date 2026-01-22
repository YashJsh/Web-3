import { Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair, VersionedTransaction } from "@solana/web3.js";
import axios from "axios";
import bs58 from "bs58";
import dotenv from "dotenv"


dotenv.config();

const connection = new Connection('https://api.mainnet-beta.solana.com');
const wallet = new Wallet(Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY)));

const main = async () => {
    const quoteResponse = await axios.get(
        'https://api.jup.ag/swap/v1/quote?' +
        'inputMint=So11111111111111111111111111111111111111112' +
        '&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v' +
        '&amount=100000000' +
        '&slippageBps=50' +
        '&restrictIntermediateTokens=true'
        ,
        {
            headers: {
                'x-api-key': process.env.JUPITER_API_KEY,
            },
        }
    );
    console.log(JSON.stringify(quoteResponse.data, null, 2));

    try {
        const swapTransaction = await axios.post(
            "https://api.jup.ag/swap/v1/swap",
            {
                quoteResponse: quoteResponse.data,
                userPublicKey: wallet.publicKey.toString()
            },
            {
                headers: {
                    'x-api-key': process.env.JUPITER_API_KEY,
                },
            }
        );
        console.log(swapTransaction);
        const swap = swapTransaction.data.swapTransaction; // It returns a response as base 64
        console.log('swap transaction');
        const transaction = VersionedTransaction.deserialize(
            Buffer.from(swap, "base64")
        );
        transaction.sign([wallet.payer]);

        const latestBlockHash = await connection.getLatestBlockhash();

        //Execute transaction 
        const rawTransaction = transaction.serialize();
        const txid = await connection.sendRawTransaction(rawTransaction, {
            skipPreflight: true,
            maxRetries: 2
        });

        await connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: txid
        });

        console.log(`https://solscan.io/tx/${txid}`);
    } catch (error) {
        console.log(error);
    }
}

main();