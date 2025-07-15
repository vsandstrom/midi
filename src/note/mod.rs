use crate::{
  Arc, Mutex, MidiOutputConnection,
  util::logging::err_send_log
};

const NOTE_ON:    u8 = 0b10010000;
const NOTE_OFF:   u8 = 0b10000000;

pub fn note_on(port: &Arc<Mutex<MidiOutputConnection>>, ch: u8, note: u8, velo: u8) {
  if let Ok(mut p) = port.try_lock() { 
    err_send_log(p.send(&[(NOTE_ON|ch), note, velo]));
  }
}

pub fn note_off(port: &Arc<Mutex<MidiOutputConnection>>, ch: u8, note: u8) {
  if let Ok(mut p) = port.try_lock() { 
    err_send_log(p.send(&[(NOTE_OFF|ch), note, 0]));
  }
}
