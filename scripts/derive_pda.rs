import { PublicKey } from "@solana/web3.js";

const programAddress = new PublicKey("11111111111111111111111111111111");

const optionalSeedString = "helloWorld";
const optionalSeedAddress = new PublicKey(
  "B9Lf9z5BfNPT4d5KMeaBFx8x1G4CULZYR1jA2kmxRDka"
);
const seeds = [Buffer.from(optionalSeedString), optionalSeedAddress.toBuffer()];
const [pda, bump] = await PublicKey.findProgramAddressSync(
  seeds,
  programAddress
);

console.log(`PDA: ${pda}`);
console.log(`Bump: ${bump}`);