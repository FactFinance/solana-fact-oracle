import * as anchor from "@project-serum/anchor";
import { Oracle } from "../target/types/oracle";

import * as BufferLayout from "@solana/buffer-layout";
import { Buffer } from "buffer";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";

describe("Fact Finance Oracle", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Oracle as anchor.Program<Oracle>;
  let keypair = anchor.web3.Keypair.generate();

  async function generateKeypair() {
    await provider.connection.requestAirdrop(keypair.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await new Promise((resolve) => setTimeout(resolve, 3 * 1000)); // Sleep 3s
    return keypair;
  }

  async function derivePda(feedid: number, pubkey: anchor.web3.PublicKey) {
    let [pda, _] = await anchor.web3.PublicKey.findProgramAddress([pubkey.toBuffer(), Buffer.from("_"), Buffer.from(feedid.toString())], program.programId);
    return pda;
  }

  async function createLedgerAccount(feedid: number, pda: anchor.web3.PublicKey, wallet: anchor.web3.Keypair) {
    await program.methods
      .createLedger(feedid)
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
  }

  async function modifyLedger(feedid: number, new_value: number, wallet: anchor.web3.Keypair) {
    let data;
    let pda = await derivePda(feedid, wallet.publicKey);

    try {
      data = await program.account.ledger.fetch(pda);
    } catch (e) {
      await createLedgerAccount(feedid, pda, wallet);
      data = await program.account.ledger.fetch(pda);
    }

    const timestamp = Math.floor(Date.now() / 1000);


    await program.methods
      .setValue(new_value, timestamp)
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    await program.methods
      .getValue()
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    await program.methods
      .setLicense(1)
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    await program.methods
      .getValue()
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    await program.methods
      .addSubscription(wallet.publicKey)
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    await program.methods
      .getValue()
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    await program.methods
      .revokeSubscription(wallet.publicKey)
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();

    data = await program.account.ledger.fetch(pda);
  }

  it("Create feed ledge, set value, timestamp and license", async () => {
    const testKeypair1 = await generateKeypair();
    await modifyLedger(123, 2, testKeypair1);
    // await modifyLedger(233, 3, 300, testKeypair1);
  });
});
