///////////////////
// Configuration //
///////////////////

// Valid choices are SUPER, ALT, CTRL, and SHIFT
const MOD: xproto::ModMask = SUPER;

// Borders
const BORDERWIDTH: u8 = 4;
const FOCUSCOL: u32 = 0x18191A;
const UNFOCUSCOL: u32 = 0x111213;

// Resize and move by mouse?
const ENABLE_MOUSE: bool = true;

// Sloppy focus?
const ENABLE_SLOPPY: bool = true;

/////////////////////
// Important stuff //
/////////////////////

extern crate xcb;

use std::process;
use xcb::{base,xproto};

const SUPER: xproto::ModMask = xproto::MOD_MASK_4;
const ALT: xproto::ModMask = xproto::MOD_MASK_1;
const CTRL: xproto::ModMask = xproto::MOD_MASK_CONTROL;
const SHIFT: xproto::ModMask = xproto::MOD_MASK_SHIFT;

fn get_connection() -> base::Connection {
    match base::Connection::connect(None) {
        Ok((conn, _)) => conn,
        Err(_) => {
            println!("Unable to connect to the X server");
            process::exit(1);
        }
    }
}

fn main() {
    let connection = get_connection();
    println!("Lorem ipsum dolor sit amet");
}
