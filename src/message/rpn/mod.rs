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
      CC|ch, RPN_MSB, 0x00, RPN_LSB, self.addr as u8, 
      CC|ch, RPN_VAL_MSB, self.val.0, RPN_VAL_LSB, self.val.1,
      CC|ch, RPN_MSB, 127, CC|ch, RPN_LSB, 127 // NULL
    ]
  }

  #[inline]
  fn validate_address(&self) -> bool { true }
  
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
  fn repr_addr(&self) -> String { format!("{:?}", self.addr) }
}

impl FourteenBit for Rpn {}
