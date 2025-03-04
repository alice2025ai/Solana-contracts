use anchor_lang::prelude::*;
use crate::state::*;
#[derive(Accounts)]
pub struct CreateSharesParameter<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init_if_needed,
        payer = owner,
        space = SharesParameter::SIZE,
        seeds = [b"parameter"],
        bump,
    )]
    pub shares_parameter: Account<'info, SharesParameter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ChangeParameterOwner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"parameter"],
        bump = shares_parameter.bump,
    )]
    pub shares_parameter: Account<'info, SharesParameter>,
}

#[derive(Accounts)]
pub struct SetFeeDestination<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"parameter"],
        bump = shares_parameter.bump,
    )]
    pub shares_parameter: Account<'info, SharesParameter>,
}

#[derive(Accounts)]
pub struct SetProtocolFeePercent<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"parameter"],
        bump = shares_parameter.bump,
    )]
    pub shares_parameter: Account<'info, SharesParameter>,
}

#[derive(Accounts)]
pub struct SetSubjectFeePercent<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"parameter"],
        bump = shares_parameter.bump,
    )]
    pub shares_parameter: Account<'info, SharesParameter>,
}

#[derive(Accounts)]
pub struct CreateShares<'info> {
    #[account(
        init, 
        payer = subject, 
        space = SharesInfoAccount::SIZE,
        seeds = [b"shares",subject.key().as_ref()],
        bump
    )]
    pub shares_info: Account<'info, SharesInfoAccount>,
    #[account(mut)]
    pub subject: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSharesBalance<'info> {
    #[account(
        init, 
        payer = user,
        space = SharesBalanceAccount::SIZE,
        seeds = [shares_subject.key().as_ref(),user.key().as_ref()],
        bump
    )]
    pub shares_balance: Account<'info, SharesBalanceAccount>,
    pub shares_subject: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyShares<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8,
        seeds = [b"vault"],
        bump
    )]
    pub vault: AccountInfo<'info>,
    pub shares_subject: AccountInfo<'info>,
    #[account(
        mut,
        has_one = shares_subject,
        seeds = [b"shares",shares_subject.key().as_ref()],
        bump = shares_info.bump
    )]
    pub shares_info: Account<'info, SharesInfoAccount>,
    #[account(
        mut,
        has_one = shares_subject,
        has_one = user,
        seeds = [shares_subject.key().as_ref(),user.key().as_ref()],
        bump = shares_balance.bump,
    )]
    pub shares_balance: Account<'info, SharesBalanceAccount>,
    #[account(mut)]
    pub protocol_fee_destination: AccountInfo<'info>,
    #[account(
        mut,
        has_one = protocol_fee_destination,
        seeds = [b"parameter"],
        bump = shares_parameter.bump,
    )]
    pub shares_parameter: Account<'info, SharesParameter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SellShares<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: AccountInfo<'info>,
    pub shares_subject: AccountInfo<'info>,
    #[account(
        mut,
        has_one = shares_subject,
        seeds = [b"shares",shares_subject.key().as_ref()],
        bump = shares_info.bump
    )]
    pub shares_info: Account<'info, SharesInfoAccount>,
    #[account(
        mut,
        has_one = shares_subject,
        has_one = user,
        seeds = [shares_subject.key().as_ref(),user.key().as_ref()],
        bump = shares_balance.bump,
    )]
    pub shares_balance: Account<'info, SharesBalanceAccount>,
    #[account(mut)]
    pub protocol_fee_destination: AccountInfo<'info>,
    #[account(
        mut,
        has_one = protocol_fee_destination,
        seeds = [b"parameter"],
        bump = shares_parameter.bump,
    )]
    pub shares_parameter: Account<'info, SharesParameter>,
    pub system_program: Program<'info, System>,
}
