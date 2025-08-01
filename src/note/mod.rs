use crate::{
  connection::Output,
  consts::note::{NOTE_OFF, NOTE_ON, DEFAULT_NOTE_OFF_VEL},
  util::logging::err_send_log,
  Arc,
  Mutex
};


/// sends a NOTE ON message with channel, note and velocity data. 
pub fn note_on(port: &Arc<Mutex<Output>>, ch: u8, note: u8, velo: u8) {
  if let Ok(mut p) = port.try_lock() { 
    err_send_log(p.send(&[(NOTE_ON|ch), note, velo]));
  }
}

/// sends a NOTE OFF message with channel and note data. 
/// velocity is omitted, since it is seldom used. 
///
/// (a velocity of 64 is sent in the byte message, as is tradition)
pub fn note_off(port: &Arc<Mutex<Output>>, ch: u8, note: u8) {
  if let Ok(mut p) = port.try_lock() { 
    err_send_log(p.send(&[(NOTE_OFF|ch), note, DEFAULT_NOTE_OFF_VEL]));
  }
}
