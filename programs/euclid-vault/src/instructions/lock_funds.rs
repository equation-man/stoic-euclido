//! THIS IS THE INSTRUCTION TO LOCK FUNDS.
use {
    anchor_lang::prelude::*;
    crate::{state::*, utils::*},
    anchor_spl::{token_interface},
    std::mem::size_of
};

pub fn handler(ctx: Context<LockFunds>, lock_amount: u64) -> Result<()> {
    check_token_program(ctx.accounts.token_program.key());
    
    msg!("Lock initial total: {}", ctx.accounts.vault_logs.amount);
    msg!("User entry amount: {}", ctx.accounts.user_vault_entry.balance);

    let decimals = ctx.accounts.token_mint.decimals;
    // Transfering the tokens
    transfer_checked(ctx.accounts.transfer_checked_ctx(), lock_amount, decimals)?;

    // Updating the account status.
    let vault_logs = &mut ctx.accounts.vault_logs;
    let user_vault_entry = &mut ctx.accounts.user_vault_entry;

    // Update the vault log amount.
    vault_logs.amount = vault_logs.amount.checked_add(lock_amount).unwrap();
    msg!("Updated total amount locked in the vault: {}", vault_logs.amount);

    // Update the user vault entry.
    user_vault_entry.balance = user_vault_entry.balance.checked_add(lock_amount).unwrap();
    msg!("Updated wallet amount: {}", user_vault_entry.balance);
    user_vault_entry.created_at = Clock::get().unwrap().unix_timestamp;

    // IMPLEMENT LOCK DURATION HERE, FEE CALCULATION TO ADJUST AMOUNT WITHDRAWABLE.

    Ok(())
}

#[derive(Accounts)]
pub struct LockFunds<'info> {
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
    pub vault_authority: UncheckedAccount<'info>
    #[account(
        mut,
        seeds = [token_mint.key().as_ref(), vault_authority.key().as_ref(), VAULT_SEED.as_bytes()],
        bump = vault_logs.bump,
        token::token_program = token_program
    )]
    pub token_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
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
    pub user_token_account: InterfaceAccount<'info, token_interface:TokenAccount><
    #[account(
        mut,
        seeds = [user.key().as_ref(), VAULT_ENTRY_SEED.as_bytes()],
        bump = user_vault_entry.bump
    )]
    pub user_vault_entry: Account<'info, VaultEntry>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> LockFunds<'info>{
    // transfer_checked for Token22
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            to: self.token_vault.to_account_info(),
            from: self.user_token_account.to_account_info(),
            mint: self.token_mint.to_account_info(),
            authority: self.user.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
