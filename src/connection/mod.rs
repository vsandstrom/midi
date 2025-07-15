use crate::{
  Arc, Mutex, MidiOutputConnection,
  util::logging::err_log
};
use midir::{MidiOutputPort, ConnectError, MidiOutput};

#[derive(Clone)]
/// Convenience struct for creating a reference counted MIDI connection
/// ```
///
/// let port = MIDIConnection::new("Digitakt II").unwrap();
/// 
/// ```
pub struct MIDIConnection { conn: Arc<Mutex<MidiOutputConnection>> }
impl MIDIConnection {
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

  pub fn new(device: &'static str) -> Self {
    match Self::init(device) {
      Ok(c) => Self{conn: Arc::new(Mutex::new(c))},
      Err(e) => err_log(e)
    }
  }

  pub fn get_conn(&mut self) -> Arc<Mutex<MidiOutputConnection>> { self.conn.clone() }
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
