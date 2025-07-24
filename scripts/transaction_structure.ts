import {
    LAMPORTS_PER_SOL,
    SystemProgram,
    Transaction,
    Keypair,
    Connection
  } from "@solana/web3.js";
  
  const connection = new Connection("http://localhost:8899", "confirmed");
  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash();
  
  // Generate sender and recipient keypairs
  const sender = Keypair.generate();
  const recipient = new Keypair();
  
  // Define the amount to transfer
  const transferAmount = 0.01; // 0.01 SOL
  
  // Create a transfer instruction for transferring SOL from sender to recipient
  const transferInstruction = SystemProgram.transfer({
    fromPubkey: sender.publicKey,
    toPubkey: recipient.publicKey,
    lamports: transferAmount * LAMPORTS_PER_SOL // Convert transferAmount to lamports
  });
  
  const transaction = new Transaction({
    blockhash,
    lastValidBlockHeight,
    feePayer: sender.publicKey
  }).add(transferInstruction);
  transaction.sign(sender);
  
  const compiledMessage = transaction.compileMessage();
  console.log(JSON.stringify(compiledMessage, null, 2));