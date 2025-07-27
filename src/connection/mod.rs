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
use std::{
  collections::VecDeque, marker::PhantomData, sync::{Arc, Mutex}
};


// #[derive(Clone)]
/// Convenience struct for creating a reference counted MIDI connection
/// ```
///
/// let port = MIDIConnection::new("Digitakt II").unwrap();
/// 
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



pub type MidiQueue = VecDeque<Vec<u8>>;

#[allow(unused)]
pub struct Input<F: FnMut(u64, &[u8], &mut Arc<Mutex<MidiQueue>>) + Send + 'static>
{
  pub conn: MidiInputConnection<Arc<Mutex<MidiQueue>>>,
  _marker: PhantomData<F>
}

impl<F: for <'a, 'b> FnMut(u64, &'a [u8], &'b mut Arc<Mutex<MidiQueue>>) +Send + 'static> Input<F>
{
  pub fn new(device: &'static str, queue: Arc<Mutex<MidiQueue>>, callback: F) -> Self
  {
    match Self::init(device, queue, callback) {
      Ok(c) => Self{
        conn: c,
        _marker: PhantomData
      },
      Err(e) => err_log(e)
    }
  }

  fn connect(input: MidiInput, port: &MidiInputPort, device: &'static str, queue: Arc<Mutex<MidiQueue>>, callback: F) -> Result<MidiInputConnection<Arc<Mutex<MidiQueue>>>, String> {

    match input.connect(port, device, callback, queue.clone()) {

      Ok(conn) => Ok(conn),
      Err(e) => Err(format!("could not connect to output port: {}", e))
    }
  } 


  fn init(device: &'static str, queue: Arc<Mutex<MidiQueue>>, callback: F) -> Result<MidiInputConnection<Arc<Mutex<MidiQueue>>>, String> {
    let input = Self::init_client()?;
    let port = Self::validate_port(&input, device, input.ports())?;
    Self::connect(input, &port, device, queue, callback)
  }
  
  fn init_client() -> Result<MidiInput, String> {
    match MidiInput::new("cpu") {
      Ok(input) => Ok(input),
      Err(e) => Err(format!("could not create MIDI input: {}", e))
    }
  }

  fn validate_port(input: &MidiInput, device: &'static str, ports: Vec<MidiInputPort>) -> Result<MidiInputPort, String> {
    ports.iter().for_each(|p| println!("{}", input.port_name(p).unwrap()));

    match ports
      .iter()
      .find(|p| 
        Some(device) == input.port_name(p).ok().as_deref()) {
      Some(p) => Ok(p.clone()),
      None => Err("could not find input port".to_owned())
    }
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

