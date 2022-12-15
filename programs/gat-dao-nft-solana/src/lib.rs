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
}
