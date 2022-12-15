use crate::helpers::{ GOVERNANCE_DAO_INFO_ACCOUNT_PREFIX, WRITER_DAO_INFO_ACCOUNT_PREFIX };
use crate::schemas::{ GovernanceInfo, WriterInfo };
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GovernanceBan<'info> {
  #[account(seeds = [GOVERNANCE_DAO_INFO_ACCOUNT_PREFIX.as_ref()], bump)]
  pub governance_info: Account<'info, GovernanceInfo>,

  #[account(mut, seeds = [WRITER_DAO_INFO_ACCOUNT_PREFIX.as_ref(), &writer.key().as_ref()], bump)]
  pub writer_info: Account<'info, WriterInfo>,

  /// CHECK: writer account
  pub writer: AccountInfo<'info>,

  #[account(mut)]
  pub admin: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub fn ban(ctx: Context<GovernanceBan>, is_banned: bool) -> Result<()> {
  let writer_info = &mut ctx.accounts.writer_info;
  writer_info.is_banned = is_banned;
  Ok(())
}