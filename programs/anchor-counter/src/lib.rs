use anchor_lang::prelude::*;

declare_id!("7jMYGPxNcnSFsh9LFJgxjqBKxH374xQYjUQ6a2pxfAnp");

#[program]
pub mod anchor_counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter Account Created");
        msg!("Current Count: { }", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, // Create a new account
        payer = user, // The account represented by user will fund the new account
        space = DISCRIMINATOR + Counter::INIT_SPACE)] // Allocate space for the new account
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
// The #[account] attribute sets the discriminator for the new account and also
// sets the owner of the account to the programId in the declare_id! macro
//
// The #[derive(InitSpace)] automatically calculates the exact space in bytes that
// the account structure will need when stored-on chain. Variables of dynamic size
// such as Vec<T> need to be annotated with #[max_len(COUNT)] if T is of fixed size
// or #[max_len(COUNT), ITEM_SIZE] if T is of dynamic size, e.g., String.
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

// The size of the account type discriminator
const DISCRIMINATOR: usize = 8;
