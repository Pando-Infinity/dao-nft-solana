use anchor_lang::prelude::*;

use crate::{
  helpers::{
    enums::VoteBy,
    errors::ErrorCode,
    Utils,
    DAO_INFO_ACCOUNT_PREFIX,
    GOVERNANCE_DAO_INFO_ACCOUNT_PREFIX,
    PROPOSAL_INFO_ACCOUNT_PREFIX,
    WRITER_CREATE_PROPOSAL_ROLE,
    WRITER_DAO_INFO_ACCOUNT_PREFIX,
  },
  schemas::{
    DaoInfo,
    DaoInfoAccountBehavior,
    GovernanceInfo,
    ProposalInfo,
    VoteInfo,
    WriterInfo,
    WriterInfoAccountBehavior,
  },
};

#[derive(Accounts)]
#[instruction(proposal_id: u16)]
pub struct WriterCreateProposal<'info> {
  #[account(seeds = [GOVERNANCE_DAO_INFO_ACCOUNT_PREFIX.as_ref()], bump)]
  pub governance_info: Account<'info, GovernanceInfo>,

  #[account(seeds = [DAO_INFO_ACCOUNT_PREFIX.as_ref()], bump)]
  pub dao_info: Account<'info, DaoInfo>,

  #[account(mut, seeds = [WRITER_DAO_INFO_ACCOUNT_PREFIX.as_ref(), &writer.key().as_ref()], bump)]
  pub writer_info: Account<'info, WriterInfo>,

  #[account(
    init_if_needed,
    seeds = [
      PROPOSAL_INFO_ACCOUNT_PREFIX.as_ref(),
      &writer.key().as_ref(),
      proposal_id.to_string().as_bytes().as_ref(),
    ],
    bump,
    payer = user,
    space = ProposalInfo::SIZE
  )]
  pub proposal_info: Account<'info, ProposalInfo>,

  #[account(mut)]
  pub user: Signer<'info>,

  /// CHECK: writer account
  pub writer: AccountInfo<'info>,

  pub system_program: Program<'info, System>,

  pub time: Sysvar<'info, Clock>,
}

pub fn create_proposal(
  ctx: Context<WriterCreateProposal>,
  proposal_id: u16,
  proposal_content: String,
  vote_by: u8,
  vote_option: Vec<String>,
  start_time: u64,
  end_time: u64
) -> Result<()> {
  let writer_info = &mut ctx.accounts.writer_info;
  let proposal_info = &mut ctx.accounts.proposal_info;
  let dao_info = &ctx.accounts.dao_info;
  let now = ctx.accounts.time.unix_timestamp as u64;
  let _vote_by: VoteBy = VoteBy::try_from(vote_by)?;

  if !writer_info.verify() {
    return Err(error!(ErrorCode::UnverifiedWriter));
  }
  if
    !writer_info.has_role(ctx.accounts.user.key(), WRITER_CREATE_PROPOSAL_ROLE) ||
    ctx.accounts.user.key() == ctx.accounts.writer.key()
  {
    return Err(error!(ErrorCode::PermissionRequired));
  }
  if
    !writer_info.is_valid_proposal_id(proposal_id) ||
    !Utils::is_happening(now, start_time, end_time) ||
    !dao_info.is_valid_proposal_content(&proposal_content) ||
    !dao_info.is_valid_vote_option(&vote_option)
  {
    return Err(error!(ErrorCode::InvalidProposalParameter));
  }
  writer_info.set_next_proposal_id(proposal_id + 1);
  proposal_info.vote_option = vote_option
    .iter()
    .map(|option| VoteInfo {
      vote_content: option.to_owned(),
      amount: 0,
    })
    .collect::<Vec<VoteInfo>>();
  proposal_info.content = proposal_content;
  proposal_info.vote_by = vote_by;
  proposal_info.start_time = start_time;
  proposal_info.end_time = end_time;
  proposal_info.writer = ctx.accounts.writer.key();
  proposal_info.id = proposal_id;
  proposal_info.bump = *ctx.bumps.get("proposal_info").unwrap();
  Ok(())
}