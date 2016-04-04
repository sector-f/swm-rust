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

fn focus(connection: &xcb::Connection, win: xcb::Window, mut focuswin: xcb::Window, mode: Mode) -> xcb::Window {
    let border = if let Mode::Inactive = mode {
        UNFOCUSCOL
    } else {
        FOCUSCOL
    };
    xcb::change_window_attributes(connection, win, &[(xcb::CW_BORDER_PIXEL, border)]);

    if let Mode::Active = mode {
        xcb::set_input_focus(connection, xcb::INPUT_FOCUS_POINTER_ROOT as u8, win, xcb::CURRENT_TIME);
        if win != focuswin {
            focuswin = win
        }
    }

    focuswin
}

fn subscribe(connection: &xcb::Connection, win: xcb::Window) {
    xcb::change_window_attributes(connection, win, &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_ENTER_WINDOW), (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)]);

    xcb::configure_window(connection, win, &[(xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, BORDERWIDTH)]);
}

fn events_loop(connection: &xcb::Connection, mut focuswin: xcb::Window) {
    let mut event: xcb::GenericEvent;

    loop {
        println!("Waiting for event");
        event = match connection.wait_for_event() {
            Some(ev) => ev,
            None => {
                println!("XCB connection broken");
                process::exit(1);
            },
        };
        println!("Event received: {}", event.response_type());

        match event.response_type() {
            xcb::CREATE_NOTIFY => {
                println!("A window was created");
                let event: &xcb::CreateNotifyEvent = xcb::cast_event(&event);
                if ! event.override_redirect() {
                    subscribe(&connection, event.window());
                    focuswin = focus(&connection, event.window(), focuswin, Mode::Active);
                }
            },
            xcb::DESTROY_NOTIFY => {
                println!("A window was destroyed");
            },
            xcb::ENTER_NOTIFY => {
                if ENABLE_MOUSE {
                    println!("Mouse has entered a window");
                }
            },
            xcb::MAP_NOTIFY => {
            },
            xcb::BUTTON_PRESS => {
                if ENABLE_MOUSE {
                    println!("Mouse button pressed");
                }
            },
            xcb::MOTION_NOTIFY => {
                if ENABLE_SLOPPY {
                }
            },
            xcb::BUTTON_RELEASE => {
                if ENABLE_MOUSE {
                    println!("Mouse button released");
                }
            },
            xcb::CONFIGURE_NOTIFY => {
            },
            _ => {
            },
        }

        connection.flush();
    }
}

fn main() {
    let connection = get_connection();
    let setup = connection.get_setup();
    let screen = get_screen(&setup);
    let focuswin = screen.root();

    if ENABLE_MOUSE {
        xcb::grab_button(&connection, false, focuswin, (xcb::EVENT_MASK_BUTTON_PRESS + xcb::EVENT_MASK_BUTTON_RELEASE) as u16, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8, focuswin, xcb::NONE, 1, MOD as u16);
        xcb::grab_button(&connection, false, focuswin, (xcb::EVENT_MASK_BUTTON_PRESS + xcb::EVENT_MASK_BUTTON_RELEASE) as u16, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8, focuswin, xcb::NONE, 3, MOD as u16);
    }

    xcb::change_window_attributes_checked(&connection, screen.root(), &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)]);

    connection.flush();

    loop {
        events_loop(&connection, focuswin);
    }

    process::exit(1);
}
