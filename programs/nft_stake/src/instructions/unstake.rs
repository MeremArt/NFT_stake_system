use anchor_lang::{prelude::*, solana_program::stake::state::Stake};
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        }, 
        MasterEditionAccount, 
        Metadata
    }, 
    token::{
        revoke, 
        Mint, 
        Revoke, 
        Token, 
        TokenAccount
    }
};

use crate::{errors::StakeError, state::{StakeAccount, StakeConfig, UserAccount}};

#[derive(Accounts)]

pub struct Unstake<'info> {

    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info , Mint>,
#[account(mut,
associated_token::mint= mint,
associated_token::authority = user,
)]
pub mint_ata:Account<'info,TokenAccount>,
#[account(
    seeds = [
        b"metadata",
        metadata_program.key().as_ref(),
        mint.key().as_ref(),
        b"edition"
    ],
    seeds::program = metadata_program.key(),
    bump,
)]
pub edition: Account<'info, MasterEditionAccount>,

#[account(
    seeds = [b"config".as_ref()],
    bump = config.bump,
)]

pub config: Account<'info, StakeConfig>,

#[account(
    mut,
    close = user,
    seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
    bump
)]

pub stake_account: Account<'info, StakeAccount>,

#[account(
    mut,
    seeds = [b"user".as_ref(), user.key().as_ref()],
    bump = user_account.bump,
)]
pub user_account: Account<'info, UserAccount>,
pub system_program: Program<'info, System>,
pub token_program: Program<'info, Token>,
pub metadata_program: Program<'info, Metadata>,

}