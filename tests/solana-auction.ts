import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { SolanaAuction } from "../target/types/solana_auction";

describe("solana-auction", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solanaAuction as Program<SolanaAuction>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
