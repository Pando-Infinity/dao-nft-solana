pub const GOVERNANCE_DAO_INFO_ACCOUNT_PREFIX: &[u8] = b"governance-info-account";
pub const DAO_INFO_ACCOUNT_PREFIX: &[u8] = b"dao-info-account";
pub const WRITER_DAO_INFO_ACCOUNT_PREFIX: &[u8] = b"writer-info-account";
pub const PROPOSAL_INFO_ACCOUNT_PREFIX: &[u8] = b"proposal-info-account";
pub const VOTE_INFO_ACCOUNT_PREFIX: &[u8] = b"vote-info-account";
pub const UNRECLAIM_NFT_ACCOUNT_PREFIX: &[u8] = b"unreclaim-nft-info-account";

pub const MAX_MANAMENT_TEAM_SIZE: usize = 200;

pub const MAX_PROPOSAL_DATA_SIZE: usize = 5000;
pub const DEFAULT_PROPOSAL_CONTENT_LENGTH: u16 = 800;
pub const DEFAULT_VOTE_CONTENT_LENGTH: u16 = 300;
pub const DEFAULT_MIN_VOTE_OPTION_AMOUNT: u8 = 2;
pub const DEFAULT_MAX_VOTE_OPTION_AMOUNT: u8 = 10;

pub const MAX_UNRECLAIM_NFTS_SIZE: usize = 100;

pub const WRITER_CREATE_PROPOSAL_ROLE: u8 = 0;
