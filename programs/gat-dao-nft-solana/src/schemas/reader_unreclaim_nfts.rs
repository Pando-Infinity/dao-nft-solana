use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::helpers::MAX_UNRECLAIM_NFTS_SIZE;

#[account]
pub struct ReaderUnReclaimNfts {
  pub proposal_info_list: Vec<Pubkey>,
  pub nft_list: Vec<Pubkey>,
}

impl ReaderUnReclaimNfts {
  pub const SIZE: usize = size_of::<Pubkey>() * 2 * MAX_UNRECLAIM_NFTS_SIZE + 8;
}