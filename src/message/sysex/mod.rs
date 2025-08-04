use super::*;


pub struct SysEx<'a> { pub data: Cow<'a, [u8]> }

impl<'a> MessageKind for SysEx<'a> {
  #[inline]
  fn to_bytes(&self, _ch: Channel) -> Vec<u8> {
      self.data.to_vec()
  }
  
  #[inline]
  fn validate_address(&self) -> bool {
    let first = self.data.first();
    let last = self.data.last();
    matches! ((first, last), (Some(0xF0), Some(0xF7))) 
  }
  
  #[inline]
  fn validate_value(&self) -> bool { true }

  #[inline]
  fn repr(&self) -> String {
    format!("{:?}", self.data)
  }

  #[inline]
  fn repr_addr(&self) -> String {
      format!("SysEx {} bytes", self.data.len())
  }
}
