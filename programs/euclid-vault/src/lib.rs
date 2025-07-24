use anchor_lang::prelude::*;

declare_id!("8mDiter36WTa8pXZxkCSmZmbpB9JPHRa3fta8CaZy2gj");

#[program]
pub mod euclid_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
