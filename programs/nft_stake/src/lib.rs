use anchor_lang::prelude::*;
mod state;
mod instructions;
mod errors;

use instructions::*;

declare_id!("HgAsB37QttTnYPWKtQAyGjbUb8eevAygxtJaN8YsWchD");

#[program]
pub mod nft_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
