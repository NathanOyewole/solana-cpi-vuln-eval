use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.seller = ctx.accounts.seller.key();
        listing.price = price;
        listing.nft_mint = ctx.accounts.nft_mint.key();
        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
        // Deserialize listing from raw account data
        let listing_data = ctx.accounts.listing.try_borrow_data()?;
        // No ownership check — anyone can pass any account
        let listing: Listing = Listing::try_deserialize(&mut &listing_data[..])?;

        require!(listing.price > 0, MarketplaceError::InvalidPrice);

        // Transfer NFT from seller to buyer
        // Transfer payment from buyer to seller
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(init, payer = seller, space = 8 + 32 + 8 + 32)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub nft_mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyNft<'info> {
    /// CHECK: listing account, deserialized manually
    pub listing: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub price: u64,
    pub nft_mint: Pubkey,
}

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid price")]
    InvalidPrice,
}
