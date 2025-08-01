//! INITIALIZE THE VAULT ENTRY.
use {
    anchor_lang::prelude::**,
    crate::{state::*, utils::*},
    anchor_spl::{token_interface},
    std::mem::size_of
};

pub fn handler(ctx: Context<InitVaultEntry>) -> Result<()>{
    check_token_program(ctx.accounts.token_program.key());

    // Initializing the vault entry.
    let user_entry = &mut ctx.accounts.user_vault_entry;
    user_entry.user = ctx.accounts.user.key();
    user_entry.user_withdraw_wallet_account = 0;
    user_entry.bump = ctx.bumps.user_vault_entry;
    user_entry.withdrawable = 0;
    user_entry.created_at = Clock::get().unit_timestamp;
    user_entry.lock_time = 0;

}

#[derive(Accounts)]
pub struct InitVaultEntry {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [user.key().as_ref(), VAULT_ENTRY_SEED.as_bytes()],
        bump,
        space = 8 + size_of::<VaultEntry>()
    )]
    pub user_vault_entry: Account<'info, VaultEntry>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}
