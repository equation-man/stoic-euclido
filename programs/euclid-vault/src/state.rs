//! CONTAINS STATE DATA STRUCTURES OR PDAs.
use {
    anchor_lang::prelude::*,
    solana_program::{pubkey::Pubkey},
};


/// PDA seeds for the program.
pub const VAULT_LOG_SEED: &str = "vault_state";
pub const VAULT_SEED: &str = "vault_seed";
pub const VAULT_ENTRY_SEED: &str = "vault_entry";
pub const VAULT_AUTH_SEED: &str = "vault_auth";


/// Defining the data structs for our accounts.
#[account]
pub struct VaultLogs {
    pub bump: u8,
    pub amount: u64,
    pub token_mint: Pubkey,
    pub token_vault_bump: u8,
    pub vault_auth_bump: u8,
    pub vault_auth: Pubkey,
}

#[account]
pub struct VaultEntry {
    pub user: Pubkey,
    pub user_withdraw_wallet_account: Pubkey,
    pub bump: u8,
    pub balance: u64,
    pub withdrawable: u64,
    pub created_at: i64,
    pub lock_time: i64,
}
