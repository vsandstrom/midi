use std::{
  sync::{atomic::AtomicBool, Arc, Mutex}, 
  time::{Duration, SystemTime}
};

use midir::MidiOutputConnection;

use spin_sleep::{
  SpinSleeper,
  SpinStrategy
};

const NRPN_MSB:   u8 = 0x63;
const NRPN_LSB:   u8 = 0x62;
const NRPN_DATA1: u8 = 0x06;
const NRPN_DATA2: u8 = 0x28;

const CC:         u8 = 0b10110000;
const START:      u8 = 0b11111010;
const STOP:       u8 = 0b11111100;
const CONTINUE:   u8 = 0b11111011;

pub struct Addr { msb: u8, lsb: u8 }
pub struct Data { coarse: u8, fine: u8 }

impl Addr {
  pub fn new(msb: u8, lsb: u8) -> Self { Self { msb, lsb } }
  pub fn repr(&self) -> String {
    format!("{}:{}", self.msb, self.lsb)
  }
}

impl Data {
  pub fn new(coarse: u8, fine: u8) -> Self { Self { coarse, fine } }
  pub fn repr(&self) -> String {
    format!("{} - {}", self.coarse, self.fine)
  }
}

pub fn cc(port: Arc<Mutex<MidiOutputConnection>>, ch: u8, addr: u8, val: u8) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[CC | ch, addr, val]).unwrap()
  }
}

pub fn nrpn(port: Arc<Mutex<MidiOutputConnection>>, ch: u8, addr: &Addr, val: &Data) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[(CC|ch), NRPN_MSB, addr.msb]).unwrap();
    p.send(&[(CC|ch), NRPN_LSB, addr.lsb]).unwrap();
    p.send(&[(CC|ch), NRPN_DATA1, val.coarse]).unwrap();
    p.send(&[(CC|ch), NRPN_DATA2, val.fine]).unwrap();
  }
}

pub fn start(port: Arc<Mutex<MidiOutputConnection>>) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[START]).unwrap()
  }
}

pub fn stop(port: Arc<Mutex<MidiOutputConnection>>) {
  if let Ok(mut p) = port.try_lock() {
    p.send(&[STOP]).unwrap();
  }
}

pub fn cont(port: Arc<Mutex<MidiOutputConnection>>) {
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
    spin_sleeper.sleep(dur - diff)
  }
}

fn calc_midi_ppq(bpm: f64) -> f64 { 60.0 / (28.0 * bpm) }

#[cfg(test)]
mod tests {
}
