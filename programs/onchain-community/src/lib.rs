use anchor_lang::prelude::*;

pub mod constant;
pub mod states;
pub mod error;
pub mod instructions;
use instructions::*;

declare_id!("wNSuywSzp36Q3TgeaCJnBjie1pMpHPbxAzkkvvUKnaX");

#[program]
pub mod onchain_community {
	use super::*;

	pub fn create_comment(
		ctx: Context<CreateComment>,
		url: String,
		content: String,
	) -> Result<()> {
		create_comment::create_comment(ctx, url, content)
	}
}