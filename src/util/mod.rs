use crate::SendError;
use std::ops::BitOr;

#[derive(Clone, Copy)]
/// Wrapper around a `u8` that represents the MIDI channel. 
/// Will make sure that channel is within a range of 
/// 0 - 15, representing 16 channels. 
pub struct Channel(pub u8);
impl Channel {
  pub fn new(channel: u8) -> Result<Self, String> {
    if 0b11110000 & channel != 0 {
      Ok(Channel(channel))
    } else {
      Err("Channel is not a value between 0 an 15.".to_string())
    }
  }
}

impl From<Channel> for u8 {
  fn from(value: Channel) -> Self {
    value.0
  }
}

impl BitOr<u8> for Channel {
  type Output = u8;
  fn bitor(self, rhs: u8) -> Self::Output {
    self.0 | rhs
  }
}

impl BitOr<Channel> for u8 {
  type Output = u8;
  fn bitor(self, rhs: Channel) -> Self::Output {
    self | rhs.0
  }
}

impl BitOr for Channel {
  type Output = u8;
  fn bitor(self, rhs: Self) -> Self::Output {
    self.0 | rhs.0
  }
}

pub(crate) fn calc_midi_ppq(bpm: f64) -> f64 { 60.0 / (28.0 * bpm) }

pub mod logging {
  use super::*;
  pub(crate) fn err_send_log(err: Result<(), SendError>) {
    match err {
      Err(SendError::InvalidData(e)) => {
        eprintln!("Error type: Invalid data -  {e}");
        std::process::exit(-2)
      },
      Err(SendError::Other(e)) => {
        eprintln!("Error type: Other -  {e}");
        std::process::exit(-3)
      },
    _ => () 
    }
  }

  pub(crate) fn err_log(e: String) -> ! {
    eprintln!("{e}");
    std::process::exit(-1)
  }

}

