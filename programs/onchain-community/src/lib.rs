use anchor_lang::prelude::*;

declare_id!("wNSuywSzp36Q3TgeaCJnBjie1pMpHPbxAzkkvvUKnaX");

#[program]
pub mod onchain_community {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
