import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorHelloWorld } from '../target/types/anchor_hello_world';
import { SystemProgram } from "@solana/web3.js";
import assert from "assert";

describe('anchor-hello-world', () => {

  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const counter = anchor.web3.Keypair.generate();

  const program = anchor.workspace.AnchorHelloWorld as Program<AnchorHelloWorld>;

  it("Initialize Account)", async () => {
    await program.rpc.initializean(provider.wallet.publicKey, {
      accounts: {
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [counter],
    });

    let counterAccount = await program.account.counter.fetch(counter.publicKey);

    assert.ok(counterAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(counterAccount.count.toNumber() === 0);
  });

  it("Say Hello", async () => {
    await program.rpc.increment({
      accounts: {
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
      },
    });

    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    assert.ok(counterAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(counterAccount.count.toNumber() == 1);
  });

  // it("Close Account)", async () => {
  //   await program.rpc.closeaccount(provider.wallet.publicKey, {
  //     accounts: {
  //       counter: counter.publicKey,
  //       user: provider.wallet.publicKey,
  //       systemProgram: SystemProgram.programId,
  //     },
  //     signers: [counter],
  //   });

  //   let counterAccount = await program.account.counter.fetch(counter.publicKey);

  //   assert.ok(counterAccount.authority.equals(provider.wallet.publicKey));
  //   assert.ok(counterAccount.count.toNumber() === 0);
  // });

});
