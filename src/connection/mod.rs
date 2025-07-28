use std::marker::PhantomData;

use crate::{
  util::logging::err_log,
  MidiOutputConnection,
};

use midir::{
  ConnectError, 
  MidiInput,
  MidiInputConnection,
  MidiInputPort,
  MidiOutput,
  MidiOutputPort
};


/// Convenience struct for creating a Midi Output connection
/// ```
/// let port = midi::connection::Output::new("IAC Driver Bus 1").unwrap();
/// ```
pub struct Output{ 
  conn: MidiOutputConnection,
}

impl Output {
  pub fn new(device: &'static str) -> Self {
    match Self::init(device) {
      Ok(c) => Self{
        conn: c,
      },
      Err(e) => err_log(e)
    }
  }

  // pub fn get_conn(&mut self) -> Arc<Mutex<MidiOutputConnection>> { self.conn }
  pub fn send(&mut self, message: &[u8]) -> Result<(), midir::SendError> {
    self.conn.send(message)
  }

  fn init_client() -> Result<MidiOutput, String> {
    match MidiOutput::new("cpu") {
      Ok(output) => Ok(output),
      Err(e) => Err(format!("could not create MIDI output: {}", e))
    }
  }

  fn validate_port(output: &MidiOutput, device: &'static str, ports: Vec<MidiOutputPort>) -> Result<MidiOutputPort, String> {
    match ports
      .iter()
      .find(|p| 
        Some(device) == output.port_name(p).ok().as_deref()) {
      Some(p) => Ok(p.clone()),
      None => Err("could not find output port".to_owned())
    }
  }

  fn connect(output: MidiOutput, port: &MidiOutputPort, device: &'static str) -> Result<MidiOutputConnection, String> {
    match output.connect(port, device) {
      Ok(conn) => Ok(conn),
      Err(e) => Err(format!("could not connect to output port: {}", e))
    }
  } 

  fn init(device: &'static str) -> Result<MidiOutputConnection, String> {
    // Setup new MIDI output client, (should not fail).
    let output = Self::init_client()?;
    // See if there is a device that corresponds to requested port.
    let port = Self::validate_port(&output, device, output.ports())?;
    // create output connection
    Self::connect(output, &port, device)
  }
}

/// Convenience struct for creating a Midi Input connection
/// ```
/// use std::collection::VecDeque;
/// use std::sync::{Arc, Mutex};
/// let port = midi::connection::Input::new(
///         "IAC Driver Bus 1",
///         Arc<Mutex<VecDeque<Vec<u8>>>>,
///         |timecode, message, data| {
///           print!("{timecode}:: ");
///           message.iter().for_each(|byte| {print!("{byte:b}")});
///           println!();
///         }
///     ).unwrap();
/// ```
pub struct Input<T, F>
  where 
    T: Send + 'static,
    F: FnMut(u64, &[u8], &mut T) + Send + 'static,
{
  pub conn: MidiInputConnection<T>,
  _marker: PhantomData<F>
}

impl<T, F> Input<T, F>
  where 
    T: Send + 'static,
    F: FnMut(u64, &[u8], &mut T) + Send + 'static,
{
  pub fn new(device: &'static str, data: T, callback: F) -> Self
  {
    match Self::init(device, data, callback) {
      Ok(c) => Self{
        conn: c,
        _marker: PhantomData
      },
      Err(e) => err_log(e)
    }
  }

  #[inline]
  fn connect(input: MidiInput, port: &MidiInputPort, device: &'static str, data: T, callback: F) -> Result<MidiInputConnection<T>, String> {
    match input.connect(port, device, callback, data) {
      Ok(conn) => Ok(conn),
      Err(e) => Err(format!("could not connect to output port: {}", e))
    }
  } 


  #[inline]
  fn init(device: &'static str, data: T, callback: F) -> Result<MidiInputConnection<T>, String> {
    let input = Self::init_client()?;
    let port = Self::validate_port(&input, device, input.ports())?;
    Self::connect(input, &port, device, data, callback)
  }
  
  #[inline]
  fn init_client() -> Result<MidiInput, String> {
    match MidiInput::new("cpu") {
      Ok(input) => Ok(input),
      Err(e) => Err(format!("could not create MIDI input: {}", e))
    }
  }

  #[inline]
  fn validate_port(input: &MidiInput, device: &'static str, ports: Vec<MidiInputPort>) -> Result<MidiInputPort, String> {
    match ports
      .iter()
      .find(|p| 
        Some(device) == input.port_name(p).ok().as_deref()) {
      Some(p) => Ok(p.clone()),
      None => Err(format!("could not find input port: {device}").to_owned())
    }
  }
}

pub fn fetch_output_port(name: &str) -> Result<MidiOutputConnection, ConnectError<MidiOutput>> {
  let output = MidiOutput::new("cpu").unwrap_or_else(|_| panic!("could not create MIDI output"));
  let ports = output.ports();
  
  let port = ports
    .iter()
    .find(|p| name == output
      .port_name(p)
      .unwrap())
    .unwrap_or_else(|| panic!("{name} is not available as midi output"));
  
  output.connect(port, name)

}

