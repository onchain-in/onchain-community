use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct CommentAccount {
  pub authority: Pubkey,
  pub url: String,
  pub content: String,
  pub vote: i8,
}

#[account]
#[derive(Default)]
pub struct CommentUpvoteAccount {
  pub authority: Pubkey,
  pub comment: Option<Pubkey>,
  pub vote: i8,
}