use std::mem::size_of;

use super::AccessControl;
use crate::helpers::MAX_MANAMENT_TEAM_SIZE;
use anchor_lang::prelude::*;

#[account]
pub struct WriterInfo {
  pub is_kyc: bool,
  pub is_banned: bool,
  pub next_proposal_id: u16,
  pub token_to_vote: Pubkey,
  pub nft_collection_to_vote: Pubkey,
  pub manager_team: AccessControl<Pubkey, u8>,
}

impl WriterInfo {
  pub const SIZE: usize = size_of::<Pubkey>() * MAX_MANAMENT_TEAM_SIZE + 68 + 3 + 8;
}

pub trait WriterInfoAccountBehavior {
  fn grant_role(&mut self, member: Pubkey, role: u8);
  fn revoke_role(&mut self, member: Pubkey);
  fn has_role(&self, member: Pubkey, role: u8) -> bool;
  fn verify(&self) -> bool;
  fn is_valid_proposal_id(&self, proposal_id: u16) -> bool;
  fn set_next_proposal_id(&mut self, proposal_id: u16);
}

impl<'info> WriterInfoAccountBehavior for Account<'info, WriterInfo> {
  fn grant_role(&mut self, member: Pubkey, role: u8) {
    self.manager_team.grant_role(member, role);
  }
  fn revoke_role(&mut self, member: Pubkey) {
    self.manager_team.revoke_role(member);
  }
  fn has_role(&self, member: Pubkey, role: u8) -> bool {
    return self.manager_team.has_role(member, role);
  }
  fn verify(&self) -> bool {
    if self.is_banned == true || self.is_kyc == false {
      return false;
    }
    return true;
  }
  fn is_valid_proposal_id(&self, proposal_id: u16) -> bool {
    return proposal_id == self.next_proposal_id;
  }
  fn set_next_proposal_id(&mut self, proposal_id: u16) {
    self.next_proposal_id = proposal_id;
  }
}