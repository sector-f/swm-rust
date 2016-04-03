///////////////////
// Configuration //
///////////////////

// Valid choices are SUPER, ALT, CTRL, and SHzIFT
const MOD: xproto::ModMask = SUPER;

// Borders
const BORDERWIDTH: u32 = 4;
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

const SUPER: xcb::ModMask = xcb::MOD_MASK_4;
const ALT: xcb::ModMask = xcb::MOD_MASK_1;
const CTRL: xcb::ModMask = xcb::MOD_MASK_CONTROL;
const SHIFT: xcb::ModMask = xcb::MOD_MASK_SHIFT;

enum Mode {
    Inactive,
    Active,
}

fn get_connection() -> xcb::Connection {
    match xcb::Connection::connect(None) {
        Ok((conn, _)) => conn,
        Err(_) => {
            println!("Unable to connect to the X server");
            process::exit(1);
        }
    }
}

fn get_screen<'a>(setup: &'a xcb::Setup<'a>) -> xcb::Screen<'a> {
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

// focus(xcb_window_t win, int mode)
fn focus(win: xcb::Window, mode: Mode) {
}

fn subscribe(connection: &xcb::Connection, win: xcb::Window) {
    xcb::change_window_attributes(connection, win, &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_ENTER_WINDOW), (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)]);

    xcb::configure_window(connection, win, &[(xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, BORDERWIDTH)]);
}

fn events_loop() {
}

fn main() {
    let connection = get_connection();
    let setup = connection.get_setup();
    let screen = get_screen(&setup);
    let focuswin = screen.root();

    if ENABLE_MOUSE {
        // xcb_grab_button(conn, 0, scr->root, XCB_EVENT_MASK_BUTTON_PRESS |
        //         XCB_EVENT_MASK_BUTTON_RELEASE, XCB_GRAB_MODE_ASYNC,
        //         XCB_GRAB_MODE_ASYNC, scr->root, XCB_NONE, 1, MOD);

        xcb::grab_button(&connection, false, focuswin, (xcb::EVENT_MASK_BUTTON_PRESS + xcb::EVENT_MASK_BUTTON_RELEASE) as u16, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8, focuswin, xcb::NONE, 1, MOD as u16);
        xcb::grab_button(&connection, false, focuswin, (xcb::EVENT_MASK_BUTTON_PRESS + xcb::EVENT_MASK_BUTTON_RELEASE) as u16, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8, focuswin, xcb::NONE, 3, MOD as u16);
    }

    xcb::change_window_attributes_checked(&connection, screen.root(), &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)]);

    connection.flush();

    loop {
        events_loop();
    }

    process::exit(1);
}
