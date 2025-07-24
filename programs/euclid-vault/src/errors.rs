//! CUSTOM ERRORS FOR THE TOKEN PROGRAM
use anchor_lang::prelude::*;

#[erro_code]
pub enum EuclidVaultError {
    #[msg!("The token mint is invalid")]
    InvalidMintError,
    #[msg!("Invalid user provided")]
    InvalidUserError,
    #[msg!("Mathematical overflow occured")]
    MathematicalOverflowError,
    #[msg!("Maximum balance exceeded")]
    InvalidAccountBalance
}
