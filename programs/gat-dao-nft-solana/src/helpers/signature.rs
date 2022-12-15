use crate::helpers::errors::ErrorCode;
use anchor_lang::{ prelude::*, solana_program };
use byteorder::ByteOrder;

struct Ed25519InstructionPart<'a> {
  address: &'a [u8],
  msg_offset: usize,
  msg_size: usize,
}

pub fn verify_signature(
  instruction_account: &AccountInfo,
  verifier: &Pubkey,
  data: Vec<u8>
) -> Result<bool> {
  let current_instruction = solana_program::sysvar::instructions::load_current_index_checked(
    &instruction_account
  )?;
  if current_instruction == 0 {
    return Err(error!(ErrorCode::InstructionAtWrongIndex));
  }
  let ed25519_ix_index = (current_instruction - 1) as u16;
  let ed25519_ix = match
    solana_program::sysvar::instructions::load_instruction_at_checked(
      ed25519_ix_index as usize,
      &instruction_account
    )
  {
    Ok(ix) => ix,
    Err(_) => {
      return Err(error!(ErrorCode::InvalidEd25519Instruction));
    }
  };
  if ed25519_ix.program_id != solana_program::ed25519_program::id() {
    return Err(error!(ErrorCode::InvalidEd25519ProgramId));
  }
  let ed25519_data_len = ed25519_ix.data.len();
  if ed25519_data_len < 2 {
    return Err(error!(ErrorCode::InvalidEd25519Instruction));
  }
  let sig_len = ed25519_ix.data[0];
  let mut index: usize = 0 + 2 + 2 + 2;
  let mut ed25519_ixs: Vec<Ed25519InstructionPart> = Vec::with_capacity(sig_len as usize);
  for _ in 0..sig_len {
    let address_offset = byteorder::LE::read_u16(&ed25519_ix.data[index..index + 2]) as usize;
    let address: &[u8] = &ed25519_ix.data[address_offset..address_offset + 32];
    index += 4;
    let msg_offset = byteorder::LE::read_u16(&ed25519_ix.data[index..index + 2]) as usize;
    index += 2;
    let msg_size = byteorder::LE::read_u16(&ed25519_ix.data[index..index + 2]) as usize;
    ed25519_ixs.push(Ed25519InstructionPart {
      address,
      msg_offset,
      msg_size,
    });
  }
  let message =
    &ed25519_ix.data
      [ed25519_ixs[0].msg_offset..ed25519_ixs[0].msg_offset + ed25519_ixs[0].msg_size];
  if verifier.to_bytes() != ed25519_ixs[0].address {
    return Err(error!(ErrorCode::InvalidSignature));
  }
  let data_hash = solana_program::hash::hash(&data);
  let message_hash = message.to_vec();

  Ok(message_hash.eq(&data_hash.try_to_vec()?))
}