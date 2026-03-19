




Output MIDI messages:

```rust
use midi::{
    connection::Output,
    messages::{Message, cc::Cc},
    transport::sleep,
    util::Channel,
};
use std::time::Duration;

let channel: Channel = Channel(0);
let addr = 1;
let dur = Duration::from_millis(100);
// check that cc address and value is within range (0-127)
let msg: Message<Cc> = Message::cc(addr, 0).unwrap(); 

let _ = Output::new("IAC Driver Bus 1", |port| {
    loop {
        for i in 0..128 {
            msg.send(port, channel);
            sleep(dur);
            // check that value is within range (0-127)
            msg.update_value(i).unwrap();
        }
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
    let port = Input::new("IAC Driver Bus 1", data.clone(), move |timecode, msg, queue| {
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
        queue.try_lock().unwrap().push_back(msg.into());
    });
}
```

