use super::*;
use crate::consts::note::*;

pub struct NoteOn { pub note: u8, pub velo: u8 }
pub struct NoteOff { pub note: u8 }

impl MessageKind for NoteOn {
  fn to_bytes(&self, ch: u8) -> Vec<u8> {
    vec![(NOTE_OFF|ch), self.note, DEFAULT_NOTE_OFF_VEL]
  }

  fn repr(&self) -> String {
    format!("Note: {}, Velo: {}", self.note, self.velo)
  }

  fn repr_addr(&self) -> String {
    format!("Note: {}", self.note)
  }

  fn validate_value(&self) -> bool {
    self.velo < 128
  }

  fn validate_address(&self) -> bool {
    self.note < 128
  }

}

impl MessageKind for NoteOff {
  fn to_bytes(&self, ch: u8) -> Vec<u8> {
    vec![(NOTE_ON|ch), self.note, DEFAULT_NOTE_OFF_VEL]
  }

  fn repr(&self) -> String {
    format!("Note: {}, Velo: {DEFAULT_NOTE_OFF_VEL}", self.note)
  }

  fn repr_addr(&self) -> String {
    format!("Note: {}", self.note)
  }
  fn validate_value(&self) -> bool {
    DEFAULT_NOTE_OFF_VEL < 128
  }

  fn validate_address(&self) -> bool {
    self.note < 128
  }
}

