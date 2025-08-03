//! INITIALIZING THE VAULT FOR LOCKING UP OUR TOKEN
use {
    anchor_lang::prelude::*,
    crate::{state::*, utils::*},
    anchor_spl::{token_interface},
    std::mem::size_of
};

pub fn handler(ctx: Context<InitializeVault>) -> Result<()> {
    check_token_program(ctx.accounts.token_program.key());

    // Initialize vault log
    let vault_logs = &mut ctx.accounts.vault_logs;
    vault_logs.bump = ctx.bumps.vault_logs;
    vault_logs.amount = 0;
    vault_logs.token_mint = ctx.accounts.token_mint.key();
    vault_logs.token_vault_bump = ctx.bumps.token_vault;
    vault_logs.vault_auth_bump = ctx.bumps.vault_authority;
    vault_logs.vault_auth = ctx.accounts.vault_authority.key();

    msg!("Stoic Euclido Vault created!");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// CHECK; PDA, We are passing here ourselves, authority of the vault.
    #[account(
        seeds = [VAULT_AUTH_SEED.as_bytes()],
        bump
    )]
    pub vault_authority:UncheckedAccount<'info>,
    // Transactions logs account for the vault.
    #[account(
        init,
        payer = payer,
        seeds = [token_mint.key().as_ref(), VAULT_LOG_SEED.as_bytes()],
        bump,
        space = 8 + size_of::<VaultLogs>()
    )]
    pub vault_logs: Account<'info, VaultLogs>,
    // Mint of the token.
    #[account(
        mint::token_program = token_program,
        mint::authority = payer,
    )]
    pub token_mint: InterfaceAccount<'info, token_interface::Mint>,
    // Vault token account for the Token Mint
    #[account(
        init,
        token::mint = token_mint,
        token::authority = vault_authority,
        token::token_program = token_program,
        // Use token_mint, vault_auth and constant as seeds for token vault.
        seeds = [token_mint.key().as_ref(), vault_authority.key().as_ref(), VAULT_SEED.as_bytes()],
        bump,
        payer = payer,
    )]
    pub token_vault: InterfaceAccount<'info, token_interface::TokenAccount>,

    // payer, pays for creation of accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}
