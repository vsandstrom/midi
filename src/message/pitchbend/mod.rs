use super::*;

pub struct PitchBend { pub msb: u8, pub lsb: u8 }

impl MessageKind for PitchBend {
  fn to_bytes(&self, ch: Channel) -> Vec<u8> {
      vec![PB|ch, self.msb, self.lsb]
  }

  #[inline]
  fn validate_address(&self) -> bool { self.msb < 128 && self.lsb < 128 }
  
  #[inline]
  fn validate_value(&self) -> bool { self.msb < 128 && self.lsb < 128 }

  #[inline]
  fn repr(&self) -> String { format!("{} {}", self.msb, self.lsb) }
  
  #[inline]
  fn repr_addr(&self) -> String { format!("{} {}", self.msb, self.lsb) }
}
