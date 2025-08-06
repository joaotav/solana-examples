// The main anchor framework with utilities for Solana development
import * as anchor from "@coral-xyz/anchor";
// Typescript type for strongly-typed program interactions
import { Program } from "@coral-xyz/anchor";
// Chai assertion library for testing
import { expect } from "chai";
// Import auto generated typescript types from the anchor-counter program IDL
import { AnchorCounter } from "../target/types/anchor_counter";

// "describe" groups related tests together into a test suite. The first argument is a
// description of what is being tested, the second is a function containing the tests.
describe("anchor-counter", () => {
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
    const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;

    // Create an address for a data account to store the counter's state
    const counter = anchor.web3.Keypair.generate();

    // it() represents individual test cases. The first argument is the test description,
    // and the second argument is a function containing the test logic.
    it("Is initialized!", async () => {
        // Call the initialize method from the program. 
        // .accounts() must receive the accounts that are passed in the Context<T> to initialize()
        // which are Counter, Signer and System. The Signer is usually auto-provided and so is the
        // System, so we only specify Counter in .accounts() here. Additional arguments, that are not
        // accounts need to be passed to the method call, e.g., initialize(42, "userX").accounts({..})
        // The .rpc() method builds, signs and submits the transaction, then returns the tx signature.
        const txSignature = await program.methods.initialize().accounts({ counter: counter.publicKey }).signers([counter]).rpc();

        // program.account is Anchor's account namespace, it contains methods for fetching account
        // data from the account types defined in the program. Anchor automatically generates a
        // "counter" property based on the struct Counter.
        // The .fetch() method takes a pubkey as input, makes an RPC call to the network, retrieves
        // the raw account data, deserializes it according to the Counter struct definition and
        // returns a Typescript object. 
        // counter.publicKey is the address of the specific counter instance we want to read from,
        // which is the same address in which we stored data during the initialization.
        const account = await program.account.counter.fetch(counter.publicKey);

        // Test to confirm that the value in the counter variable is equal 0, as is expected
        // after initialization
        expect(account.count.toNumber()).to.equal(0);
    });

    it("Incremented the count", async () => {
        const txSignature = await program.methods.increment().accounts({ counter: counter.publicKey, user: provider.wallet.publicKey }).rpc();

        const account = await program.account.counter.fetch(counter.publicKey);

        // Test to confirm that the value in the counter variable is equal 1, as is expected
        // after incrementing from 0
        expect(account.count.toNumber()).to.equal(1);
    });

})


