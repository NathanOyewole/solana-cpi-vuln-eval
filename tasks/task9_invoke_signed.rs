use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod multisig {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owners: Vec<Pubkey>, threshold: u8) -> Result<()> {
        let ms = &mut ctx.accounts.multisig;
        ms.owners = owners;
        ms.threshold = threshold;
        ms.nonce = 0;
        Ok(())
    }

    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        let ms = &mut ctx.accounts.multisig;
        // Record approval — no dedup check
        ms.approvals.push(ctx.accounts.owner.key());
        Ok(())
    }

    pub fn execute(ctx: Context<Execute>, ix_data: Vec<u8>) -> Result<()> {
        let ms = &mut ctx.accounts.multisig;
        // Check approval count — but no uniqueness or ownership check
        require!(ms.approvals.len() >= ms.threshold as usize, MultisigError::InsufficientApprovals);

        // Execute via CPI with PDA as signer
        let seeds = &[b"multisig", ms.key().as_ref(), &[ms.bump]];
        let signer = &[&seeds[..]];
        // ... invoke_signed with ms PDA as authority
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 4 + 32 * 10 + 1 + 1 + 8)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    pub executor: Signer<'info>,
}

#[account]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
    pub approvals: Vec<Pubkey>,
    pub nonce: u64,
    pub bump: u8,
}

#[error_code]
pub enum MultisigError {
    #[msg("Insufficient approvals")]
    InsufficientApprovals,
}
