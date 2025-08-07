// The main anchor framework with utilities for Solana development
import * as anchor from "@coral-xyz/anchor";
// Typescript type for strongly-typed program interactions
import { Program } from "@coral-xyz/anchor";
// Import auto generated typescript types from the king-of-the-hill program IDL
import { KingOfTheHill } from "../../target/types/king_of_the_hill";
// Chai assertion library for testing
import { assert } from "chai";

// "describe" groups related tests together into a test suite. The first argument is a
// description of what is being tested, the second is a function containing the tests.
describe("King of the Hill Tests", () => {
      // Configure the network provider according to the current environment, in order:
    // 1. ANCHOR_PROVIDER_URL environment variable
    // 2. Solana CLI config from "solana config get"
    // 3. Anchor.toml config file under [provider]
    // 4. Default fallback (usually localnet)
  const provider = anchor.AnchorProvider.env();

  // Sets the provider for all Anchor operations
  anchor.setProvider(provider);

  // anchor.workspace is an object that is created automatically by anchor when running tests.
  // It contains references to all deployed programs. When "Anchor test" is ran, Anchor reads
  // the Anchor.toml file, looks at [programs.localnet], deploys the program to the test validator
  // and creates the workspace entries with the program names.
  //
  // Program<T> is a generic type that provides type safety for a program's methods.
  // This line casts the generic workspace object to a strongly-typed Program interface.
  const program = anchor.workspace.KingOfTheHill as Program<KingOfTheHill>;

  let initialKing, newKing;
  let gameStatePDA, prizePoolPDA;

  // Utility function for airdrops
  async function fundWallet(account, amount) {
    // If account is a keypair, extract the publicKey, if it's already a publicKey object
    // use it directly
    const publicKey = account.publicKey ? account.publicKey : account;
 
    // Request an airdrop for publicKey
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(publicKey, amount),
      "confirmed"
    );
  }

  // The before() hook is a test function that runs once before all other tests in the 
  // current test suite. It is useful for one-time setup operations (e.g., funding, wallet creation) 
  // that must be performed before the tests can be executed.
  before(async () => {
    // Generate two new wallet accounts for testing
    initialKing = anchor.web3.Keypair.generate();
    newKing = anchor.web3.Keypair.generate();

    // Fund and initialize both wallets with airdrops
    await fundWallet(initialKing, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await fundWallet(newKing, 4 * anchor.web3.LAMPORTS_PER_SOL);

    // Derives a PDA for the game state account, matching the constraint in the
    // Initialize struct in the Rust program. 
    [gameStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("game_state")],
      program.programId
    );

    // Derives a PDA for the prize pool account, matching the constraint in the
    // BecomeKing struct in the Rust program
    [prizePoolPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("prize_pool")],
      program.programId
    );
  });

  // it() represents individual test cases. The first argument is the test description,
  // and the second argument is a function containing the test logic.
  it("Initializes the game correctly", async () => {
    await fundWallet(gameStatePDA, 1 * anchor.web3.LAMPORTS_PER_SOL);
    await fundWallet(prizePoolPDA, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Creates a BigNumber for the initial prize amount (1,000,000,000 lamports)
    // anchor.BN is Anchor's BigNumber type for handling large integers safely.
    let initialPrize = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);

    // Calls the initialize instruction from the Rust program
    const tx = await program.methods
      .initialize(initialPrize)
      .accounts({
        gameState: gameStatePDA,
        initialKing: initialKing.publicKey,
        prizePool: prizePoolPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([initialKing]) // Initial king pays for account creation and initial prize
      .rpc(); // Sends the transaction to the blockchain and waits for confirmation

    // Fetches the game state from the blockchain
    let gameState: any = await program.account.gameState.fetch(gameStatePDA);

    // Checks that initialKing has been recorded as the current king using .toBase58() to
    // convert public keys to comparable strings.
    assert.equal(gameState.king.toBase58(), initialKing.publicKey.toBase58());

    // Checks that the prize amount is exactly 1 SOL (the bid amount)
    assert.equal(
      gameState.prize.toString(),
      new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL).toString()
    );
  });
 
  it("Changes the king correctly", async () => {
    
    const initialKingBalanceBefore = await provider.connection.getBalance(initialKing.publicKey);
    let newPrize = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);

    const becomeKingTx = await program.methods.becomeKing(newPrize)
      .accounts({
          gameState: gameStatePDA,
          king: initialKing.publicKey, // Correct usage of current king
          payer: newKing.publicKey, // New king who pays and becomes the king
          prizePool: prizePoolPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newKing]) // Signing by newKing
      .rpc();

    // Assert
    const initialKingBalanceAfter = await provider.connection.getBalance(initialKing.publicKey);

    const expectedBalance = initialKingBalanceBefore + new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL).toNumber();
    assert.ok(initialKingBalanceAfter >= expectedBalance, "Old king did not receive the funds back correctly");

    // Fetch the updated game state.
    const updatedGameState = await program.account.gameState.fetch(gameStatePDA);

    // Assertions to confirm the state has updated as expected.
    assert.equal(updatedGameState.king.toBase58(), newKing.publicKey.toBase58(), "King should be updated to newKing.");
    assert.equal(updatedGameState.prize.toString(), newPrize.toString(), "Prize should be updated to newPrize.");
  })
});