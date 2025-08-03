pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use {
    anchor_lang::prelude::*,
    anchor_lang::solana_program::{pubkey::Pubkey},
    instructions::*
};

declare_id!("8mDiter36WTa8pXZxkCSmZmbpB9JPHRa3fta8CaZy2gj");

#[program]
pub mod euclid_vault {
    use super::*;

    pub fn init_vault(ctx: Context<InitializeVault>) -> Result<()> {
        //msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Creating vault...");
        init_vault::handler(ctx)
    }

    pub fn init_vault_entry(ctx: Context<InitVaultEntry>, target_wallet_key: Pubkey) -> Result<()> {
        msg!("Creating vault entry...");
        init_vault_entry::handler(ctx, target_wallet_key)
    }

    pub fn lock_funds(ctx: Context<LockFunds>, amount: u64) -> Result<()> {
        msg!("Locking funds...");
        lock_funds::handler(ctx, amount)
    }

    pub fn unlock_funds(ctx: Context<UnlockFunds>, withdraw_amount: u64) -> Result<()> {
        msg!("Unlocking funds...");
        unlock_funds::handler(ctx, withdraw_amount)
    }
}
