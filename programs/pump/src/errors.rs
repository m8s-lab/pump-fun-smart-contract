use anchor_lang::prelude::*;

#[error_code]
pub enum DexProgramError {
    #[msg("Insufficient funds to swap")]
    InsufficientFunds,

    #[msg("Failed to allocate shares")]
    FailedToAllocateShares,

    #[msg("Failed to deallocate shares")]
    FailedToDeallocateShares,

    #[msg("Insufficient shares")]
    InsufficientShares,

    #[msg("Invalid amount to swap")]
    InvalidAmount,

    #[msg("Invalid fee")]
    InvalidFee,

    #[msg("Duplicate tokens are not allowed")]
    DuplicateTokenNotAllowed,

    #[msg("Failed to add liquidity")]
    FailedToAddLiquidity,

    #[msg("Failed to remove liquidity")]
    FailedToRemoveLiquidity,

    #[msg("Overflow or underflow occured")]
    OverflowOrUnderflowOccurred,
}
