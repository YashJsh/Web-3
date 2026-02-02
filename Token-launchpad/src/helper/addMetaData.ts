import {
	createV1,
	findMetadataPda,
	mplTokenMetadata,
	TokenStandard
} from "@metaplex-foundation/mpl-token-metadata";
import { mplToolbox } from "@metaplex-foundation/mpl-toolbox";
import {
  percentAmount,
  publicKey,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { type WalletContextState } from "@solana/wallet-adapter-react";


export const addMetadataToken = async (wallet :  WalletContextState,tokenMintAddress : string, name : string, symbol : string, uri : string)=>{
    const umi = createUmi("https://api.devnet.solana.com")
	.use(mplTokenMetadata())
	.use(mplToolbox());

    umi.use(walletAdapterIdentity(wallet));

    const tokenMetadata = {
        name : name,
        symbol : symbol,
        uri : uri
    };

    const metadataAccountAddress = await findMetadataPda(umi, {
		mint: publicKey(tokenMintAddress),
	});

    if (metadataAccountAddress){
        console.log("MetaData account already exists");
    }

    const tx = await createV1(umi, {
        mint : publicKey(tokenMintAddress),
        authority: umi.identity,
		payer: umi.identity,
		updateAuthority: umi.identity,
		name: tokenMetadata.name,
		symbol: tokenMetadata.symbol,
		uri: tokenMetadata.uri,
		sellerFeeBasisPoints: percentAmount(0),
		tokenStandard: TokenStandard.Fungible,
    }).sendAndConfirm(umi);

    let txSig = base58.deserialize(tx.signature);
	console.log(`https://explorer.solana.com/tx/${txSig}?cluster=devnet`)
    const str = `https://explorer.solana.com/tx/${txSig}?cluster=devnet`;
    return str;
}