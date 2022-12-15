use super::AccessControl;
use crate::helpers::MAX_MANAMENT_TEAM_SIZE;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[account]
pub struct GovernanceInfo {
    pub admin: Pubkey,
    pub manager_team: AccessControl<Pubkey, u8>,
}

impl GovernanceInfo {
    pub const SIZE: usize = size_of::<Pubkey>() * MAX_MANAMENT_TEAM_SIZE + 3 + 32 + 8;
}
