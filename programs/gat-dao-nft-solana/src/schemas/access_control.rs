use anchor_lang::prelude::*;

#[account]
pub struct Member<
  T: anchor_lang::AnchorSerialize + anchor_lang::AnchorDeserialize,
  G: anchor_lang::AnchorSerialize + anchor_lang::AnchorDeserialize
> {
  pub member: T,
  pub role: G,
}

#[account]
pub struct AccessControl<
  T: anchor_lang::AnchorSerialize + anchor_lang::AnchorDeserialize,
  G: anchor_lang::AnchorSerialize + anchor_lang::AnchorDeserialize
> {
  pub role_data: Vec<Member<T, G>>,
}

impl<T, G> AccessControl<T, G>
  where
    T: PartialEq + anchor_lang::AnchorSerialize + anchor_lang::AnchorDeserialize,
    G: PartialEq + anchor_lang::AnchorSerialize + anchor_lang::AnchorDeserialize
{
  pub fn grant_role(&mut self, member: T, role: G) {
    let member_index = self.role_data.iter().position(|x| x.member == member);
    if member_index.is_none() {
      self.role_data.push(Member { member, role });
      return;
    }
    self.role_data[member_index.unwrap() as usize].role = role;
  }

  pub fn revoke_role(&mut self, member: T) {
    let member_index = self.role_data.iter().position(|x| x.member == member);
    if !member_index.is_none() {
      self.role_data.remove(member_index.unwrap() as usize);
    }
  }

  pub fn has_role(&self, member: T, role: G) -> bool {
    let member_index = self.role_data.iter().position(|x| x.member == member);
    if !member_index.is_none() {
      return self.role_data[member_index.unwrap() as usize].role == role;
    }
    return false;
  }
}