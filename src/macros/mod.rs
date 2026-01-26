use crate::message::{Message, cc::Cc};
use crate::message::nrpn::Nrpn;


#[macro_export]
macro_rules! midi {
  // CHORDS=----------------------------------------------------------------------
  (chord on: [$($n:literal),*], $v:literal, $p:ident, $c:ident; $($rest:tt)*) => {
    $(
      $crate::midi! {note on: $n, $v, $p, $c;}
    )*
    $crate::midi! {$($rest)*}
  };
  
  (chord on: [$($n:expr),*], [$($v:expr),*], $p:ident, $c:ident; $($rest:tt)*) => {
    $(
      $crate::midi! {note on: $n, $v, $p, $c;}
    )*
    $crate::midi! {$($rest)*}
  };

  (chord off: [$($n:literal),*], $p:ident, $c:ident; $($rest:tt)*) => {
    $(
      $crate::midi! {note off: $n, $p, $c;}
    )*
    $crate::midi! {$($rest)*}
  };


  (chord on: $n:ident, $v:ident, $p:ident, $c:ident; $($rest:tt)*) => {
    for (note, vel) in $n.iter().zip($v.iter()) {
      $crate::midi! {note on: *note, *vel, $p, $c;}
    }
    $crate::midi! {$($rest)*}
  };
  
  (chord on: [$($n:expr),*], [$($v:expr),*], $p:ident, $c:ident; $($rest:tt)*) => {
    $(
      $crate::midi! {note on: $n, $v, $p, $c;}
    )*
    $crate::midi! {$($rest)*}
  };

  (chord off: [$($n:literal),*], $p:ident, $c:ident; $($rest:tt)*) => {
    $(
      $crate::midi! {note off: $n, $p, $c;}
    )*
    $crate::midi! {$($rest)*}
  };

  (chord off: $n:ident, $p:ident, $c:ident; $($rest:tt)*) => {
    for note in $n.iter() {
      $crate::midi! {note off: *note, $p, $c;}
    }
    $crate::midi! {$($rest)*}
  };

  // NOTE=----------------------------------------------------------

  (note on: $n:expr, $v:expr, $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::message::Message::new(
      $crate::message::note::NoteOn{note: $n, velo: $v}
    ).expect("could not create midi note on message").send(&$p, $c);
    $crate::midi! {$($rest)*}
  };

  (note off: $v:expr, $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::message::Message::new(
      $crate::message::note::NoteOff{note: $v}
    ).expect("could not create midi note off message").send(&$p, $c);
    $crate::midi! {$($rest)*}
  };

  // CC=------------------------------------------------------------------

  (cc: $addr:expr, $val:expr, $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::message::Message::new(
      $crate::message::cc::Cc{addr: $addr, val: $val}
    ).expect("could not create cc message").send(&$p, $c);
    $crate::midi! {$($rest)*}
  };

  //NRPN =--------------------------------------------------------------------------

  (nrpn: ($ax:expr, $ay:expr), ($vx:expr, $vy:expr), $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::message::Message::new(
      $crate::message::nrpn::Nrpn{addr: ($ax, $ay), val: ($vx, $vy)}
    ).expect("could not create nrpn message").send(&$p, $c);
    $crate::midi! {$($rest)*}
  };
  
  (nrpn: $addr:ident, $val:ident, $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::midi! {nrpn: ($addr.0, $addr.1), ($val.0, $val.1), $p, $c; $($rest)*}
  };
  
  (nrpn: $addr:ident, ($val0:literal, $val1:literal), $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::midi! {nrpn: ($addr.0, $addr.1), ($val0, $val1), $p, $c; $($rest)*}
  };

  (nrpn: ($addr0:literal, $addr1:literal), $val:ident, $p:ident, $c:ident; $($rest:tt)*) => {
$crate::midi! {nrpn: ($addr0, $addr1), ($val.0, $val.1), $p, $c; $($rest)*}
  };

  //RPN =--------------------------------------------------------------------------
  
  (rpn: $addr:ident, ($vx:expr, $vy:expr), $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::message::Message::new(
      $crate::message::rpn::Rpn{addr: $addr, val: ($vx, $vy)}
    ).expect("could not create rpn message").send(&$p, $c);
    $crate::midi! {$($rest)*}
  };
  
  (rpn: $addr:ident, $val:ident, $p:ident, $c:ident; $($rest:tt)*) => {
    $crate::midi! {rpn: $addr, ($val.0, $val.1), $p, $c; $($rest)*}
  };

  (rpn: $addr:expr, ($val0:expr, $val1:expr), $p:ident, $c:ident; $($rest:tt)*) => {
    let t = $addr;
    $crate::midi! {rpn: t, ($val0, $val1), $p, $c; $($rest)*}
  };


  // SYSEX=---------------------------------------------------------

  // (sysex: $data:expr, $p:ident, $c:ident; $($rest:tt)*) => {
  //   let cow = std::borrow::Cow::Borrowed($data);
  //   $crate::message::Message::new(
  //     $crate::message::sysex::SysEx{data: cow}
  //   ).expect("could not create sysex message").send(&$p, $c);
  //   $crate::midi! {$($rest)*}
  // };

  // (sysex: [$($data:expr),*], $p:ident, $c:ident; $($rest:tt)*) => {
  //   let cow = std::borrow::Cow::borrowed(&[$($data),*]);
  //   $crate::midi! {sysex: cow, p, c; $($rest)*}
  // };

  // WAIT=---------------------------------------------------------
  
  (wait: $d:expr; $($rest:tt)*) => {
    $crate::transport::sleep(std::time::Duration::from_millis($d));
    $crate::midi! {$($rest)*}
  };

  () => {};
}

#[macro_export]
macro_rules! seq {
  ($($midi_loop:tt)*) => {
    loop {
      $crate::midi!($($midi_loop)*);
    }
  };
  () => {};
}

#[macro_export]
macro_rules! testing {
  ([$($data:expr),*]) => {
    println!("{}", [$($data),*]);
  };
    () => {
        
    };
}


// fn c() {
//   Message::new(Nrpn{addr: 0, val}).unwrap();
// }
