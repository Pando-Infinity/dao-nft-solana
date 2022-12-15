use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct VoteData {
  pub reader: [u8; 32],
  pub vote_power: u64,
  pub proposal_info: [u8; 32],
}