use anchor_lang::prelude::*;

declare_id!("9j4hBaQSzLMxdRKdPSveY7fwrPQwdUkvPf99tNyWBcGZ");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
