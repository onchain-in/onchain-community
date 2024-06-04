use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constant::*, states::*};

#[derive(Accounts)]
#[instruction(
	url: String,
  content: String
)]
pub struct CreateComment<'info> {
	#[account(
		init,
		// seeds = [
		// 	COMMENT_SEED,
    //   url,
		// ],
		// bump,
		payer = authority,
		space = size_of::<CommentAccount>() + url.len() + content.len() + 4
	)]
	pub comment: Account<'info, CommentAccount>,
	#[account(mut)]
	pub authority: Signer<'info>,
	pub system_program: Program<'info, System>,
}

pub fn create_comment(
	ctx: Context<CreateComment>,
  url: String,
  content: String,
) -> Result<()> {
	let comment = &mut ctx.accounts.comment;

  comment.authority = ctx.accounts.authority.key();
  comment.url = url;
  comment.content = content;
  comment.vote = 0;

	Ok(())
}