use super::*;

pub struct NrpnNoTerminator { pub addr: (u8,u8), pub val: (u8, u8) }

impl MessageKind for NrpnNoTerminator {
  fn to_bytes(&self, ch: Channel) -> Vec<u8> {
      vec![
        CC|ch, NRPN_MSB, self.addr.0, CC|ch, NRPN_LSB, self.addr.1, 
        CC|ch, NRPN_VAL_MSB, self.val.0, CC|ch, NRPN_VAL_LSB, self.val.1, 
        // CC|ch, NRPN_MSB, 127, CC|ch, NRPN_LSB, 127 // NULL 
      ]
  }

  #[inline]
  fn validate_address(&self) -> bool {
    self.addr.0 < 128 && self.addr.1 < 128
  }
  
  #[inline]
  fn validate_value(&self) -> bool {
    self.val.0 < 128 && self.val.1 < 128
  }

  #[inline]
  fn repr(&self) -> String {
    format!("{} {}", self.val.0, self.val.1)
  }
  
  #[inline]
  fn repr_addr(&self) -> String {
    format!("{} {}", self.addr.0, self.addr.1)
  }
}

impl NrpnNoTerminator {
  pub const MAX: u16 = 0x3fff;
}

impl FourteenBit for NrpnNoTerminator {
  fn split(num: u16) -> Result<(u8, u8), FourteenBitError> {
    if num & 0b1100_0000_0000_0000 != 0 { 
      return Err(FourteenBitError::Overflow(format!("Num {num} bigger than {}", Self::MAX)))
    }
    Ok(((num >> 7) as u8, (num & 0b0111_1111) as u8))
  }
}
