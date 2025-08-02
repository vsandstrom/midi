use std::sync::atomic::AtomicBool;
use std::time::{Duration, SystemTime};
use crate::{Arc, Mutex, connection::Output,
  util::{
    logging::err_send_log,
    calc_midi_ppq
  },
  consts::transport::{START, STOP, CONTINUE, CLOCK}
};
/// re-export from spin_sleep crate
pub use spin_sleep::{SpinSleeper, SpinStrategy, sleep};

pub fn start(port: &Arc<Mutex<Output>>) {
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(&[START]));
  }
}

pub fn stop(port: &Arc<Mutex<Output>>) {
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(&[STOP]));
  }
}

pub fn cont(port: &Arc<Mutex<Output>>) {
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(&[CONTINUE]));
  }
}

pub fn clock(port: &Arc<Mutex<Output>>) {
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(&[CLOCK]));
  }
}

pub fn transport(port: &Arc<Mutex<Output>>, bpm: f64, run: Arc<AtomicBool>) {
  let dur = Duration::from_secs_f64(calc_midi_ppq(bpm));
  let spin_sleeper = SpinSleeper::new(10_000)
    .with_spin_strategy(SpinStrategy::YieldThread);

  'clock: loop {
    let now = SystemTime::now();
    if !run.load(std::sync::atomic::Ordering::Acquire) { break 'clock }
    if let Ok(mut p) = port.try_lock() {
      err_send_log(p.send(&[0b11111000]));
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

