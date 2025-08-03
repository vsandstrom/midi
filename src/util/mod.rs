use crate::SendError;

pub struct Channel(u8);
impl Channel {
  fn new(channel: u8) -> Result<Self, String> {
    if channel < 16 {return Ok(Channel(channel))}
    Err("Channel is not a value between 0 an 15.".to_string())
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

