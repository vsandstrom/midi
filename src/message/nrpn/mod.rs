use super::*;

pub struct Nrpn { pub addr: (u8,u8), pub val: (u8, u8) }

impl MessageKind for Nrpn {
  fn to_bytes(&self, ch: u8) -> Vec<u8> {
      vec![
        CC|ch, NRPN_MSB, self.addr.0, CC|ch, NRPN_LSB, self.addr.1, 
        CC|ch, NRPN_VAL_MSB, self.val.0, CC|ch, NRPN_VAL_LSB, self.val.1, 
        CC|ch, NRPN_MSB, 127, CC|ch, NRPN_LSB, 127 // NULL 
      ]
  }

  #[inline]
  fn validate_address(&self) -> bool {
    let (msb, lsb) = self.addr;
    msb < 128 && lsb < 128
  }
  
  #[inline]
  fn validate_value(&self) -> bool {
    let (coarse, fine) = self.val;
    coarse < 128 && fine < 128
  }

  #[inline]
  fn repr(&self) -> String {
      let (coarse, fine) = self.val;
      format!("{coarse} {fine}")
  }
  
  #[inline]
  fn repr_addr(&self) -> String {
      let (msb, lsb) = self.addr;
      format!("{msb} {lsb}")
  }
}

impl FourteenBit for Nrpn {}
