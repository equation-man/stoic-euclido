//! THIS IS THE INSTRUCTION TO UNLOCK OR WITHDRAW FUNDS FROM THE VAULT
use {
    anchor_lang::prelude::*;
    crate::{state::*, utils::*},
    anchor_spl::{token_interface},
    std::mem::size_of
};

pub fn handler(ctx: Context<UnlockFunds>, withdraw_amount: u64) -> Result<()> {
    check_token_program(ctx.accounts.token_program.key());

    Ok(())
}

#[derive(Accounts)]
pub struct UnlockFunds<'info> {
    // Vault logs account for recording the logs.
    #[account(
        mut,
        seeds = [token_mint.key().as_ref(), VAULT_LOG_SEED.as_bytes()],
        bump = vault_logs.bump
    )]
    pub vault_logs: Account<'info, VaultLogs>,
    // Mint of the token to transfer.
    #[account(
        mut,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    /// CHECK: PDA, auth over all token vaults
    #[account(
        seeds = [VAULT_AUTH_SEED.as_bytes()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [token_mint.key().as_ref(), vault_authority.key().as_ref(), VAULT_SEED.as_bytes()],
        bump = vault_logs.bump,
        token::token_program = token_program
    )]
    pub token_vault: InterfaceAccount<'info, token_interface::ToknAccount>,
    #[account(
        mut,
        constraint = user.key() == user_vault_entry.user @ EuclidVaultError::InvalidUserError,
    )]
    pub user: Signer<'info>,
    #[account(
        mut,
        constraint = user_token_account.mint == vault_logs.token_mint @ EuclidVaultError::InvalidMintError,
        token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
        mut,
        seeds = [user.key().as_ref(), VAULT_ENTRY_SEED.as_bytes()],
        bump = user_vault_entry.bump
    )]
    pub user_vault_entry: Account<'info, VaultEntry>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> UnlockFunds<'info>{
    // transfer_checked for Token22
}
