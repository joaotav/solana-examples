// Wildcard import that brings all the commonly used items from the Anchor framework's prelude
// module into scope. It provides access to essential items like the core Anchor macros,
// account types, context types, error handling, commonly used traits and utility types.
use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
mod hello_anchor {
    // This brings all items from the parent module into scope.
    use super::*;
    // While in practice you can define instructions without a Context<T> argument, they are
    // very limited and cannot access any accounts, read/write blockchain state or transfer SOL.
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // The attribute #[account(init)] indicates that this account is being created
    // rather than just being accessed.
    // The attribute `payer = signer` indicates who pays for the account creation
    // and the amount of SOL required for rent exemption.
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    // The #[account(mut)] attribute tells the program that this account will be modified
    // during instruction execution, which requires the account to be marked as writable.
    // In this context, the signer needs to be mutable because its lamport balance will be
    // modified when creating the new account (to pay for rent).
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    data: u64,
}
