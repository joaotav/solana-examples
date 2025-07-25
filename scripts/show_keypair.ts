import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";

// Generates a new random keypair with a 64 bytes private key and a 32 bytes public key
const keypair = Keypair.generate();

console.log(`Public Key: ${keypair.publicKey}`);

// Print the private key encoded as base58 string
console.log(`Secret Key: ${bs58.encode(keypair.secretKey)}`);