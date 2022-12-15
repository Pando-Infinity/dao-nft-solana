use anchor_lang::prelude::*;
use anchor_spl::token;
use mpl_token_metadata::state::Metadata;

pub struct Nft {}

impl Nft {
  pub fn transfer<'info>(
    sender_token_account: AccountInfo<'info>,
    receiver_token_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>
  ) -> Result<()> {
    let transfer_ctx: CpiContext<token::Transfer> = CpiContext::new(token_program, token::Transfer {
      from: sender_token_account,
      to: receiver_token_account,
      authority,
    });
    token::transfer(transfer_ctx, 1)
  }

  pub fn transfer_with_seed<'info>(
    sender_token_account: AccountInfo<'info>,
    receiver_token_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    seed: &[&[&[u8]]]
  ) -> Result<()> {
    let transfer_ctx: CpiContext<token::Transfer> = CpiContext::new_with_signer(
      token_program,
      token::Transfer {
        from: sender_token_account,
        to: receiver_token_account,
        authority,
      },
      seed
    );
    token::transfer(transfer_ctx, 1)
  }

  pub fn verify_creator(metadata: &Metadata, creator: &Pubkey) -> bool {
    if
      metadata.data.creators.as_ref().unwrap()[0].address == creator.to_owned() &&
      metadata.data.creators.as_ref().unwrap()[0].verified
    {
      return true;
    }
    return false;
  }

  pub fn verify_collection(metadata: &Metadata, collection: &Pubkey) -> bool {
    if
      metadata.collection.as_ref().unwrap().key == collection.to_owned() &&
      metadata.collection.as_ref().unwrap().verified
    {
      return true;
    }
    return false;
  }
}