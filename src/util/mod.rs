use crate::SendError;

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

