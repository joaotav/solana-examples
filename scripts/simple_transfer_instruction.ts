import {
    LAMPORTS_PER_SOL,
    SystemProgram,
    Transaction,
    Keypair
  } from "@solana/web3.js";
  
  // Generate sender and recipient keypairs
  const sender = Keypair.generate(); // Keypair.generate() is equivalent to new Keypair()
  const recipient = new Keypair();
  
  // Define the amount to transfer
  const transferAmount = 0.01; // 0.01 SOL
  
  // Create a transfer instruction for transferring SOL from sender to recipient
  // by calling the system program
  const transferInstruction = SystemProgram.transfer({
    fromPubkey: sender.publicKey,
    toPubkey: recipient.publicKey,
    lamports: transferAmount * LAMPORTS_PER_SOL // Convert transferAmount to lamports
  });
  
  console.log(JSON.stringify(transferInstruction, null, 2));

  