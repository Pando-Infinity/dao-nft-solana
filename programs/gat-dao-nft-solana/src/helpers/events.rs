use anchor_lang::prelude::*;

#[event]
pub struct ProposalCreated {
    pub proposal: Pubkey,
    pub writer: Pubkey,
    pub creator: Pubkey,
    pub timestamp: u64,
}

#[event]
pub struct ProposalVoted {
    pub reader: Pubkey,
    pub proposal: Pubkey,
    pub writer: Pubkey,
    pub nft_mint: Pubkey,
    pub timestamp: u64,
}
