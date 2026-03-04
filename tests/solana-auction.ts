import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { SystemProgram } from "@solana/web3.js";
import { SolanaAuction } from "../target/types/solana_auction";

describe("solana-auction", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solanaAuction as Program<SolanaAuction>;

  it("Initializes auction", async () => {
    const [auctionPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), auctionPda.toBuffer()],
      program.programId
    );

    const now = Math.floor(Date.now() / 1000);
    const tx = await program.methods
      .initializeAuction(new anchor.BN(now + 30), new anchor.BN(now + 3600), new anchor.BN(1_000_000))
      .accounts({
        seller: provider.wallet.publicKey,
        auction: auctionPda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
