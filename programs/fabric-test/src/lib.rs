use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

use std::ops::Deref;

declare_id!("2BG8YKnnFPq9KMEK9APRchysCHau86nC1iRig5j6TJVT");

#[program]
pub mod fabric_test {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, name: String) -> ProgramResult {
        msg!("Initialize staking pool!");

        let pool = &mut ctx.accounts.pool;

        let name_bytes = name.as_bytes();
        let mut name_data = [b' '; 20];
        name_data[..name_bytes.len()].copy_from_slice(name_bytes);

        pool.name = name_data;
        pool.bumps = BumpSeeds {
            pool: *ctx.bumps.get("pool").unwrap(),
            staking_vault: *ctx.bumps.get("staking_vault").unwrap(),
            redeemable_mint: *ctx.bumps.get("redeemable_vault").unwrap(),
            authority: *ctx.bumps.get("authority").unwrap(),
        };
        pool.staking_vault = ctx.accounts.staking_vault.key();
        pool.staking_mint = ctx.accounts.staking_mint.key();
        pool.redeemable_mint = ctx.accounts.redeemable_mint.key();

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> ProgramResult {
        msg!("Stake SPL token to the vault");

        if ctx.accounts.source.amount < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        let pool_name = ctx.accounts.pool.name.as_ref();
        let seeds = &[
            AUTHORITY_SEED.as_bytes(),
            pool_name.strip(),
            &[ctx.accounts.pool.bumps.authority],
        ];

        token::transfer(ctx.accounts.into_transfer_context(), amount)?;
        token::mint_to(ctx.accounts.into_mint_to_context(&[&seeds[..]]), amount)?;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
        msg!("Unstake SPL token to the vault");

        if ctx.accounts.source.amount < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        let pool_name = ctx.accounts.pool.name.as_ref();
        let seeds = &[
            AUTHORITY_SEED.as_bytes(),
            pool_name.strip(),
            &[ctx.accounts.pool.bumps.authority],
        ];

        token::transfer(ctx.accounts.into_transfer_context(&[&seeds[..]]), amount)?;
        token::burn(ctx.accounts.into_burn_context(), amount)?;

        Ok(())
    }
}

const AUTHORITY_SEED: &str = "authority_seed";
const VAULT_SEED: &str = "vault_seed";
const REDEEMABLE_MINT_SEED: &str = "redeemable_seed";

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    #[account(
        init,
        seeds = [name.as_bytes()],
        bump,
        payer = payer
    )]
    pub pool: Box<Account<'info, StakePool>>,
    #[account(
        seeds = [AUTHORITY_SEED.as_bytes(), name.as_bytes()],
        bump
    )]
    pub authority: AccountInfo<'info>,
    pub staking_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        token::mint = staking_mint,
        token::authority = authority,
        seeds = [VAULT_SEED.as_bytes(), name.as_bytes()],
        bump,
        payer = payer
    )]
    pub staking_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        mint::decimals = 9 as u8,
        mint::authority = authority,
        seeds = [REDEEMABLE_MINT_SEED.as_bytes(), name.as_bytes()],
        bump,
        payer = payer
    )]
    pub redeemable_mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        has_one = staking_vault,
        has_one = redeemable_mint
    )]
    pub pool: Box<Account<'info, StakePool>>,
    #[account(mut)]
    pub staking_vault: AccountInfo<'info>,
    #[account(mut)]
    pub redeemable_mint: AccountInfo<'info>,
    #[account(
        mut,
        constraint = source.mint == pool.staking_mint @ ErrorCode::InvalidMint
    )]
    pub source: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = destination.mint == pool.redeemable_mint @ ErrorCode::InvalidMint
    )]
    pub destination: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [AUTHORITY_SEED.as_bytes(), pool.name.as_ref().strip()],
        bump = pool.bumps.authority
    )]
    pub pool_authority: AccountInfo<'info>,
    pub user_authority: Signer<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn into_mint_to_context<'a, 'b, 'c>(
        &self,
        signer: &'a [&'b [&'c [u8]]],
    ) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.redeemable_mint.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.pool_authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new_with_signer(cpi_program, cpi_accounts, signer)
    }

    pub fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.source.to_account_info(),
            to: self.staking_vault.to_account_info(),
            authority: self.user_authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        has_one = staking_vault,
        has_one = redeemable_mint
    )]
    pub pool: Box<Account<'info, StakePool>>,
    #[account(mut)]
    pub staking_vault: AccountInfo<'info>,
    #[account(mut)]
    pub redeemable_mint: AccountInfo<'info>,
    #[account(
        mut,
        constraint = source.mint == pool.redeemable_mint @ ErrorCode::InvalidMint
    )]
    pub source: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = destination.mint == pool.staking_mint @ ErrorCode::InvalidMint
    )]
    pub destination: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [AUTHORITY_SEED.as_bytes(), pool.name.as_ref().strip()],
        bump = pool.bumps.authority
    )]
    pub pool_authority: AccountInfo<'info>,
    pub user_authority: Signer<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    pub fn into_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.redeemable_mint.to_account_info(),
            to: self.source.to_account_info(),
            authority: self.user_authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_transfer_context<'a, 'b, 'c>(
        &self,
        signer: &'a [&'b [&'c [u8]]],
    ) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.staking_vault.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.pool_authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new_with_signer(cpi_program, cpi_accounts, signer)
    }
}

#[account]
#[derive(Default)]
pub struct StakePool {
    pub name: [u8; 20],
    pub staking_vault: Pubkey,
    pub staking_mint: Pubkey,
    pub redeemable_mint: Pubkey,
    pub bumps: BumpSeeds,
}

#[derive(Clone, Default, AnchorDeserialize, AnchorSerialize)]
pub struct BumpSeeds {
    pub pool: u8,
    pub staking_vault: u8,
    pub redeemable_mint: u8,
    pub authority: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}

pub trait StripAsciiWhitespace {
    fn strip(&self) -> &[u8];
}

impl<T: Deref<Target = [u8]>> StripAsciiWhitespace for T {
    fn strip(&self) -> &[u8] {
        let from = match self.iter().position(|x| !x.is_ascii_whitespace()) {
            Some(i) => i,
            None => return &self[0..0],
        };
        let to = self.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
        &self[from..=to]
    }
}
