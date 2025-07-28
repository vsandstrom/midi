use std::marker::PhantomData;

use crate::{
  util::logging::err_log,
  MidiOutputConnection,
};

use midir::{
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
}


trait MidiConnection {
  type MidiType;
  type MidiPort;
  fn validate_port(input: &Self::MidiType, device: &'static str, ports: Vec<Self::MidiPort>) -> Result<Self::MidiPort, String>;
  fn init_client() -> Result<Self::MidiType, String>;
}


impl MidiConnection for Output {
  type MidiType = MidiOutput;
  type MidiPort = MidiOutputPort;
  fn validate_port(output: &Self::MidiType, device: &'static str, ports: Vec<Self::MidiPort>) -> Result<Self::MidiPort, String> {
    match ports
      .iter()
      .find(|p| 
        Some(device) == output.port_name(p).ok().as_deref()) {
      Some(p) => Ok(p.clone()),
      None => Err("could not find output port".to_owned())
    }
  }
  
  fn init_client() -> Result<Self::MidiType, String> {
    match Self::MidiType::new("cpu") {
      Ok(output) => Ok(output),
      Err(e) => Err(format!("could not create MIDI output: {}", e))
    }
  }
}

impl<T, F> MidiConnection for Input<T, F> 
  where 
    T: Send + 'static,
    F: FnMut(u64, &[u8], &mut T) + Send + 'static,
{
  type MidiType = MidiInput;
  type MidiPort = MidiInputPort;
  fn validate_port(input: &Self::MidiType, device: &'static str, ports: Vec<Self::MidiPort>) -> Result<Self::MidiPort, String> {
    match ports
      .iter()
      .find(|p| 
        Some(device) == input.port_name(p).ok().as_deref()) {
      Some(p) => Ok(p.clone()),
      None => Err("could not find input port".to_owned())
    }
      
  }
  
  fn init_client() -> Result<Self::MidiType, String> {
    match Self::MidiType::new("cpu") {
      Ok(input) => Ok(input),
      Err(e) => Err(format!("could not create MIDI input: {}", e))
    }
  }
}
