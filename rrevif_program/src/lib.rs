use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use std::str::FromStr;
use anchor_lang::solana_program::system_instruction;

declare_id!(""); //program id

#[program]
pub mod work3_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn payment(ctx: Context<Wallet>) -> ProgramResult {
        let owner: &Signer = &ctx.accounts.owner;
        let fee_amount: u64 = 10000000;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(), // from
            &ctx.accounts.pool.key(), // to
            pool_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.owner.to_account_info(), //from
                ctx.accounts.pool.to_account_info(), //to
            ],
        );

        // fee
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(), // from
            &ctx.accounts.admin.key(), // to
            fee_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.owner.to_account_info(), //from
                ctx.accounts.admin.to_account_info(), //to
            ],
        );

        Ok(())
    }

    pub fn approve_work(ctx: Context<ApproveWork>, approved: bool) -> ProgramResult {
        let pool: &mut Account<Pool> = &mut ctx.accounts.pool;
        let from_account = pool.to_account_info();
        let player_account = ctx.accounts.owner.to_account_info();
        let admin_account = ctx.accounts.admin.to_account_info();

        // if **from_account.try_borrow_lamports()? < amount_of_lamports {
        //     return Err(ErrorCode::ContentTooLong.into());
        // }

        // send payment
        if (approved)
        {
            let lamports = 50000000 as f64;

            pool.win += 1;
            let fee_amount: u64 = 0.001 as u64;
            let work_amount: u64 = 0.001 as u64;
            **from_account.try_borrow_mut_lamports()? -= work_amount;
            **player_account.try_borrow_mut_lamports()? += work_amount;

            // fees
            **from_account.try_borrow_mut_lamports()? -= fee_amount;
            **admin_account.try_borrow_mut_lamports()? += fee_amount;
        }
        else
        {
            // payment not approved
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Wallet<'info> {
    #[account(mut, seeds = [b"hangman_solwager".as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, address = Pubkey::from_str("8WnqfBUM4L13fBUudvjstHBEmUcxTPPX7DGkg3iyMmc8").unwrap())]
    pub admin: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveWork<'info> {
    #[account(mut, seeds = [b"hangman_solwager".as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    #[account(mut, address = Pubkey::from_str("8WnqfBUM4L13fBUudvjstHBEmUcxTPPX7DGkg3iyMmc8").unwrap())]
    pub admin: AccountInfo<'info>,
    #[account(mut, address = Pubkey::from_str("8ECeRHmzdQKE3sBLZ5r8wAoEWfAVNEupJRu3EtkXvA4Q").unwrap())]
    pub server: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}
