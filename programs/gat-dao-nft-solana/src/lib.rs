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
}
