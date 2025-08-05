use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

declare_id!("iALpvVQS1CoqrFuoHopycSv8tUCZRu75dZbfWBYj9ny");

#[program]
pub mod king_of_the_hill {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_prize: u64) -> Result<()> {
        // Ensures that the initial prize is greater than zero
        require!(initial_prize > 0, ErrorCode::NeedInitialPrize);

        let game_state = &mut ctx.accounts.game_state;

        // Sets initial king from Initialize as the first king
        game_state.king = ctx.accounts.initial_king.key();
        // Records the initial prize amount in the game state
        game_state.prize = initial_prize;

        // Call the System Program to prepare a transfer instruction
        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.initial_king.key(), // sender's pubkey
            &ctx.accounts.prize_pool.key(), // receiver's pubkey
            initial_prize, // amount in lamports
        );

        // The invoke function receives the instruction we want to execute as the first
        // parameter and an array of accounts required by the instruction, in order, as
        // the second parameter.
        invoke(
            &transfer_instruction,
            &[
                // The System Program doesn't understand Anchor's typed wrappers, it only 
                // works with raw AccountInfo structs, hence the conversion.
                ctx.accounts.initial_king.to_account_info(), // sender's account
                ctx.accounts.prize_pool.to_account_info(), // receiver's account
                ctx.accounts.system_program.to_account_info(), // system program
            ],
        )?; // Will propagate an error if the instruction fails to execute

        Ok(())
    }

    pub fn become_king(ctx: Context<BecomeKing>, new_prize: u64) -> Result<()> {
        // Ensures that the new prize is bigger than the current prize
        require!(
            new_prize > ctx.accounts.game_state.prize,
            ErrorCode::BidTooLow
        );

        // Call the system program to prepare a transfer instruction
        let transfer_to_pool_instruction = system_instruction::transfer(
            &ctx.accounts.payer.key(), // sender's pubkey
            &ctx.accounts.prize_pool.key(), // receiver's pubkey
            new_prize, // lamports
        );

        // Execute the previously prepared instruction
        invoke(
            &transfer_to_pool_instruction,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.prize_pool.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Perform manual lamport manipulation to refund the previous king.
        // Cross-program invocations to the system program (with system_instruction::)
        // are not needed because the prize_pool PDA is owned by this program, so it can 
        // directly modify its lamports.
        ctx.accounts.prize_pool.sub_lamports(ctx.accounts.game_state.prize);
        ctx.accounts.king.add_lamports(ctx.accounts.game_state.prize);

        // Sets the new bidder as the current king
        ctx.accounts.game_state.king = ctx.accounts.payer.key();
        // Updates the prize to the new higher amount
        ctx.accounts.game_state.prize = new_prize;

        Ok(())
    }
}


// In essence, #[derive(Accounts)] is used to prepare a structure that contains all the accounts
// required by an instruction along with constraints for each of those accounts. The macro validates
// accounts, deserializes account data and performs security checks.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, // Create a new account on the blockchain
        payer = initial_king, // initial_king will pay the rent for creating this account
        space = DISCRIMINATOR + GameState::INIT_SPACE, // Allocate space for the new account
        seeds = [b"game_state"], // Creates a PDA using this seed
        bump, // Anchor automatically stores the canonical bump used for the PDA
    )]
    // This account will contain GameState data
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    // initial_king needs to be mutable to pay for account creation
    pub initial_king: Signer<'info>,
    #[account(init, 
        payer = initial_king, 
        space = DISCRIMINATOR + 8, 
        seeds = [b"prize_pool"], // Create a PDA using this seed
        bump,
    )]
    /// CHECK: This is okay - it's a PDA to store SOL and doesn't need a data layout
    // This PDA will be used by the program to store the prize money.
    // Unchecked accounts are not deserialized or validated by Anchor, they contain raw account data
    // from AccountInfo. Unchecked accounts are efficient in this case where we only need to store
    // SOL and check the balance.
    pub prize_pool: UncheckedAccount<'info>,
    // Reference to Solana's built-in System Program, which is required whenever creating new
    // accounts or transferring SOL.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BecomeKing<'info> {
    #[account(mut, // The account is mutable because the current king needs to be updated
        // Constraint to check if the "king" field in GameState matches the key of the "king" 
        // field in BecomeKing
        has_one = king, 
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    /// CHECK: This is okay - it's only receiving SOL and we don't need any other access
    pub king: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"prize_pool"], bump)]
    // seeds = [b"prize_pool"] is a constraint requiring that the account passed to this field
    // is always equal to the PDA derived with the seeds [b"prize_pool"] and this program's ID. 
    // If a different account is passed, the transaction will fail. This makes sure there is 
    // only one valid prize pool address and only this program can own it.
    /// CHECK: This is okay - it's only receiving SOL and we don't need any other access
    pub prize_pool: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
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
pub struct GameState {
    pub king: Pubkey,
    pub prize: u64,
    pub prize_pool_bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The initial prize must be greater than zero")]
    NeedInitialPrize,
    #[msg("The bid must be higher than the current prize")]
    BidTooLow,
    #[msg("Invalid prize pool account")]
    InvalidPrizePoolAccount,
}

// The size of the account type discriminator
const DISCRIMINATOR: usize = 8;