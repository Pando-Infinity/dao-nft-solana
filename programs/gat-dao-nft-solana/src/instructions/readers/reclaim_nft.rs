use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::{
  cross_programs::Nft,
  helpers::{
    errors::ErrorCode,
    PROPOSAL_INFO_ACCOUNT_PREFIX,
    UNRECLAIM_NFT_ACCOUNT_PREFIX,
    VOTE_INFO_ACCOUNT_PREFIX,
  },
  schemas::{ ProposalInfo, ReaderUnReclaimNfts, VoteLogging },
};

#[derive(Accounts)]
pub struct ReaderReclaimNft<'info> {
  #[account()]
  pub proposal_info: Account<'info, ProposalInfo>,

  #[account(
      mut,
      seeds = [
        UNRECLAIM_NFT_ACCOUNT_PREFIX.as_ref(),
        &reader.key().as_ref(),
      ],
      bump,
    )]
  pub reader_unreclaim_nft: Account<'info, ReaderUnReclaimNfts>,

  #[account(
      mut,
      seeds = [
        VOTE_INFO_ACCOUNT_PREFIX.as_ref(),
        &proposal_info.key().as_ref(),
        &reader.key().as_ref(),
      ],
      bump,
    )]
  pub vote_logging: Account<'info, VoteLogging>,

  pub nft_mint: Box<Account<'info, token::Mint>>,

  #[account(associated_token::mint = nft_mint, associated_token::authority = proposal_info)]
  pub proposal_nft_account: Box<Account<'info, token::TokenAccount>>,

  #[account(
      mut,
      associated_token::mint = nft_mint,
      associated_token::authority = reader
    )]
  pub user_nft_account: Box<Account<'info, token::TokenAccount>>,

  pub reader: Signer<'info>,

  pub time: Sysvar<'info, Clock>,

  pub token_program: Program<'info, token::Token>,
}

pub fn reclaim_nft(ctx: Context<ReaderReclaimNft>) -> Result<()> {
  let reader_unreclaim_nft = &mut ctx.accounts.reader_unreclaim_nft;
  let proposal_info = &ctx.accounts.proposal_info;
  let vote_logging = &mut ctx.accounts.vote_logging;
  if vote_logging.is_reclaim {
    return Err(error!(ErrorCode::NftReclaimed));
  }
  let proposal_index = reader_unreclaim_nft.proposal_info_list
    .iter()
    .position(|&x| x == proposal_info.key());
  if proposal_index.is_none() {
    return Err(error!(ErrorCode::NftNotAvailableToReclaim));
  }
  let now = ctx.accounts.time.unix_timestamp as u64;
  if
    vote_logging.nft_to_vote != ctx.accounts.nft_mint.key() ||
    reader_unreclaim_nft.nft_list[proposal_index.unwrap()] != ctx.accounts.nft_mint.key()
  {
    return Err(error!(ErrorCode::NftReclaimNotMatch));
  }
  if proposal_info.end_time > now {
    return Err(error!(ErrorCode::CannotReclaimInVoteTime));
  }
  let seeds: &[&[&[u8]]] = &[
    &[
      PROPOSAL_INFO_ACCOUNT_PREFIX.as_ref(),
      &proposal_info.writer.to_bytes(),
      &[proposal_info.bump],
    ],
  ];
  Nft::transfer_with_seed(
    ctx.accounts.proposal_nft_account.to_account_info(),
    ctx.accounts.user_nft_account.to_account_info(),
    proposal_info.to_account_info(),
    ctx.accounts.token_program.to_account_info(),
    seeds
  )?;
  vote_logging.is_reclaim = true;
  reader_unreclaim_nft.proposal_info_list.remove(proposal_index.unwrap());
  Ok(())
}