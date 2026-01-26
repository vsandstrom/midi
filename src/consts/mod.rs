/// Contains bitmasks and utility numbers for identifying and sending MIDI messages
/// ```
/// // example
///
/// pub const MIDDLE_C:       u8 = 0x3c;
/// // ...
/// pub mod note {
///    pub const NOTE_ON:     u8 = 0b10010000;
///    pub const NOTE_OFF:    u8 = 0b10000000;
/// }
/// ```
pub const MIDDLE_C:           u8 = 0x3c;
pub const MS_IN_NANO:         u32 = 1_000_000;

pub mod note {
  pub const NOTE_ON:          u8 = 0b10010000;
  pub const NOTE_OFF:         u8 = 0b10000000;
  pub(crate) const DEFAULT_NOTE_OFF_VEL: u8 = 64;
}

pub mod transport {
  pub const START:            u8 = 0b11111010;
  pub const STOP:             u8 = 0b11111100;
  pub const CONTINUE:         u8 = 0b11111011;
  pub const CLOCK:            u8 = 0b11111000;
}

pub mod message {
  // Address : Most valuable byte
  pub const NRPN_MSB:         u8 = 0x63;
  // Address : Least valuable byte
  pub const NRPN_LSB:         u8 = 0x62;
  // Address : Most valuable byte
  pub const RPN_MSB:          u8 = 0x65;
  // Address : Least valuable byte
  pub const RPN_LSB:          u8 = 0x64;
  // Value : Most valuable byte
  pub const NRPN_VAL_MSB:     u8 = 0x06;
  // Value : Least valuable byte
  pub const NRPN_VAL_LSB:     u8 = 0x26;
  // Value : Most valuable byte
  pub const RPN_VAL_MSB:      u8 = NRPN_VAL_MSB;
  // Value : Least valuable byte
  pub const RPN_VAL_LSB:      u8 = NRPN_VAL_LSB;
  // Control Change
  pub const CC:               u8 = 0xB0;
  /// Pitchbend
  pub const PB:               u8 = 0xE0;
  // SysEx Begin sequence
  pub const SYSEX_BEGIN:      u8 = 0xF0;
  // SysEx End sequence
  pub const SYSEX_END:        u8 = 0xF7;
}

