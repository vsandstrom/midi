
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
use self::connection::Output;
use std::time::Duration;
use self::consts::MS_IN_NANO;
use crate::transport::sleep;
use self::message::cc::Cc;
use self::message::Message;

  #[test]
  fn test_cc() {
    let mut msg = Message::new(Cc{addr: 1, val: 100}).unwrap();
    let _ = Output::new("IAC-drivrutin Buss 1", |port| {
      for i in 1..=10 {
        msg.send(&port, 0);
        sleep(Duration::new(0, MS_IN_NANO * 100));
        msg.update_value(100 + i).unwrap()
      }
    });
  }
}
