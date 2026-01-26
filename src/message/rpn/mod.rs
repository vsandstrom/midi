use super::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RpnKind{
  PitchBend      = 0x00,
  FineTune       = 0x01,
  CoarseTune     = 0x02,
  TuneProgChange = 0x03,
  TuneBankSel    = 0x04,
  ModDepthRange  = 0x05
}

pub struct Rpn  { pub addr: RpnKind, pub val: (u8, u8) }

impl MessageKind for Rpn {
  fn to_bytes(&self, ch: Channel) -> Vec<u8> {
    
    vec![
      CC|ch, RPN_MSB, self.addr as u8, 
      CC|ch, RPN_LSB, 0x00, 
      CC|ch, RPN_VAL_MSB, self.val.0,
      CC|ch, RPN_VAL_LSB, self.val.1,
      CC|ch, RPN_MSB, 127, // NULL
      CC|ch, RPN_LSB, 127, // NULL
    ]
  }

  #[inline]
  fn validate_address(&self) -> bool { true }
  
  #[inline]
  fn validate_value(&self) -> bool {
    self.val.0 < 128 && self.val.1 < 128
  }

  #[inline]
  fn repr(&self) -> String {
    format!("{} {}", self.val.0, self.val.1)
  }
  
  #[inline]
  fn repr_addr(&self) -> String { format!("{:?}", self.addr) }
}

impl Rpn {
  pub const MAX: u16 = 0x3fff;
}

impl FourteenBit for Rpn {
  fn split(num: u16) -> Result<(u8, u8), FourteenBitError> {
    if num & 0b1100_0000_0000_0000 != 0 { 
      return Err(FourteenBitError::Overflow(format!("Num {num} bigger than {}", Self::MAX)))
    }
    Ok(((num >> 7) as u8, (num & 0b0111_1111) as u8))
  }
}
