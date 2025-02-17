import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Casino } from "../target/types/casino";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from 'chai';
import BN from 'bn.js';
describe("casino", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Casino as Program<Casino>;
  const userWallet =anchor.web3.Keypair.generate();
  const ow=anchor.web3.Keypair.generate();
  const ownerWallet = new PublicKey("H1V3XkxhGuADph1ajAWmTjwUcY6Y8EVX3PfXosdsP2JM");

   
  let vaultPDA: PublicKey;
  let vaultBump: number;

  before(async () => {
    // Airdrop SOL to the user wallet for testing
    const signature = await provider.connection.requestAirdrop(
      userWallet.publicKey,
      10 * LAMPORTS_PER_SOL // 2 SOL
    );
    console.log("The Signature is", signature);
    await provider.connection.confirmTransaction(signature);

    // Find the vault PDA
   
  });


  console.log("Your public key", userWallet.publicKey.toBase58());




  it("Is  Initialize time !", async () => { 

    const user = userWallet.publicKey;
    const vault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tluav"), userWallet.publicKey.toBuffer()],
      program.programId
    )[0];
    const vault_Account_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), userWallet.publicKey.toBuffer()],
      program.programId
    )[0];


  

    try{
    const tx1 = await program.methods.initialize().accountsStrict({
      user: user,
      vault: vault,
      vaultAccount: vault_Account_pda,
      systemProgram: SystemProgram.programId,
    }).signers([userWallet]).rpc()

    console.log("The Transaction is", tx1); 

  

    }catch(e){
      console.error(e);

    }
  });


  it("Is  Bet time !", async () => {

    const user = userWallet.publicKey;
    const vault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tluav"), userWallet.publicKey.toBuffer()],
      program.programId
    )[0];
    const vault_Account_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), userWallet.publicKey.toBuffer()],
      program.programId
    )[0];
   

    try {

      const balance = await provider.connection.getBalance(userWallet.publicKey);
      console.log("The Balance is", balance);

     const tx= await   program.methods.bet(new BN(2)).accountsStrict({
        user: user,
        vault: vault,
        vaultAccount: vault_Account_pda,
        systemProgram: SystemProgram.programId,
       }).signers([userWallet]).rpc()

       const balance2 = await provider.connection.getBalance(userWallet.publicKey);
       console.log("The Balance is", balance2);
       const balance3 = await provider.connection.getBalance(vault_Account_pda);
       console.log("The Balance of Pda vault is ", balance3);
    }  catch(e){
      console.error(e);
    }
 })



  it("Is  Betprocess time !", async () => {
 const user = userWallet.publicKey;
 const vault = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("tluav"), userWallet.publicKey.toBuffer()],
  program.programId
)[0];
const vault_Account_pda = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), userWallet.publicKey.toBuffer()],
  program.programId
)[0];
try {
  const tx= await program.methods.betprocess(6).accountsStrict({
    user:user,
    vault: vault,
    vaultAccount: vault_Account_pda,
    systemProgram: SystemProgram.programId,
    ownerAccount: ow.publicKey,

   }).signers([userWallet]).rpc()

   const balance = await provider.connection.getBalance(vault_Account_pda);
   console.log("The Balance in pda is", balance);

  }  catch(e){
    console.error(e);
  }
  });

});
