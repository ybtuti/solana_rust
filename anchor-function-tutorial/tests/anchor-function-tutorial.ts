import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorFunctionTutorial } from "../target/types/anchor_function_tutorial";

describe("anchor-function-tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorFunctionTutorial as Program<AnchorFunctionTutorial>;

  it("Calls boaty mcboatface", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should add", async () => {
    const tx = await program.methods.add(new anchor.BN(10), new anchor.BN(4)).rpc();
    console.log("Addition transaction signature is", tx);    
  });

  it("Should subtract", async () => {
    const tx = await program.methods.subtract(new anchor.BN(30), new anchor.BN(10)).rpc();
    console.log("Subtraction transaction signature is", tx);
  })
});
