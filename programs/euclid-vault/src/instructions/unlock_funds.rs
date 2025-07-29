//! THIS IS THE INSTRUCTION TO UNLOCK OR WITHDRAW FUNDS FROM THE VAULT
use {
    anchor_lang::prelude::*;
    crate::{state::*, utils::*, errors::*},
    anchor_spl::{token_interface},
    std::mem::size_of
};

pub fn handler(ctx: Context<UnlockFunds>, withdraw_amount: u64) -> Result<()> {
    check_token_program(ctx.accounts.token_program.key());

    let user_vault_entry = &ctx.accounts.user_vault_entry;
    let balance = user_vault_entry.balance;
    let decimals = ctx.accounts.token_mint.decimals;

    msg!("The stake balance is: {}", user_vault_entry.balance);
    msg!("Withdrawal amount is: {}", withdraw_amount);
    msg!("The total vault balance is: {}", ctx.accounts.vault_logs.amount);

    // Verify if the user have >= requested withdrawal amount in their stake.
    if withdraw_amount > user_vault_entry.balance {
        return Err(EuclidVaultError::InvalidAccountBalance);
    }

    // Program signer seeds
    let auth_bump = ctx.accounts.vault_logs.vault_auth_bump;
    let auth_seeds = &[VAULT_AUTH_SEED.as_bytes(), &[auth_bump]];
    let signer = &[&auth_seeds[..]];

    // Transfer tokens
    transfer_checked(ctx.accounts.transfer_checked_ctx(signer), amount, decimals)?;

    // Borrow mutable reference to update the state of the accounts
    let vault_logs = &mut ctx.accounts.vault_logs;
    let user_entry = &mut ctx.accounts.user_vault_entry;

    // Subract the transferred amount from the pool total.
    vault_logs.amount = vault_logs.amount.checked_sub(withdrawa_amount).unwrap();
    msg!("The total vault balance is: {}", vault_logs.amount);

    // Update the stake entry.
    user_entry.balance = user_entry.balance.checked_sub(withdrawal_amount).unwrap();
    user_entry.created_at = Clock::get().unwrap().unix_timestamp;

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
    pub fn transfer_checked_ctx<'a>($'a self, seeds: &'a [&[&[u8]]]) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.token_vault.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.vault_authority.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds)
    }
}
