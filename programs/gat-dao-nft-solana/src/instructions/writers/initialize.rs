use crate::helpers::constants::WRITER_DAO_INFO_ACCOUNT_PREFIX;
use crate::schemas::writer_info::WriterInfo;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct WriterInitialize<'info> {
  #[account(
    init,
    seeds = [WRITER_DAO_INFO_ACCOUNT_PREFIX.as_ref(), &writer.key().as_ref()],
    payer = writer,
    space = WriterInfo::SIZE,
    bump
  )]
  pub writer_info: Account<'info, WriterInfo>,

  #[account(mut)]
  pub writer: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub fn dao_initialize(
  ctx: Context<WriterInitialize>,
  nft_collection_to_vote: Pubkey,
  token_to_vote: Pubkey
) -> Result<()> {
  let writer_info = &mut ctx.accounts.writer_info;
  writer_info.is_kyc = false;
  writer_info.is_banned = false;
  writer_info.next_proposal_id = 0;
  writer_info.token_to_vote = token_to_vote;
  writer_info.nft_collection_to_vote = nft_collection_to_vote;
  Ok(())
}