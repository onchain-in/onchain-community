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
		space = 8 + size_of::<CommentAccount>()
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
	comment.vote = 0;

	let url_bytes = url.as_bytes();
	let url_len = url_bytes.len().min(64);
	comment.url[..url_len].copy_from_slice(&url_bytes[..url_len]);
	if url_len < 256 {
		comment.url[url_len..].fill(0);
	}

	let content_bytes = content.as_bytes();
	let content_len = content_bytes.len().min(1024);
	comment.content[..content_len].copy_from_slice(&content_bytes[..content_len]);
	if content_len < 512 {
		comment.content[content_len..].fill(0);
	}

	Ok(())
}