use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
pub mod cross_programs;
pub mod helpers;
pub use helpers::*;
pub mod schemas;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod gat_dao_nft_solana {
    use super::*;

    pub fn governance_initialize(
        ctx: Context<GovernanceInitialize>,
        verifier: Pubkey,
    ) -> Result<()> {
        governance::initialize(ctx, verifier)
    }

    pub fn governance_kyc(ctx: Context<GovernanceKyc>) -> Result<()> {
        governance::kyc(ctx)
    }

    pub fn governance_ban(ctx: Context<GovernanceBan>, is_banned: bool) -> Result<()> {
        governance::ban(ctx, is_banned)
    }

    pub fn writer_dao_initialize(
        ctx: Context<WriterInitialize>,
        nft_collection_to_vote: Pubkey,
        token_to_vote: Pubkey,
    ) -> Result<()> {
        writers::dao_initialize(ctx, nft_collection_to_vote, token_to_vote)
    }

    pub fn writer_grant_role(ctx: Context<WriterRole>, member: Pubkey, role: u8) -> Result<()> {
        writers::grant_role(ctx, member, role)
    }

    pub fn writer_revoke_role(ctx: Context<WriterRole>, member: Pubkey) -> Result<()> {
        writers::revoke_role(ctx, member)
    }

    pub fn writer_create_proposal(
        ctx: Context<WriterCreateProposal>,
        proposal_number: u16,
        proposal_content: String,
        vote_option: Vec<String>,
        vote_by: u8,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        writers::create_proposal(
            ctx,
            proposal_number,
            proposal_content,
            vote_by,
            vote_option,
            start_time,
            end_time,
        )
    }

    pub fn reader_vote_by_nft(
        ctx: Context<ReaderVoteWithNft>,
        proposal_number: u16,
        vote_option_index: u16,
        vote_data: VoteData,
    ) -> Result<()> {
        readers::vote_with_nft(ctx, proposal_number, vote_option_index, vote_data)
    }

    pub fn reader_reclaim_nft(ctx: Context<ReaderReclaimNft>) -> Result<()> {
        readers::reclaim_nft(ctx)
    }
}
