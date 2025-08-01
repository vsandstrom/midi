use super::*;

pub struct Cc { pub addr: u8, pub val: u8 }

impl MessageKind for Cc {
  fn to_bytes(&self, ch: u8) -> Vec<u8> {
      vec![CC|ch, self.addr, self.val]
  }
  
  fn validate_address(&self) -> bool {
    self.addr < 128
  }
  
  fn validate_value(&self) -> bool {
    self.val < 128
  }

  fn repr(&self) -> String {
    format!("{}", self.val)
  }

  fn repr_addr(&self) -> String {
    format!("{}", self.addr)
  }
}

