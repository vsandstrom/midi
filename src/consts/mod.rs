/// Contains bitmasks and utility numbers for identifying and sending MIDI messages
/// ```
/// // example
///
/// pub const MIDDLE_C:      u8 = 0x3c;
/// // ...
/// pub mod note {
///    pub const NOTE_ON:    u8 = 0b10010000;
///    pub const NOTE_OFF:   u8 = 0b10000000;
/// }
/// ```
pub const MIDDLE_C:   u8 = 0x3c;
pub const MS_IN_NANO: u32 = 1_000_000;

pub mod note {
  pub const NOTE_ON:    u8 = 0b10010000;
  pub const NOTE_OFF:   u8 = 0b10000000;
}

pub mod transport {
  pub const START:      u8 = 0b11111010;
  pub const STOP:       u8 = 0b11111100;
  pub const CONTINUE:   u8 = 0b11111011;
  pub const CLOCK:      u8 = 0b11111000;
}

pub mod message {
  pub const NRPN_MSB:   u8 = 0x63;
  pub const NRPN_LSB:   u8 = 0x62;
  pub const RPN_MSB:    u8 = 0x65;
  pub const RPN_LSB:    u8 = 0x64;
  pub const NRPN_DATA1: u8 = 0x06;
  pub const NRPN_DATA2: u8 = 0x26;
  pub const CC:         u8 = 0xB0;
}
