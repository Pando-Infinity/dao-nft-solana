use std::mem::size_of;

use anchor_lang::prelude::*;

#[account]
pub struct VoteLogging {
  pub is_voted: bool,
  pub vote_power: u64,
  pub nft_to_vote: Pubkey,
  pub is_reclaim: bool,
}

impl VoteLogging {
  pub const SIZE: usize = size_of::<VoteLogging>() + 8;
}