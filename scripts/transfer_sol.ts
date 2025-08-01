import {
    LAMPORTS_PER_SOL,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,
    Keypair,
    Connection
  } from "@solana/web3.js";
  
// Creates a new connection object to interact with the Solana localnet.
// Sets the default commitment parameter to "confirmed" meaning that operations
// must be confirmed on chain.
// Solana has different commitment levels:
// - `"processed"` - fastest, least secure (transaction processed by leader)
// - `"confirmed"` - balanced (confirmed by cluster, ~400ms)
// - `"finalized"` - slowest, most secure (confirmed by supermajority, ~13 seconds)
//
// Subsequent operations using the connection will use the commitment parameter specified when
// the connection was created, unless they explicitly specify a different parameter.
const connection = new Connection("http://localhost:8899", "confirmed");

// Generate sender and recipient Ed25519 keypairs
const sender = new Keypair();
const receiver = new Keypair();

// Request an airdrop from the network and returns a transaction
// signature string
const signature = await connection.requestAirdrop(
sender.publicKey,
LAMPORTS_PER_SOL // A constant containing 1 billion (1 billion lamports = 1 SOL)
);

const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash();

// Waits for the airdrop transaction to be confirmed on the blockchain,
// ensuring the SOL is in the sender's account before proceeding.
// The blockhash and lastValidBlockHeeight are used to prevent indefinite waiting.
await connection.confirmTransaction({signature, blockhash, lastValidBlockHeight}, "confirmed");

let senderBalance = await connection.getBalance(sender.publicKey);
let receiverBalance = await connection.getBalance(receiver.publicKey);

console.log("Sender's initial balance:", `${senderBalance}`);
console.log("Receiver's initial balance:", `${receiverBalance}`);

// Creates an instruction to transfer SOL between accounts.
// Uses Solana's System Program
const transferInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: receiver.publicKey,
  lamports: 0.01 * LAMPORTS_PER_SOL // transfers 0.01 SOL (10,000,000 lamports)
});

// Creates a new transaction object/container and adds the transfer instruction
// to the transaction. Transactions can contain multiple instructions.
const transaction = new Transaction().add(transferInstruction);

// Signs the transaction with the Sender's private key and sends it to the network.
// Waits for confirmation and returns the transaction signature.
// This function handles the recent_blockhash and other required metadata and then
// sends the transaction to the network and waits for confirmation.
const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [sender] // signer keypair  
);

console.log("Transaction Signature:", `${transactionSignature}`);

senderBalance = await connection.getBalance(sender.publicKey);
receiverBalance = await connection.getBalance(receiver.publicKey);

console.log("Sender's final balance:", `${senderBalance}`);
console.log("Receiver's final balance:", `${receiverBalance}`);


