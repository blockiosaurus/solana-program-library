use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("TokejQddRXE5bRhY5NuvThX3mLQT1c4we56c1Bhnebc");

#[program]
mod spl_token_set {
    use super::*;
    
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        mint_authority: Option<Pubkey>,
        freeze_authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.mint.mint_authority = mint_authority;
        ctx.accounts.mint.freeze_authority = freeze_authority;
        Ok(())
    }

    pub fn initialize_account(
        ctx: Context<InitializeAccount>,
        size: u64,
        mint: Pubkey,
        owner: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.token_set.mint = mint;
        ctx.accounts.token_set.owner = owner;
        ctx.accounts.token_set.close_authority = Option::None;

        ctx.accounts.token_set.tokens = Vec::with_capacity(size as usize);
        Ok(())
    }

    pub fn transfer(
        ctx: Context<Transfer>,
        token_number: u32,
    ) -> Result<()> {
        if ctx.accounts.token_set.tokens[token_number as usize] == ctx.accounts.source_owner.key() {
            ctx.accounts.token_set.tokens[token_number as usize] = ctx.accounts.destination_owner.key();
        }
        Ok(())
    }

    //pub fn approve(_ctx: Context<Approve>) -> Result<()> {
    //    Ok(())
    //}

    //pub fn revoke(_ctx: Context<Revoke>) -> Result<()> {
    //    Ok(())
    //}

    pub fn set_authority(_ctx: Context<SetAuthority>) -> Result<()> {
        Ok(())
    }

    pub fn mint_to(_ctx: Context<MintTo>) -> Result<()> {
        Ok(())
    }

    pub fn burn(_ctx: Context<Burn>) -> Result<()> {
        Ok(())
    }

    pub fn close_account(_ctx: Context<CloseAccount>) -> Result<()> {
        Ok(())
    }

    pub fn freeze_account(_ctx: Context<FreezeAccount>) -> Result<()> {
        Ok(())
    }

    pub fn thaw_account(_ctx: Context<ThawAccount>) -> Result<()> {
        Ok(())
    }

    //pub fn transfer_checked(_ctx: Context<TransferChecked>) -> Result<()> {
    //    Ok(())
    //}

    //pub fn approve_checked(_ctx: Context<ApproveChecked>) -> Result<()> {
    //    Ok(())
    //}

    //pub fn mint_to_checked(_ctx: Context<MintToChecked>) -> Result<()> {
    //    Ok(())
    //}

    //pub fn burn_checked(_ctx: Context<BurnChecked>) -> Result<()> {
    //    Ok(())
    //}
}

#[derive(Accounts)]
pub struct InitializeMint <'info> {
    #[account(init, payer = user, space = 8 + 4 + 4)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(size: u64)]
pub struct InitializeAccount <'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 8 + 4 + (2 * 32 * size) as usize)] // Generous guess of BTreeMap size requirements
    pub token_set: Account<'info, TokenSet>,
    //pub mint: Pubkey,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,}

#[derive(Accounts)]
pub struct Transfer <'info> {
    #[account(mut)]
    pub token_set: Account<'info, TokenSet>,
    #[account(mut)]
    pub source_owner: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is just a wallet being sent the new token, so no check necessary
    pub destination_owner: UncheckedAccount<'info>,
}

//#[derive(Accounts)]
//pub struct Approve {}

//#[derive(Accounts)]
//pub struct Revoke {}

#[derive(Accounts)]
pub struct SetAuthority {}

#[derive(Accounts)]
pub struct MintTo {}

#[derive(Accounts)]
pub struct Burn {}

#[derive(Accounts)]
pub struct CloseAccount {}

#[derive(Accounts)]
pub struct FreezeAccount {}

#[derive(Accounts)]
pub struct ThawAccount {}

//#[derive(Accounts)]
//pub struct TransferChecked {}

//#[derive(Accounts)]
//pub struct ApproveChecked {}

//#[derive(Accounts)]
//pub struct MintToChecked {}

//#[derive(Accounts)]
//pub struct BurnChecked {}

pub enum AccountState {
    Unitialized,
    Initialized,
    Frozen,
}

pub enum AuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
}

#[account]
pub struct Mint {
    /// Optional authority used to mint new tokens. The mint authority may only be provided during
    /// mint creation. If no mint authority is present then the mint has a fixed supply and no
    /// further tokens may be minted.
    pub mint_authority: Option<Pubkey>,
    /// Total supply of tokens.
    //pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    //pub decimals: u8,
    /// Is `true` if this structure has been initialized
    //pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: Option<Pubkey>,
}

#[account]
pub struct TokenSet {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Option<Pubkey>,
    /// The size of the collection
    pub size: u64,
    /// Optional authority to close the account.
    pub close_authority: Option<Pubkey>,
    /// The vector of token owners
    pub tokens: Vec<Pubkey>,
}