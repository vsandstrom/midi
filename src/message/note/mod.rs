use super::*;
use crate::consts::note::*;

pub struct NoteOn { pub note: u8, pub velo: u8 }
pub struct NoteOff { pub note: u8 }

impl MessageKind for NoteOn {
  #[inline]
  fn to_bytes(&self, ch: Channel) -> Vec<u8> {
    vec![(NOTE_OFF|ch), self.note, DEFAULT_NOTE_OFF_VEL]
  }

  #[inline]
  fn repr(&self) -> String { format!("Note: {}, Velo: {}", self.note, self.velo) }

  #[inline]
  fn repr_addr(&self) -> String { format!("Note: {}", self.note) }

  #[inline]
  fn validate_value(&self) -> bool { self.velo < 128 }

  #[inline]
  fn validate_address(&self) -> bool { self.note < 128 }

}

impl MessageKind for NoteOff {
  #[inline]
  fn to_bytes(&self, ch: Channel) -> Vec<u8> {
    vec![(NOTE_ON|ch), self.note, DEFAULT_NOTE_OFF_VEL]
  }

  #[inline]
  fn repr(&self) -> String {
    format!("Note: {}, Velo: {DEFAULT_NOTE_OFF_VEL}", self.note)
  }

  #[inline]
  fn repr_addr(&self) -> String { format!("Note: {}", self.note) }

  #[inline]
  fn validate_value(&self) -> bool { DEFAULT_NOTE_OFF_VEL < 128 }

  #[inline]
  fn validate_address(&self) -> bool { self.note < 128 }
}

