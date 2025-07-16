use crate::{
  Arc, Mutex, MidiOutputConnection,
  util::logging::err_send_log,
  connection::Output
};

use std::marker::PhantomData;
use std::result;

const NRPN_MSB:   u8 = 0x63;
const NRPN_LSB:   u8 = 0x62;
const RPN_MSB:    u8 = 0x65;
const RPN_LSB:    u8 = 0x64;
const NRPN_DATA1: u8 = 0x06;
const NRPN_DATA2: u8 = 0x26;
const CC:         u8 = 0xB0;

#[derive(Debug, Clone)]
pub enum MidiMessageError {
  Address(String),
  Value(String)
}

pub struct Nrpn;
pub struct Rpn;
pub struct Cc;

pub trait MessageKind {
  type Addr;
  type Val;
  /// Returns a MIDI message formatted in bytes
  fn to_bytes(addr: &Self::Addr, val: &Self::Val, ch: u8) -> Vec<u8>;
  /// Validates the MIDI message type Address
  fn validate_address(addr: &Self::Addr) -> bool;
  /// Validates the MIDI message type Value
  fn validate_value(val: &Self::Val) -> bool;
  /// Returns a string representation of the Value part of this particular MIDI message type
  fn repr(val: &Self::Val) -> String;
  /// Returns a string representation of the Address part of this particular MIDI message type
  fn repr_addr(addr: &Self::Addr) -> String;

}

pub trait FourteenBit {
  fn split(num: u16) -> (u8, u8) {
    ((num >> 7) as u8, (num & 0b0111_1111) as u8)
  }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RpnKind{
  PitchBend      = 0x00,
  FineTune       = 0x01,
  CoarseTune     = 0x02,
  TuneProgChange = 0x03,
  TuneBankSel    = 0x04,
  ModDepthRange  = 0x05
}

pub struct Message<T: MessageKind> {
addr: T::Addr,
  val: T::Val,
  _marker: PhantomData<T>
}


impl MessageKind for Nrpn {
  type Addr = (u8, u8);
  type Val = (u8, u8);
  fn to_bytes(addr: &Self::Addr, val: &Self::Val, ch: u8) -> Vec<u8> {
      vec![
        CC|ch, NRPN_MSB, addr.0, 
        CC|ch, NRPN_LSB, addr.1, 
        CC|ch, NRPN_DATA1, val.0, 
        CC|ch, NRPN_DATA2, val.1, 
      ]
  }

  #[inline]
  fn validate_address(addr: &Self::Addr) -> bool {
    let (msb, lsb) = addr;
    *msb < 128 && *lsb < 128
  }
  
  #[inline]
  fn validate_value(val: &Self::Val) -> bool {
    let (coarse, fine) = val;
    *coarse < 128 && *fine < 128
  }

  #[inline]
  fn repr(val: &Self::Val) -> String {
      let (coarse, fine) = val;
      format!("{coarse} {fine}")
  }
  
  #[inline]
  fn repr_addr(addr: &Self::Addr) -> String {
      let (msb, lsb) = addr;
      format!("{msb} {lsb}")
  }
}

impl FourteenBit for Nrpn {}

impl MessageKind for Rpn {
  type Addr = RpnKind;
  type Val = (u8, u8);
  fn to_bytes(addr: &Self::Addr, val: &Self::Val, ch: u8) -> Vec<u8> {
    vec![
      CC|ch, RPN_MSB, 0x00, RPN_LSB, *addr as u8, 
      CC|ch, NRPN_DATA1, val.0, NRPN_DATA2, val.1 
    ]
  }

  #[inline]
  fn validate_address(_addr: &Self::Addr) -> bool {
    true
  }
  
  #[inline]
  fn validate_value(val: &Self::Val) -> bool {
    let (coarse, fine) = val;
    *coarse < 128 && *fine < 128
  }

  #[inline]
  fn repr(val: &Self::Val) -> String {
    let (coarse, fine) = val;
    format!("{coarse} {fine}")
  }
  
  #[inline]
  fn repr_addr(addr: &Self::Addr) -> String {
      format!("{addr:?}")
  }
}

impl FourteenBit for Rpn {}

impl MessageKind for Cc {
  type Addr = u8;
  type Val = u8;
  fn to_bytes(addr: &Self::Addr, val: &Self::Val, ch: u8) -> Vec<u8> {
      vec![CC|ch, *addr, *val]
  }
  
  fn validate_address(addr: &Self::Addr) -> bool {
    *addr < 128
  }
  
  fn validate_value(val: &Self::Val) -> bool {
    *val < 128
  }

  fn repr(val: &Self::Val) -> String {
    format!("{val}")
  }

  fn repr_addr(addr: &Self::Addr) -> String {
    format!("{addr}")
  }
}

impl<T: MessageKind> Message<T> {
  pub fn new(addr: T::Addr, val: T::Val) -> Result<Self, MidiMessageError> {
    if !T::validate_address(&addr) { 
      return Err(
        MidiMessageError::Address(
          format!("Invalid address: {}", T::repr_addr(&addr))
        )
      )
    }
    if !T::validate_value(&val) { 
      return Err(
        MidiMessageError::Value(
          format!("Invalid value: {}", T::repr(&val))
        )
      ) 
    }

    Ok(Self{
      addr,
      val,
      _marker: PhantomData
    })
  }

  /// Send a MIDI message
  /// Does not object at too big u8 values, external check advised.
  /// Will accept values bigger than 128 if ['Message<CC>'] 
  /// or bigger than (128, 128) if ['Message<Nrpn'],
  /// because the underlying ['MidiOutputConnection']
  /// from the ['midir'](https://github.com/Boddlnagg/midir) crate allows this. 
  pub fn message(&self, port: &Arc<Mutex<Output>>, ch: u8) { 
    let msg = T::to_bytes(&self.addr, &self.val, ch);
    if let Ok(mut p) = port.try_lock() {
      err_send_log(p.send(&msg))
    } 
  }

  pub fn update(&mut self, val: T::Val) -> Result<(), String> {
    if !T::validate_value(&val) {
      return Err(format!("Too big a value: {}", T::repr(&val)))
    }
    self.val = val;
    Ok(())
  }
}


