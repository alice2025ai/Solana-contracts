pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::instructions::*;
use crate::utils::{get_price, sol_transfer, LAMPORTS_ONE_SOL};
use solana_program::msg;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

#[program]
mod shares {
    use super::*;
    pub fn create_shares_parameter(ctx: Context<CreateSharesParameter>) -> Result<()> {
        require!(
            ctx.accounts.shares_parameter.owner == System::id(),
            ErrorCode::AccountAlreadyExist
        );
        ctx.accounts.shares_parameter.owner = *ctx.accounts.owner.key;
        ctx.accounts.shares_parameter.bump = ctx.bumps.shares_parameter;
        Ok(())
    }

    pub fn change_parameter_owner(
        ctx: Context<ChangeParameterOwner>,
        new_owner: Pubkey,
    ) -> Result<()> {
        ctx.accounts.shares_parameter.owner = new_owner;
        Ok(())
    }

    pub fn set_fee_destination(
        ctx: Context<SetFeeDestination>,
        protocol_fee_destination: Pubkey,
    ) -> Result<()> {
        ctx.accounts.shares_parameter.protocol_fee_destination = protocol_fee_destination;
        Ok(())
    }

    pub fn set_subject_fee_percent(
        ctx: Context<SetSubjectFeePercent>,
        subject_fee_percent: u64,
    ) -> Result<()> {
        ctx.accounts.shares_parameter.subject_fee_percent = subject_fee_percent;
        Ok(())
    }

    pub fn set_protocol_fee_percent(
        ctx: Context<SetProtocolFeePercent>,
        protocol_fee_percent: u64,
    ) -> Result<()> {
        ctx.accounts.shares_parameter.protocol_fee_percent = protocol_fee_percent;
        Ok(())
    }

    pub fn create_shares(ctx: Context<CreateShares>) -> Result<()> {
        ctx.accounts.shares_info.supply = 0;
        ctx.accounts.shares_info.shares_subject = *ctx.accounts.subject.key;
        ctx.accounts.shares_info.bump = ctx.bumps.shares_info;
        Ok(())
    }

    pub fn create_shares_balance(ctx: Context<CreateSharesBalance>) -> Result<()> {
        ctx.accounts.shares_balance.balance = 0;
        ctx.accounts.shares_balance.shares_subject = *ctx.accounts.shares_subject.key;
        ctx.accounts.shares_balance.user = *ctx.accounts.user.key;
        ctx.accounts.shares_balance.bump = ctx.bumps.shares_balance;
        Ok(())
    }

    pub fn buy_shares(ctx: Context<BuyShares>, amount: u64) -> Result<()> {
        let shares_subject = &ctx.accounts.shares_subject;
        require!(
            ctx.accounts.shares_info.supply > 0 || shares_subject.key() == ctx.accounts.user.key(),
            ErrorCode::FirstShareOnlyForSubject
        );
        let price = get_price(ctx.accounts.shares_info.supply, amount);
        let protocol_fee_percent = ctx.accounts.shares_parameter.protocol_fee_percent;
        let subject_fee_percent = ctx.accounts.shares_parameter.subject_fee_percent;
        let protocol_fee = price * protocol_fee_percent / LAMPORTS_ONE_SOL;
        let subject_fee = price * subject_fee_percent / LAMPORTS_ONE_SOL;
        msg!(
            "price = {},protocolFee={},subjectFee={}",
            price,
            protocol_fee,
            subject_fee
        );
        require!(
            ctx.accounts.user.lamports() >= price + protocol_fee + subject_fee,
            ErrorCode::InsufficientPayment
        );
        ctx.accounts.shares_balance.balance += amount;
        ctx.accounts.shares_info.supply += amount;
        msg!(
            "user {} buy shares {} amount {} at price {}",
            ctx.accounts.user.key,
            shares_subject.key,
            amount,
            price
        );

        sol_transfer(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            price,
        )?;
        sol_transfer(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.protocol_fee_destination.to_account_info(),
            protocol_fee,
        )?;
        sol_transfer(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.shares_subject.to_account_info(),
            subject_fee,
        )?;
        Ok(())
    }

    pub fn sell_shares(ctx: Context<BuyShares>, amount: u64) -> Result<()> {
        let shares_subject = &ctx.accounts.shares_subject;
        require!(
            ctx.accounts.shares_info.supply > amount,
            ErrorCode::SellLastShare
        );
        let price = get_price(ctx.accounts.shares_info.supply - amount, amount);
        let protocol_fee_percent = ctx.accounts.shares_parameter.protocol_fee_percent;
        let subject_fee_percent = ctx.accounts.shares_parameter.subject_fee_percent;
        let protocol_fee = price * protocol_fee_percent / LAMPORTS_ONE_SOL;
        let subject_fee = price * subject_fee_percent / LAMPORTS_ONE_SOL;
        msg!(
            "price = {},protocolFee={},subjectFee={}",
            price,
            protocol_fee,
            subject_fee
        );
        require!(
            ctx.accounts.shares_balance.balance > amount,
            ErrorCode::InsufficientShares
        );
        require!(
            ctx.accounts.vault.to_account_info().lamports() >= price,
            ErrorCode::InsufficientPayment
        );
        ctx.accounts.shares_balance.balance -= amount;
        ctx.accounts.shares_info.supply -= amount;
        msg!(
            "user {} sell shares {} amount {} at price {}",
            ctx.accounts.user.key,
            shares_subject.key,
            amount,
            price
        );

        **ctx
            .accounts
            .vault
            .to_account_info()
            .try_borrow_mut_lamports()? -= price;
        **ctx
            .accounts
            .user
            .to_account_info()
            .try_borrow_mut_lamports()? += price - protocol_fee - subject_fee;
        **ctx
            .accounts
            .protocol_fee_destination
            .to_account_info()
            .try_borrow_mut_lamports()? += protocol_fee;
        **ctx
            .accounts
            .shares_subject
            .to_account_info()
            .try_borrow_mut_lamports()? += subject_fee;

        Ok(())
    }

}
