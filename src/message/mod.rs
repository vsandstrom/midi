pub mod cc;
pub mod nrpn;
pub mod rpn;
pub mod sysex;
pub mod note;

use std::borrow::Cow;
use crate::{
  connection::Output, 
  consts::message::{
    CC,
    NRPN_LSB,
    NRPN_MSB,
    NRPN_VAL_LSB,
    NRPN_VAL_MSB,
    RPN_LSB,
    RPN_MSB,
    RPN_VAL_LSB,
    RPN_VAL_MSB,
  },
  consts::note::{NOTE_ON, NOTE_OFF, DEFAULT_NOTE_OFF_VEL},
  util::logging::err_send_log,
  Arc,
  Mutex
};

use cc::Cc;
use nrpn::Nrpn;
use rpn::{Rpn, RpnKind};
use sysex::SysEx;
use note::{NoteOff, NoteOn};

#[derive(Clone, Copy, Debug)]
pub struct Message<T: MessageKind> {
  kind: T
}


pub trait MessageKind {
  /// Returns a MIDI message formatted in bytes
  fn to_bytes(&self, ch: u8) -> Vec<u8>;
  /// Validates the MIDI message type Address
  fn validate_address(&self) -> bool;
  /// Validates the MIDI message type Value
  fn validate_value(&self) -> bool;
  /// Returns a string representation of the Value part of this particular MIDI message type
  fn repr(&self) -> String;
  /// Returns a string representation of the Address part of this particular MIDI message type
  fn repr_addr(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum MidiMessageError {
  Address(String),
  Value(String)
}

pub trait FourteenBit {
  fn split(num: u16) -> (u8, u8) {
    ((num >> 7) as u8, (num & 0b0111_1111) as u8)
  }
}


impl<T: MessageKind> Message<T> {
  pub fn new(kind: T) -> Result<Self, MidiMessageError> {
    if !T::validate_address(&kind) { 
      return Err(
        MidiMessageError::Address(
          format!("Invalid address: {}", T::repr_addr(&kind))
        )
      )
    }
    if !T::validate_value(&kind) { 
      return Err(
        MidiMessageError::Value(
          format!("Invalid value: {}", T::repr(&kind))
        )
      ) 
    }

    Ok(Self{
      kind
    })
  }

  /// Send a MIDI message
  /// Does not object at too big u8 values, external check advised.
  /// Will accept values bigger than 128 if ['Message<CC>'] 
  /// or bigger than (128, 128) if ['Message<Nrpn'],
  /// because the underlying ['MidiOutputConnection']
  /// from the ['midir'](https://github.com/Boddlnagg/midir) crate allows this. 
  pub fn message(&self, port: &Arc<Mutex<Output>>, ch: u8) { 
    let msg = T::to_bytes(&self.kind, ch);
    if let Ok(mut p) = port.try_lock() {
      err_send_log(p.send(&msg))
    } 
  }
}

impl Message<Cc> {
  pub fn update_value(&mut self, val: u8) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    self.kind.val = val;
    Ok(())
  }

  pub fn update(&mut self, addr: u8, val: u8) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    if !self.kind.validate_address() { 
      return Err(format!("Too big a value: {}", &self.kind.repr_addr()))
    }
    self.kind.addr = addr;
    self.kind.val = val;
    Ok(())
  }
}

impl Message<Nrpn> {
  pub fn update_value(&mut self, val: &(u8, u8)) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    self.kind.val = *val;
    Ok(())
  }

  pub fn update(&mut self, addr: &(u8, u8), val: &(u8, u8)) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    if !self.kind.validate_address() { 
      return Err(format!("Too big a value: {}", &self.kind.repr_addr()))
    }
    self.kind.addr = *addr;
    self.kind.val = *val;
    Ok(())
  }
}


impl Message<Rpn> {
  pub fn update_value(&mut self, val: &(u8, u8)) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    self.kind.val = *val;
    Ok(())
  }

  pub fn update(&mut self, addr: &RpnKind, val: &(u8, u8)) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }

    if !self.kind.validate_address() { 
      return Err(format!("Unathorized address: {}", &self.kind.repr_addr()))
    }
    self.kind.addr = *addr;
    self.kind.val = *val;
    Ok(())
  }
}

impl<'a> Message<SysEx<'a>> {
  /// Does not check for any errors in byte formatting. 
  ///
  pub fn update(&mut self, data: &'a [u8]) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Message does not contain SysEx start and end bytes: {}", &self.kind.repr()))
    }
    self.kind.data = Cow::Borrowed(data);
    Ok(())
  }
}

impl Message<NoteOn> {
  pub fn update_velocity(&mut self, velo: u8) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    self.kind.velo = velo;
    Ok(())
  }
  
  pub fn update_note(&mut self, note: u8) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    self.kind.note = note;
    Ok(())
  }

  pub fn update(&mut self, note: u8, velo: u8) -> Result<(), String> {
    if !self.kind.validate_value() {
      return Err(format!("Too big a value: {}", &self.kind.repr()))
    }
    if !self.kind.validate_address() { 
      return Err(format!("Too big a value: {}", &self.kind.repr_addr()))
    }
    self.kind.note = note;
    self.kind.velo = velo;
    Ok(())
  }
}

/// Sends an Cc message to the given Output. 
///
/// Contiuous Controller message
pub fn cc(port: &Arc<Mutex<Output>>, ch: u8, addr: u8, val: u8) {
  let msg = [CC|ch, addr, val];
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(&msg))
  } 
}

/// Sends an Nrpn message to the given Output. 
/// Non-registered Parameter Number message
pub fn nrpn(port: &Arc<Mutex<Output>>, ch: u8, addr: (u8, u8), val: (u8, u8)) {
  let msg = [
      CC|ch, NRPN_MSB, addr.0, CC|ch, NRPN_LSB, addr.1, 
      CC|ch, NRPN_VAL_MSB, val.0, CC|ch, NRPN_VAL_LSB, val.1, 
      CC|ch, NRPN_MSB, 127, CC|ch, NRPN_LSB, 127 // NULL 
    ];
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(&msg))
  } 
}


/// Sends an Rpn message to the given Output. 
/// It uses the enum RpnKind to choose which destination 
/// should receive the message.
///
/// Registered Parameter Number message
pub fn rpn(port: &Arc<Mutex<Output>>, ch: u8, addr: &RpnKind, val: (u8, u8)) {
  let msg = [
    CC|ch, RPN_MSB, 0x00, RPN_LSB, *addr as u8, 
    CC|ch, RPN_VAL_MSB, val.0, RPN_VAL_LSB, val.1,
    CC|ch, RPN_MSB, 127, CC|ch, RPN_LSB, 127 // NULL
  ];
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(&msg))
  } 

}

/// Sends a raw u8 byte array to the given Output. 
/// A SysEx message needs to be wrapped with the bytes 
/// `0xF0` - signalling start of message [`SYSEX_BEGIN`](midi::consts::SYSEX_BEGIN),
/// and `0xF7` - signalling the end. [`SYSEX_END`](midi::consts::SYSEX_END)
///
/// System Exclusive message
pub fn sysex(port: &Arc<Mutex<Output>>, data: &[u8]) {
  if let Ok(mut p) = port.try_lock() {
    err_send_log(p.send(data))
  } 
}



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
