use anchor_lang::prelude::*;
use std::mem::size_of;

#[account]
pub struct DaoInfo {
  pub max_vote_option_per_proposal: u8,
  pub min_vote_option_per_proposal: u8,
  pub max_proposal_content_length: u16,
  pub max_vote_option_content_length: u16,
  pub verifier: Pubkey,
}

impl DaoInfo {
  pub const SIZE: usize = size_of::<DaoInfo>() + 8;
}

pub trait DaoInfoAccountBehavior {
  fn is_valid_vote_option(&self, vote_option: &Vec<String>) -> bool;
  fn is_valid_proposal_content(&self, proposal_content: &str) -> bool;
}

impl<'info> DaoInfoAccountBehavior for Account<'info, DaoInfo> {
  fn is_valid_vote_option(&self, vote_option: &Vec<String>) -> bool {
    let vote_option_exceed_content_length = vote_option
      .iter()
      .position(|option| option.len() > (self.max_vote_option_content_length as usize));
    if vote_option_exceed_content_length.is_some() {
      return false;
    }
    if
      vote_option.len() <= (self.max_vote_option_per_proposal as usize) &&
      vote_option.len() >= (self.min_vote_option_per_proposal as usize)
    {
      return true;
    }
    return false;
  }

  fn is_valid_proposal_content(&self, proposal_content: &str) -> bool {
    return proposal_content.len() <= (self.max_proposal_content_length as usize);
  }
}