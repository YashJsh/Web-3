
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JackpotProgram } from "../target/types/jackpot_program";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("jackpot-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JackpotProgram as Program<JackpotProgram>;

  //Initializing pda
  const [masterKey] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("master_key")],
    program.programId
  );

  const [treasury] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("treasury")],
    program.programId
  );


  it("Initializes auction", async () => {
    const duration = new anchor.BN(5);
    await program.methods.initializeAccount(duration)
      .accounts({
        signer: provider.wallet.publicKey,
        //@ts-ignore
        masterKey: masterKey,
        treasury,
        systemProgram: SystemProgram.programId
      })
      .rpc();

    const account = await program.account.masterKey.fetch(masterKey);
    expect(account.highestBid.toNumber()).to.equal(0);
    expect(account.highestBidder.toBase58()).to.equal(
      PublicKey.default.toBase58()
    );
  });

  it("Allows valid bid", async () => {
    // call bid
    let bidAmount = anchor.web3.LAMPORTS_PER_SOL / 10;
    await program.methods
      .bid(new anchor.BN(bidAmount))
      .accounts({
        signer: provider.wallet.publicKey,
        //@ts-ignore
        masterKey,
        treasury,
        systemProgram: SystemProgram.programId
      })
      .rpc()
    // fetch state
    const account = await program.account.masterKey.fetch(masterKey);

    // check highest_bid
    expect(account.highestBidder.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    expect(account.highestBid.toNumber()).to.equal(bidAmount);
  });

  it("Rejects low bid", async () => {

    try {
      await program.methods
        .bid(new anchor.BN(1))
        .accounts({
          signer: provider.wallet.publicKey,
          //@ts-ignore
          masterKey,
          treasury,
          systemProgram: SystemProgram.programId
        })
        .rpc()
      expect.fail("Should have failed")
    } catch (error) {
      expect(error).to.exist;
    }
  });

  it("Settles correctly", async () => {

    await new Promise((resolve) => setTimeout(resolve, 6000));

    await program.methods.settle().accounts({
      //@ts-ignore
      masterKey,
      treasury,
      auctioneer: new PublicKey("6KpVFh4ehrWZoWNaNWWsd4MitZ9axhybgS6CXdNiUP1V"),
      highestBidder: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    }).rpc()

    expect(true).to.be.true;
  });
});
