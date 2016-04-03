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

////////////////////
// Initialization //
////////////////////

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

fn get_screen<'a>(setup: &'a xproto::Setup<'a>) -> xproto::Screen<'a> {
    match setup.roots().next() {
        Some(scr) => scr,
        None => {
            println!("Lost connection to X server");
            process::exit(1);
        },
    }
}

//////////
// Main //
//////////

fn deploy<'a>(setup: &'a &xproto::Setup<'a>) -> xproto::Screen<'a> {
    let screen = get_screen(&setup);
    screen
}

// focus(xcb_window_t win, int mode)
fn focus() {
}

fn subscribe(win: xproto::Window) {
}

fn events_loop() {
}

fn main() {
    let connection = get_connection();
    let setup = connection.get_setup();


    loop {
        events_loop();
    }

    process::exit(1);
}
