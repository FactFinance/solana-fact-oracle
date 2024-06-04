import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccount } from '@solana/spl-token';
import { Consumer } from "../target/types/consumer";
import { Oracle } from "../target/types/oracle";
import { Keypair, PublicKey } from "@solana/web3.js";
const {  SystemProgram, Transaction } = require('@solana/web3.js');
import fs from "mz/fs";
import path from "path";

async function getKey() {
  const PROGRAM_KEYPAIR_PATH = path.join(path.resolve(__dirname, "/home/juvinski/.config/solana/"), "id.json");  
  const secretKeyString = await fs.readFile(PROGRAM_KEYPAIR_PATH, { encoding: "utf8" });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const programKeypair = Keypair.fromSecretKey(secretKey);  
  return programKeypair;
}



describe("Fact Finance Oracle", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const consumer = anchor.workspace.Consumer as anchor.Program<Consumer>;  
  const oracle = anchor.workspace.Oracle as anchor.Program<Oracle>;

   
  const feedid1 = 2000;
  const feedid2 = feedid1+1;
  const new_value = 50000;
  const timestamp = Date.now() / 1000;
  const license = 0;
  const price=1_000_000_000;


  it("Initialize the oracle!", async () => {
      const wallet = await getKey();     
      let [settingsAccount, _3] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_settings")], oracle.programId);
      let [subscribersAccount, _1] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_subscribers")], oracle.programId);
      try {
        let settings = await oracle.account.settings.fetch(settingsAccount);          
      } catch (e) {
        try { 
        await oracle.methods
          .initializeOracle()           
          .accounts({
            settings: settingsAccount,
            subscribers: subscribersAccount
          })
          .signers([wallet])
          .rpc();
          } catch (e) {
            console.log('error creating setting pda:', e);
          }
      }
   
  });


  it("Initialize feed!", async () => {
    async function run(feedid: number) {
      const wallet = await getKey();
      let [datafeedAccount, _] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_datafeed"), Buffer.from(feedid.toString())], oracle.programId);
      let [auditordAccount, _2] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_auditor"), Buffer.from(feedid.toString())], oracle.programId);      

      try {
        let datafeed = await oracle.account.dataFeed.fetch(datafeedAccount);
        
      } catch (e) {
        
        try { 
        await oracle.methods
          .initializeDatafeed(feedid)
          .accounts({
            datafeed: datafeedAccount,            
            auditor: auditordAccount,            
            signer: provider.wallet.publicKey,
          })
          .signers([wallet])
          .rpc();
        } catch (e) {
          console.error(e)
        }
      }
    }
    await run(feedid1);
    await run(feedid2);
  });



  it("Set Value!", async () => {
    async function run(feedid: number) {
      const new_value = 500666943;

      const wallet = await getKey();            
      let [datafeedAccount, _] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_datafeed"), Buffer.from(feedid.toString())], oracle.programId);      
      let [auditordAccount, _2] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_auditor"), Buffer.from(feedid.toString())], oracle.programId);
      let [settingsAccount, _3] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_settings")], oracle.programId);
      await oracle.methods
        .setValue(new_value, timestamp, 'Bitcoin')
        .accounts({
          settings: settingsAccount,
          datafeed: datafeedAccount,
          auditor: auditordAccount,
        })        
        .rpc();
    }
    await run(feedid1);
    //await run(feedid2);
  });
  it("Set License!", async () => {
    async function run(feedid: number) {
      const wallet = await getKey();
      let [datafeedAccount, _] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_datafeed"), Buffer.from(feedid.toString())], oracle.programId);      
      let [settingsAccount, _3] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_settings")], oracle.programId);
      await oracle.methods
        .setLicense(license)
        .accounts({
          settings: settingsAccount,
          datafeed: datafeedAccount,
        })
        .rpc();
    }
    await run(feedid1);
    await run(feedid2);
  });



  it("Add subscription!", async () => {
    async function run(feedid: number) {
      const wallet = await getKey();
      let [datafeedAccount, _] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_datafeed"), Buffer.from(feedid.toString())], oracle.programId);
      let [subscribersAccount, _1] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_subscribers")], oracle.programId);
      let [settingsAccount, _3] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_settings")], oracle.programId);
	    const pubkeyStr = "4SaWY3ErtEoh9ixRQnhzBNKC5CzuTyZmDoEZhtNXriSD";
	    const pubkeymac = new PublicKey(pubkeyStr);
      await oracle.methods
        .addSubscription(wallet.publicKey)
        .accounts({
          settings: settingsAccount,          
          subscribers: subscribersAccount,
        })
        .rpc();
    }
    await run(feedid1);
    await run(feedid2);
  });

  it("Pull the oracle!", async () => {
    async function run(feedid: number) {
      const wallet = await getKey();
            
      let [datafeedAccount, _] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_datafeed"), Buffer.from(feedid.toString())], oracle.programId);
      let [subscribersAccount, _1] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_subscribers")], oracle.programId);
      await consumer.methods
        .pullOracle()
        .accounts({
          datafeed: datafeedAccount,
          subscribers: subscribersAccount,
          oracleProgram: oracle.programId,
        })      
        .rpc();
    }
    await run(feedid1);
   //await run(feedid2);
  });

  it("Revoke subscription!", async () => {
    const wallet = await getKey();

    async function run(feedid: number) {
      const wallet = await getKey();
      let [datafeedAccount, _] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_"), Buffer.from(feedid.toString())], oracle.programId);
      let [subscribersAccount, _1] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_subscribers")], oracle.programId);
      let [settingsAccount, _3] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_settings")], oracle.programId);
      await oracle.methods
        .revokeSubscription(wallet.publicKey)
        .accounts({
          settings: settingsAccount,          
          subscribers: subscribersAccount,
        })
        .rpc();
    }
    await run(feedid1);
    await run(feedid2);
  });


  it("Subscript with bonk!", async () => {
    const wallet = await getKey();

    const senderTokenAccount = await createAssociatedTokenAccount(
      provider.connection,
      wallet,
      new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),  // bonk program address
      provider.wallet.publicKey
  );

    async function run(feedid: number) {
      const wallet = await getKey();
      let [datafeedAccount, _] = await anchor.web3.PublicKey.findProgramAddress([wallet.publicKey.toBuffer(), Buffer.from("_"), Buffer.from(feedid.toString())], oracle.programId);
      let [subscribersAccount, _1] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_subscribers")], oracle.programId);
      let [settingsAccount, _3] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("_settings")], oracle.programId);
      await oracle.methods
        .subscribe(wallet.publicKey)
        .accounts({         
         sender:senderTokenAccount,
         receiver: senderTokenAccount,
         settings:settingsAccount,
         subscribers:subscribersAccount,
         token: TOKEN_PROGRAM_ID,
        })
        .rpc();
    }
    await run(feedid1);
    await run(feedid2);
  });



});
