use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, reward_mint: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.reward_mint = reward_mint;
        vault.total_staked = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.total_staked += amount;
        // Transfer deposit tokens to vault
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        // Transfer rewards from vault to user
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_reward_token.to_account_info(),
            to: ctx.accounts.user_reward_token.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user_token: AccountInfo<'info>,
    #[account(mut)]
    pub vault_token: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    /// CHECK: vault reward token account
    #[account(mut)]
    pub vault_reward_token: AccountInfo<'info>,
    /// CHECK: user reward token account
    #[account(mut)]
    pub user_reward_token: AccountInfo<'info>,
    /// CHECK: vault authority PDA
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub reward_mint: Pubkey,
    pub total_staked: u64,
}

#[error_code]
pub enum VaultError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
