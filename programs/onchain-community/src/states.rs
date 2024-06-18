use anchor_lang::prelude::*;

#[account]
pub struct CommentAccount {
  pub authority: Pubkey,
  pub vote: i8,
  pub url: [u8; 128],
  pub content: [u8; 512],
}

#[account]
pub struct CommentUpvoteAccount {
  pub authority: Pubkey,
  pub comment: Option<Pubkey>,
  pub vote: i8,
}