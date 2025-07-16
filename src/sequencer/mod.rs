use crate::*;

struct Note {
  ch: u8, 
  note: u8,
  vel: u8
}

struct Step {
  note: Vec<Note>,
}

pub struct Sequencer<const N: usize> {
  steps: [Step; N]
}
