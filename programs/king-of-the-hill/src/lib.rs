use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::ID;

#[program]
pub mod king_of_the_hill {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_prize: u64) -> Result<()> {
        require!(initial_prize > 0, ErrorCode::NeedInitialPrize);
        
        let game_state = &mut ctx.accounts.game_state;

        game_state.king = ctx.accounts.initial_king.key();
        game_state.prize = initial_prize;

        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.initial_king.key(),
            &ctx.accounts.prize_pool.key(),
            initial_prize,
        );

        invoke(
            &transfer_instruction,
            &[
                ctx.accounts.initial_king.to_account_info(),
                ctx.accounts.prize_pool.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())

    }
    
    pub fn become_king(ctx: Context<BecomeKing>, new_prize: u64) -> Result<()> {
        require!(
            new_prize > ctx.accounts.game_state.prize,
            ErrorCode::BidTooLow
        );

        let transfer_to_pool_instruction = system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.prize_pool.key(),
            new_prize,
        );

        invoke(
            &transfer_to_pool_instruction,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.prize_pool.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        ctx.accounts.prize_pool.sub_lamports(ctx.accounts.game_state.prize);
        ctx.accounts.king.add_lamports(ctx.accounts.game_state.prize);

        ctx.accounts.game_state.king = ctx.accounts.payer.key();
        ctx.accounts.game_state.prize = new_prize;

        Ok(())

    }  
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, 
        payer = initial_king,
        space = DISCRIMINATOR + GameState::INIT_SPACE, 
        seeds = [b"game_state"], 
        bump,
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub initial_king: Signer<'info>,
    #[account(init, 
        payer = initial_king, 
        space = DISCRIMINATOR + 8, 
        seeds = [b"prize_pool"], 
        bump,
    )]
    /// CHECK: This is okay - it's a PDA to store SOL and doesn't need a data layout
    pub prize_pool: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BecomeKing<'info> {
    #[account(mut, has_one = king,)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    /// CHECK: This is okay - it's only receiving SOL and we don't need any other access
    pub king: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"prize_pool"], bump)]
    /// CHECK: This is okay - it's only receiving SOL and we don't need any other access
    pub prize_pool: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>
}

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

const DISCRIMINATOR: usize = 8;

