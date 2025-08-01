//! CONTAINS HELPER FUNCTIONS.
use { 
    anchor_lang::prelude::*,
    solana_program::{pubkey::Pubkey}, 
    anchor_spl::{
        token::ID as TOKEN_ID,
        token_2022::ID as TOKEN22_ID
    },
};

/// Checks and logs the token program used.
pub fn check_token_program(token_program: Pubkey) {
    match token_program {
        TOKEN_ID => {
            msg!("Targeting original token program.");
        },
        TOKEN22_ID => {
            msg!("Targeting Token22 program.");
        },
        _ => {
            msg!("Given pubkey is not a Token program.");
        }
    }
    msg!("Targeting token program: {}", token_program);
}
