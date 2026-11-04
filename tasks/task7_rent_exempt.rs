use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let position = &mut ctx.accounts.position;
        position.owner = ctx.accounts.owner.key();
        position.collateral = 0;
        position.borrowed = 0;
        Ok(())
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        let position = &mut ctx.accounts.position;
        position.collateral += amount;
        Ok(())
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        let position = &mut ctx.accounts.position;
        require!(position.collateral * 100 >= amount * 150, LendingError::InsufficientCollateral);
        position.borrowed += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: position account, created manually
    #[account(mut)]
    pub position: AccountInfo<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut, has_one = owner)]
    pub position: Account<'info, Position>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut, has_one = owner)]
    pub position: Account<'info, Position>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Position {
    pub owner: Pubkey,
    pub collateral: u64,
    pub borrowed: u64,
}

#[error_code]
pub enum LendingError {
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
}
