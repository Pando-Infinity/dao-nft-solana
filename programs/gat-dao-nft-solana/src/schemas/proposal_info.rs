use anchor_lang::prelude::*;

use crate::helpers::MAX_PROPOSAL_DATA_SIZE;

#[account]
pub struct ProposalInfo {
    pub writer: Pubkey,
    pub id: u16,
    pub bump: u8,
    pub start_time: u64,
    pub end_time: u64,
    pub content: String,
    pub vote_by: u8,
    pub vote_option: Vec<VoteInfo>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VoteInfo {
    pub vote_content: String,
    pub amount: u64,
}

impl ProposalInfo {
    pub const SIZE: usize = MAX_PROPOSAL_DATA_SIZE + 8;
    pub fn can_vote(&self, vote_time: u64, vote_index: u16) -> bool {
        if vote_time > self.end_time
            || vote_time < self.start_time
            || vote_index + 1 > (self.vote_option.len() as u16)
        {
            return false;
        }
        return true;
    }
}
