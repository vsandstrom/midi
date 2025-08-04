




Output MIDI messages:

```rust
use midi::{
    connection::Output,
    messages::cc,
    transport::sleep
};
use std::time::Duration;

let channel = 0;
let addr = 1;
let dur = Duration::new(1, 0);
// Use callback with `loop` to quickly generate a MIDI sequence. 
let _ = Output::new("IAC Driver Bus 1", |port| {
    for i in 0..128 {
        cc(&port, channel, addr, i);
        sleep(dur);
    }
})
```


Parsing MIDI Input:

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use midi::{
    connection::Input, 
    consts::{
        message::{CC, NRPN_MSB, SYSEX_BEGIN}
        note::{NOTE_OFF, NOTE_ON}
    }
};

fn main() {
    let data = Arc::new(Mutex::new(VecDeque::<Vec<u8>>::new()));
    let port = Input::new("IAC Drivrutin Buss 1", data.clone(), move |timecode, msg, _data| {
        print!("{timecode}");
        // match on midi input type
        match msg[0] {
            CC => {println!("cc message: {:?}", msg)},
            NOTE_ON => {println!("note on: {:?}", msg)},
            NOTE_OFF => {println!("note off: {:?}", msg)},
            NRPN_MSB => {println!("nrpn message: {:?}", msg)},
            SYSEX_BEGIN => {println!("sysex: {:?}", msg)},
            _ => panic!("unknown midi message")
        }
        // add message to data queue
        data.try_lock().unwrap().push_back(msg.into());
    });
}
```

