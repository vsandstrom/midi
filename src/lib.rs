
/// Convenience module for creating Midi Input/Output connections
/// ```ignore
/// // Output: 
/// let port = midi::connection::Output::new("IAC Driver Bus 1").unwrap();
/// ```
/// ========
/// ```ignore
/// // Input:
/// use std::collection::VecDeque;
/// use std::sync::{Arc, Mutex};
/// let port = midi::connection::Input::new(
///         "IAC Driver Bus 1",
///         Arc<Mutex<VecDeque<Vec<u8>>>>,
///         |timecode, message, data| {
///           print!("{timecode}:: ");
///           message.iter().for_each(|byte| {print!("{byte:b}")});
///           println!();
///         }
///     ).unwrap();
/// ```
pub mod connection;
pub mod note;
pub mod transport;
pub mod message;
pub mod util;
// pub mod sequencer;
/// Contains bitmasks and utility numbers for identifying and sending MIDI messages
/// ```
/// // example
///
/// pub const MIDDLE_C:      u8 = 0x3c;
/// // ...
/// pub mod note {
///    pub const NOTE_ON:    u8 = 0b10010000;
///    pub const NOTE_OFF:   u8 = 0b10000000;
/// }
/// ```
pub mod consts;

use std::sync::{Arc, Mutex};
use midir::SendError;
pub use midir::{MidiOutputConnection, MidiInputConnection};


#[cfg(test)]
mod tests {
  use super::*;
  use std::time::Duration;
  use crate::transport::sleep;
  use self::message::cc::Cc;
  use self::message::Message;
  use crate::consts::{MIDDLE_C, MS_IN_NANO};
  use crate::connection::Output;
  use crate::util::Channel;

  #[test]
  fn test_cc() {
    let mut msg = Message::new(Cc{addr: 1, val: 100}).unwrap();
    let channel = Channel::new(0).unwrap();
    let _ = Output::new("IAC-drivrutin Buss 1", |port| {
      for i in 1..=10 {
        msg.send(&port, channel);
        sleep(Duration::new(0, MS_IN_NANO * 100));
        msg.update_value(100 + i).unwrap()
      }
    });
  }


  #[test]
  fn send_note() {
    use crate::message::{note::NoteOn, note::NoteOff};
    let note_on = Message::new(NoteOn{note: MIDDLE_C, velo: 100}).unwrap();
    let note_off = Message::new(NoteOff{note: MIDDLE_C}).unwrap();
    let channel = Channel::new(0).unwrap();
    let func = |port| {
      for _ in 0..100 {
        note_on.send(&port, channel); 
        sleep(Duration::new(0, MS_IN_NANO * 500));
        note_off.send(&port, channel); 
        sleep(Duration::new(0, MS_IN_NANO * 500));
      }
    };

    let port = Output::new("IAC Driver Bus 1", func);
  }

  #[test] 
  fn send_cc() {
    use crate::message::cc::Cc;
    let msg = Message::new( Cc{addr: 80, val: 100}).unwrap();
    let channel = Channel::new(0).unwrap();
    let func = |port| {
      for _ in 0..100 {
        msg.send(&port, channel); 
        sleep(Duration::new(0, MS_IN_NANO * 500));
      }
    };

    let port = Output::new("IAC Driver Bus 1", func);
  }
}
