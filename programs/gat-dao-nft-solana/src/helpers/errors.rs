use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("GatDao: Invalid vote by value")]
  InvalidVoteByValue,

  #[msg("GatDao: Unverified writer")]
  UnverifiedWriter,

  #[msg("GatDao: Permission required")]
  PermissionRequired,

  #[msg("GatDao: Invalid proposal parameters")]
  InvalidProposalParameter,

  #[msg("GatDao: Cannot vote now")]
  CannotVotNow,

  #[msg("GatDao: Already voted")]
  AlreadyVoted,

  #[msg("GatDao: Invalid vote data")]
  InvalidVoteData,

  #[msg("GatDao: Nft not initialized")]
  NftNotInitialized,

  #[msg("GatDao: Nft creator not match")]
  NftCreatorNotMatch,

  #[msg("GatDao: Nft collection not match")]
  NftCollectionNotMatch,

  #[msg("GatDao: Nft not available to reclaim")]
  NftNotAvailableToReclaim,

  #[msg("GatDao: Can not reclaim in vote time")]
  CannotReclaimInVoteTime,

  #[msg("GatDao: Nft reclaimed")]
  NftReclaimed,

  #[msg("GatDao: Nft reclaim not match")]
  NftReclaimNotMatch,

  #[msg("GatDao: Invalid ed25519 instruction")]
  InvalidEd25519Instruction,

  #[msg("GatDao: Invalid ed25519 program id")]
  InvalidEd25519ProgramId,

  #[msg("GatDao: Invalid signature")]
  InvalidSignature,

  #[msg("GatDao: Instruction at wrong index")]
  InstructionAtWrongIndex,
}