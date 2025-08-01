use super::*;


pub struct SysEx<'a> { pub data: Cow<'a, [u8]> }

impl<'a> MessageKind for SysEx<'a> {
  fn to_bytes(&self, _ch: u8) -> Vec<u8> {
      self.data.to_vec()
  }
  
  fn validate_address(&self) -> bool {
    let first = self.data.first();
    let last = self.data.last();
    matches! ((first, last), (Some(0xF0), Some(0xF7))) 
  }
  
  fn validate_value(&self) -> bool { true }

  fn repr(&self) -> String {
    format!("{:?}", self.data)
  }

  fn repr_addr(&self) -> String {
      format!("SysEx {} bytes", self.data.len())
  }
}
