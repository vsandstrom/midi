use midir::MidiOutputPort;

use super::{MidiOutputConnection, MidiOutput};
use super::{Arc, Mutex};

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

  pub fn new(device: &'static str) -> Result<Self, String> {
    // Setup new MIDI output client, (should not fail).
    let output = Self::init_client()?;
    // See if there is a device that corresponds to requested port.
    let port = Self::validate_port(&output, device, output.ports())?;
    // create output connection
    let conn = Self::connect(output, &port, device)?;
    Ok( Self{ conn: Arc::new( Mutex::new(conn)) })
  }

  pub fn get_conn(&mut self) -> Arc<Mutex<MidiOutputConnection>> { self.conn.clone() }
}
