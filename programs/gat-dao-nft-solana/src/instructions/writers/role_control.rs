use anchor_lang::prelude::*;

use crate::{ schemas::{ WriterInfo, WriterInfoAccountBehavior }, WRITER_DAO_INFO_ACCOUNT_PREFIX };

#[derive(Accounts)]
pub struct WriterRole<'info> {
  #[account(mut, seeds = [WRITER_DAO_INFO_ACCOUNT_PREFIX.as_ref(), &writer.key().as_ref()], bump)]
  pub writer_info: Account<'info, WriterInfo>,

  pub writer: Signer<'info>,
}

pub fn grant_role(ctx: Context<WriterRole>, member: Pubkey, role: u8) -> Result<()> {
  let writer_info = &mut ctx.accounts.writer_info;
  writer_info.grant_role(member, role);
  Ok(())
}

pub fn revoke_role(ctx: Context<WriterRole>, member: Pubkey) -> Result<()> {
  let writer_info = &mut ctx.accounts.writer_info;
  writer_info.revoke_role(member);
  Ok(())
}