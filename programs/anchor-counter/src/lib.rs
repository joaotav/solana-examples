use anchor_lang::prelude::*;

declare_id!("3YBAAuxLMjGFUpYZMpJcLb1S2efo4A9v4RdatxvZvp9d");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
