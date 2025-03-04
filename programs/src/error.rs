use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
    #[msg("Only the shares' subject can buy the first share")]
    FirstShareOnlyForSubject,
    #[msg("Insufficient payment")]
    InsufficientPayment,
    #[msg("Account already exist")]
    AccountAlreadyExist,
    #[msg("Cannot sell the last share")]
    SellLastShare,
    #[msg("Insufficient shares")]
    InsufficientShares,
}
