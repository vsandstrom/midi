
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


