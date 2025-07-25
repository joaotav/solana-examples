import { PublicKey } from "@solana/web3.js";

// Creates a public key using an address
const userWallet = new PublicKey("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");

// Define an input seed for PDA derivation
const seeds = [
  Buffer.from("user_profile"),
  userWallet.toBuffer()
];

// Derive a PDA using the seed and Program ID.
// Receive the PDA and the canonical bump.
const [pda, bump] = PublicKey.findProgramAddressSync(
  seeds,
  userWallet
);

console.log(`PDA: ${pda}`);
console.log(`Canonical Bump: ${bump}`);