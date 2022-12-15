use crate::helpers::errors::ErrorCode;

#[derive(PartialEq)]
pub enum VoteBy {
  TOKEN = 0,
  NFT = 1,
}

impl TryFrom<u8> for VoteBy {
  type Error = ErrorCode;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(VoteBy::TOKEN),
      1 => Ok(VoteBy::NFT),
      _ => Err(ErrorCode::InvalidVoteByValue.into()),
    }
  }
}