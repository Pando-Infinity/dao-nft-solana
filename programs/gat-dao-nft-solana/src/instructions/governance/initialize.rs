use crate::helpers::{
  DAO_INFO_ACCOUNT_PREFIX,
  DEFAULT_MAX_VOTE_OPTION_AMOUNT,
  DEFAULT_MIN_VOTE_OPTION_AMOUNT,
  DEFAULT_PROPOSAL_CONTENT_LENGTH,
  DEFAULT_VOTE_CONTENT_LENGTH,
  GOVERNANCE_DAO_INFO_ACCOUNT_PREFIX,
};
use crate::schemas::{ DaoInfo, GovernanceInfo };
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GovernanceInitialize<'info> {
  #[account(
    init,
    seeds = [GOVERNANCE_DAO_INFO_ACCOUNT_PREFIX.as_ref()],
    bump,
    payer = admin,
    space = GovernanceInfo::SIZE
  )]
  pub governance_info: Account<'info, GovernanceInfo>,

  #[account(
    init,
    seeds = [DAO_INFO_ACCOUNT_PREFIX.as_ref()],
    bump,
    payer = admin,
    space = DaoInfo::SIZE
  )]
  pub dao_info: Account<'info, DaoInfo>,

  #[account(mut)]
  pub admin: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<GovernanceInitialize>, verifier: Pubkey) -> Result<()> {
  let governance_info = &mut ctx.accounts.governance_info;
  let dao_info = &mut ctx.accounts.dao_info;
  governance_info.admin = ctx.accounts.admin.key();
  dao_info.max_proposal_content_length = DEFAULT_PROPOSAL_CONTENT_LENGTH;
  dao_info.max_vote_option_content_length = DEFAULT_VOTE_CONTENT_LENGTH;
  dao_info.max_vote_option_per_proposal = DEFAULT_MAX_VOTE_OPTION_AMOUNT;
  dao_info.min_vote_option_per_proposal = DEFAULT_MIN_VOTE_OPTION_AMOUNT;
  dao_info.verifier = verifier;
  Ok(())
}