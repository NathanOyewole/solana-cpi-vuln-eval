use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.balance = 0;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.balance >= amount, VaultError::InsufficientFunds);
        vault.balance -= amount;
        Ok(())
    }

    pub fn set_authority(ctx: Context<SetAuthority>, new_authority: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = new_authority;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum VaultError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
