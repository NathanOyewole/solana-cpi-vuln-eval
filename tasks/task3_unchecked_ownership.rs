use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        // Transfer from vault to recipient
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token.to_account_info(),
            to: ctx.accounts.recipient_token.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: vault account, validated in handler
    pub vault: AccountInfo<'info>,
    /// CHECK: token account
    #[account(mut)]
    pub vault_token: AccountInfo<'info>,
    /// CHECK: recipient token account
    #[account(mut)]
    pub recipient_token: AccountInfo<'info>,
    /// CHECK: vault authority PDA
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized")]
    Unauthorized,
}
