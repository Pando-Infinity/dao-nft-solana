use crate::cross_programs::Nft;
use crate::helpers::{
    errors::ErrorCode, PROPOSAL_INFO_ACCOUNT_PREFIX, WRITER_DAO_INFO_ACCOUNT_PREFIX,
};
use crate::helpers::{events::*, VoteData, UNRECLAIM_NFT_ACCOUNT_PREFIX, VOTE_INFO_ACCOUNT_PREFIX};
use crate::schemas::{DaoInfo, ProposalInfo, ReaderUnReclaimNfts, VoteLogging, WriterInfo};
use crate::{verify_signature, DAO_INFO_ACCOUNT_PREFIX};
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use mpl_token_metadata::{
    pda::{EDITION, PREFIX},
    state::{Metadata, TokenMetadataAccount},
    ID,
};

#[derive(Accounts)]
#[instruction(proposal_number: u16)]
pub struct ReaderVoteWithNft<'info> {
    #[account(seeds = [DAO_INFO_ACCOUNT_PREFIX.as_ref()], bump)]
    pub dao_info: Account<'info, DaoInfo>,

    #[account(seeds = [WRITER_DAO_INFO_ACCOUNT_PREFIX.as_ref(), &writer.key().as_ref()], bump)]
    pub writer_info: Account<'info, WriterInfo>,

    #[account(
    seeds = [
      PROPOSAL_INFO_ACCOUNT_PREFIX.as_ref(),
      &writer.key().as_ref(),
      proposal_number.to_string().as_bytes().as_ref(),
    ],
    bump
  )]
    pub proposal_info: Account<'info, ProposalInfo>,

    #[account(
    init_if_needed,
    seeds = [
      VOTE_INFO_ACCOUNT_PREFIX.as_ref(),
      &proposal_info.key().as_ref(),
      &reader.key().as_ref(),
    ],
    bump,
    payer = reader,
    space = VoteLogging::SIZE
  )]
    pub vote_logging: Account<'info, VoteLogging>,

    #[account(
    init_if_needed,
    seeds = [UNRECLAIM_NFT_ACCOUNT_PREFIX.as_ref(), &reader.key().as_ref()],
    bump,
    payer = reader,
    space = ReaderUnReclaimNfts::SIZE
  )]
    pub reader_unreclaim_nft: Account<'info, ReaderUnReclaimNfts>,

    pub nft_mint: Box<Account<'info, token::Mint>>,

    #[account(
    init_if_needed,
    payer = reader,
    associated_token::mint = nft_mint,
    associated_token::authority = proposal_info
  )]
    pub proposal_nft_account: Box<Account<'info, token::TokenAccount>>,

    #[account(
      mut,
      associated_token::mint = nft_mint,
      associated_token::authority = reader
    )]
    pub user_nft_account: Box<Account<'info, token::TokenAccount>>,

    /// CHECK: nft metadata account
    #[account(
    seeds = [PREFIX.as_bytes().as_ref(), ID.as_ref(), nft_mint.key().as_ref()],
    bump,
    seeds::program = ID
  )]
    pub nft_metadata_account: AccountInfo<'info>,

    /// CHECK: nft master edition account
    #[account(
    seeds = [
      PREFIX.as_bytes().as_ref(),
      ID.as_ref(),
      nft_mint.key().as_ref(),
      EDITION.as_bytes().as_ref(),
    ],
    bump,
    seeds::program = ID
  )]
    pub nft_master_edition_account: AccountInfo<'info>,

    /// CHECK: writer account
    pub writer: AccountInfo<'info>,

    #[account(mut)]
    pub reader: Signer<'info>,

    pub time: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, token::Token>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,

    /// CHECK: Just a pure account
    pub pre_instruction: AccountInfo<'info>,
}

pub fn vote_with_nft(
    ctx: Context<ReaderVoteWithNft>,
    _proposal_number: u16,
    vote_option_index: u16,
    vote_data: VoteData,
) -> Result<()> {
    let proposal_info = &mut ctx.accounts.proposal_info;
    let nft_metadata_account = &ctx.accounts.nft_metadata_account;
    let nft_master_edition_account = &ctx.accounts.nft_master_edition_account;
    let writer = &ctx.accounts.writer;
    let reader = &ctx.accounts.reader;
    let writer_info = &ctx.accounts.writer_info;
    let vote_logging = &mut ctx.accounts.vote_logging;
    let reader_unreclaim_nft = &mut ctx.accounts.reader_unreclaim_nft;
    let now = ctx.accounts.time.unix_timestamp as u64;

    if reader.key().to_bytes().ne(&vote_data.reader)
        || proposal_info.key().to_bytes().ne(&vote_data.proposal_info)
        || !verify_signature(
            &ctx.accounts.pre_instruction,
            &ctx.accounts.dao_info.verifier,
            vote_data.try_to_vec()?,
        )?
    {
        return Err(error!(ErrorCode::InvalidVoteData));
    }
    if vote_logging.is_voted {
        return Err(error!(ErrorCode::AlreadyVoted));
    }
    if !proposal_info.can_vote(now, vote_option_index) {
        return Err(error!(ErrorCode::CannotVotNow));
    }
    if nft_metadata_account.data_is_empty() || nft_master_edition_account.data_is_empty() {
        return Err(error!(ErrorCode::NftNotInitialized));
    }
    let nft_metadata_account_data = &Metadata::from_account_info(nft_metadata_account).unwrap();
    if !Nft::verify_creator(nft_metadata_account_data, writer.key) {
        return Err(error!(ErrorCode::NftCreatorNotMatch));
    }
    if !Nft::verify_collection(
        nft_metadata_account_data,
        &writer_info.nft_collection_to_vote,
    ) {
        return Err(error!(ErrorCode::NftCollectionNotMatch));
    }
    Nft::transfer(
        ctx.accounts.user_nft_account.to_account_info(),
        ctx.accounts.proposal_nft_account.to_account_info(),
        ctx.accounts.reader.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
    )?;

    proposal_info.vote_option[vote_option_index as usize].amount += vote_data.vote_power;
    vote_logging.is_voted = true;
    vote_logging.is_reclaim = false;
    vote_logging.vote_power = vote_data.vote_power;
    vote_logging.nft_to_vote = ctx.accounts.nft_mint.key();
    reader_unreclaim_nft
        .nft_list
        .push(ctx.accounts.nft_mint.key());
    reader_unreclaim_nft
        .proposal_info_list
        .push(ctx.accounts.proposal_info.key());
    emit!(ProposalVoted {
        reader: ctx.accounts.reader.key(),
        writer: ctx.accounts.writer.key(),
        proposal: ctx.accounts.proposal_info.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        timestamp: now,
    });
    Ok(())
}
