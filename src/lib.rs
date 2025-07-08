pub mod connection;

use std::{
  sync::{atomic::AtomicBool, Arc, Mutex}, 
  time::{Duration, SystemTime}
};

pub use midir::{MidiOutput, MidiOutputConnection, ConnectError, SendError};

pub use spin_sleep::{
  SpinSleeper,
  SpinStrategy
};

const NRPN_MSB:   u8 = 0x63;
const NRPN_LSB:   u8 = 0x62;
const NRPN_DATA1: u8 = 0x06;
const NRPN_DATA2: u8 = 0x26;

const CC:         u8 = 0b10110000;
const NOTE_ON:    u8 = 0b10010000;
const NOTE_OFF:   u8 = 0b10000000;
const START:      u8 = 0b11111010;
const STOP:       u8 = 0b11111100;
const CONTINUE:   u8 = 0b11111011;

pub mod consts {
  pub const MIDDLE_C:   u8 = 0x3c;
  pub const MS_IN_NANO: u32 = 1_000_000;
}
/// Contains the address component of an NRPN message
/// It is up to the user to use valid numbers [0 - 127]
pub struct Addr { msb: u8, lsb: u8 }

/// Contains the data component of an NRPN message
/// It is up to the user to use valid numbers [0 - 127]
pub struct Data { coarse: u8, fine: u8 }

impl Addr {
  pub fn new(msb: u8, lsb: u8) -> Self { Self { msb, lsb } }
  pub fn repr(&self) -> String {
    format!("{}:{}", self.msb, self.lsb)
  }
}

impl Data {
  pub fn new(coarse: u8, fine: u8) -> Self { Self { coarse, fine } }

  pub fn from(num: u16) -> Self { 
    let nrpn = Self::split(num);
    Self { coarse: nrpn.0, fine: nrpn.1 } 
  }

  /// Returns Err if `num` is too big to fit in an NRPN message
  /// i.e. bigger than or equal to 16383.
  pub fn update(&mut self, num: u16) -> Result<(u8, u8), &str> { 
    if num >= 16383 {return Err("Value is too big for NRPN")};
    self.coarse = (num >> 7) as u8;
    self.fine = (num & 0b0111_1111) as u8;
    Ok((self.coarse, self.fine))
  }

  pub fn repr(&self) -> String {
    format!("{} - {}", self.coarse, self.fine)
  }

  fn split(num: u16) -> (u8, u8) {
    ((num >> 7) as u8, (num & 0b0111_1111) as u8)
  }

  pub fn get(&self) -> (u8, u8) {
    (self.coarse, self.fine)
  }
}



pub fn fetch_output_port(name: &str) -> Result<MidiOutputConnection, ConnectError<MidiOutput>> {
  let output = MidiOutput::new("cpu").expect("could not create MIDI output");
  let ports = output.ports();
  
  let port = ports
    .iter()
    .find(|p| name == output
      .port_name(p)
      .unwrap())
    .expect("digitakt is not available as midi output");
  
  output.connect(port, name)
}

/// Send a MIDI message
pub fn cc(port: &Arc<Mutex<MidiOutputConnection>>, ch: u8, addr: u8, val: u8) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[CC | ch, addr, val]).unwrap()
  }
}

/// Sends a string of 4 midi messages, manifesting as NRPN message.
pub fn nrpn(port: &Arc<Mutex<MidiOutputConnection>>, ch: u8, addr: &Addr, val: &Data) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[(CC|ch), NRPN_MSB, addr.msb]).unwrap();
    p.send(&[(CC|ch), NRPN_LSB, addr.lsb]).unwrap();
    p.send(&[(CC|ch), NRPN_DATA1, val.coarse]).unwrap();
    p.send(&[(CC|ch), NRPN_DATA2, val.fine]).unwrap();
  }
}

pub fn note_on(port: &Arc<Mutex<MidiOutputConnection>>, ch: u8, note: u8, velo: u8) {
  if let Ok(mut p) = port.try_lock() { 
    p.send(&[(NOTE_ON|ch), note, velo]).unwrap()
  }
}

pub fn note_off(port: &Arc<Mutex<MidiOutputConnection>>, ch: u8, note: u8) {
  if let Ok(mut p) = port.try_lock() { 
    p.send(&[(NOTE_OFF|ch), note, 0]).unwrap()
  }
}

pub fn start(port: &Arc<Mutex<MidiOutputConnection>>) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[START]).unwrap()
  }
}

pub fn stop(port: &Arc<Mutex<MidiOutputConnection>>) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[STOP]).unwrap();
  }
}

pub fn cont(port: &Arc<Mutex<MidiOutputConnection>>) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[CONTINUE]).unwrap();
  }
}

pub fn clock(port: Arc<Mutex<MidiOutputConnection>>, bpm: f64, run: Arc<AtomicBool>) {
  let dur = Duration::from_secs_f64(calc_midi_ppq(bpm));
  let spin_sleeper = SpinSleeper::new(10_000)
    .with_spin_strategy(SpinStrategy::YieldThread);

  'clock: loop {
    let now = SystemTime::now();
    if !run.load(std::sync::atomic::Ordering::Acquire) { break 'clock }
    if let Ok(mut p) = port.try_lock() {
      p .send(&[0b11111000]) .unwrap()
    }
    let diff = SystemTime::now().duration_since(now).unwrap();

    if diff > dur {
      let new_diff = diff.div_f64(dur.as_secs_f64()).mul_f64(dur.as_secs_f64());

      spin_sleeper.sleep(new_diff)
    } else {
      spin_sleeper.sleep(dur - diff)
    }
  }
}

fn calc_midi_ppq(bpm: f64) -> f64 { 60.0 / (28.0 * bpm) }

#[cfg(test)]
mod tests {
}
