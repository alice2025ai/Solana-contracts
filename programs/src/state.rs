use anchor_lang::prelude::*;
#[account]
pub struct SharesParameter {
    pub owner: Pubkey,
    pub protocol_fee_percent: u64,
    pub subject_fee_percent: u64,
    pub protocol_fee_destination: Pubkey,
    pub bump: u8,
}

impl SharesParameter {
    pub const SIZE: usize = 8 + 32 * 2 + 8 * 2 + 1;
}
#[account]
pub struct SharesInfoAccount {
    pub shares_subject: Pubkey,
    pub supply: u64,
    pub bump: u8,
}

impl SharesInfoAccount {
    pub const SIZE: usize = 8 + 32 + 8 + 1;
}

#[account]
pub struct SharesBalanceAccount {
    pub shares_subject: Pubkey,
    pub user: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl SharesBalanceAccount {
    pub const SIZE: usize = 8 + 32 * 2 + 8 + 1;
}
