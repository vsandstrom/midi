pub mod connection;
pub mod note;
pub mod transport;
pub mod message;
pub mod util;
pub mod sequencer;

use std::sync::{Arc, Mutex};
use midir::SendError;
pub use midir::MidiOutputConnection;

pub mod consts {
  pub const MIDDLE_C:   u8 = 0x3c;
  pub const MS_IN_NANO: u32 = 1_000_000;
}

