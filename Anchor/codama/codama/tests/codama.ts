import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Codama } from "../target/types/codama";

describe("codama", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.codama as Program<Codama>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
