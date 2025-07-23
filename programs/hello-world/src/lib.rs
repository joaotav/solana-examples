use anchor_lang::prelude::*;

// This macro sets the public key for the program.
declare_id!("9j4hBaQSzLMxdRKdPSveY7fwrPQwdUkvPf99tNyWBcGZ");

// the #[program] macro marks this module as a Solana program
#[program]
pub mod hello_world {
    use super::*; // Imports all items from the parent module scope (Context, Result, msg!, etc.)

    // A function defines an instruction that can be called from clients
    // The Context parameter contains Accounts naeeded for this instruction and program metadata
    pub fn hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello"); // msg!() is Anchor's logging macro - this prints "Hello" to the program logs
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
