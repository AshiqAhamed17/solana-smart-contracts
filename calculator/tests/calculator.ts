import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import * as assert from "assert";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const newAccount = anchor.web3.Keypair.generate();

  const program = anchor.workspace.calculator as Program<Calculator>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .init(7)
    .accounts({
      signer: anchor.getProvider().wallet.publicKey,
      account: newAccount.publicKey
    })
    .signers([newAccount])
    .rpc();

    console.log("Your transaction: ", tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.strictEqual(account.num, 7);
  });

it("Is Double!", async () => {
    // Add your test here.
    const tx = await program.methods
    .double()
    .accounts({
      signer: anchor.getProvider().wallet.publicKey,
      account: newAccount.publicKey
    })
    .rpc();

    console.log("Your transaction: ", tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.strictEqual(account.num, 14);

  });

});
