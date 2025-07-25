import {
    LAMPORTS_PER_SOL,
    SystemProgram,
    Transaction,
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

// Gets the recent blockhash needed for Message.recent_blockhash along with it's height
// A hash is valid for recent_blockhash for approximately 150 blocks after it has been created
const { blockhash, lastValidBlockHeight } =
  await connection.getLatestBlockhash();

// Generate sender and recipient Ed25519 keypairs
const sender = Keypair.generate();
const recipient = new Keypair();

// Requests an aidrop from the network and returns a transaction signature string
const signature = await connection.requestAirdrop(
  sender.publicKey,
  LAMPORTS_PER_SOL * 0.02 // Request 0.02 SOL
)

await connection.
// Define the amount to transfer
const transferAmount = 0.01; // 0.01 SOL

// Create a transfer instruction for transferring SOL from sender to recipient 
const transferInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey, // Sender address
  toPubkey: recipient.publicKey, // Receiver address
  lamports: transferAmount * LAMPORTS_PER_SOL // Convert transferAmount to lamports
});

// Create a transaction with explicit, manually set parameters
const transaction = new Transaction({
  blockhash,
  lastValidBlockHeight,
  feePayer: sender.publicKey
}).add(transferInstruction);

// Sign the transaction
transaction.sign(sender);

// Compiles message and logs it
const compiledMessage = transaction.compileMessage();
console.log(JSON.stringify(compiledMessage, null, 2));