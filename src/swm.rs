///////////////////
// Configuration //
///////////////////

// Valid choices are SUPER, ALT, CTRL, and SHzIFT
const MOD: xproto::ModMask = SUPER;

// Borders
const BORDERWIDTH: u32 = 4;
// const FOCUSCOL: u32 = 0x18191A;
const FOCUSCOL: u32 = 0xFF0000;
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
            focus(connection, focuswin, focuswin, Mode::Inactive);
            focuswin = win;
        }
    }

    focuswin
}

fn subscribe(connection: &xcb::Connection, win: xcb::Window) {
    xcb::change_window_attributes(connection, win, &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_ENTER_WINDOW), (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)]);

    xcb::configure_window(connection, win, &[(xcb::CONFIG_WINDOW_BORDER_WIDTH as u16, BORDERWIDTH)]);
}

fn events_loop(connection: &xcb::Connection, screen: &xcb::Screen, mut focuswin: xcb::Window) {
    let mut event: xcb::GenericEvent;
    let mut win: xcb::Window = 0;
    let mut values: [u32; 3] = [0; 3];

    loop {
        event = match connection.wait_for_event() {
            Some(ev) => ev,
            None => {
                println!("XCB connection broken");
                process::exit(1);
            },
        };

        match event.response_type() {
            xcb::CREATE_NOTIFY => {
                let event: &xcb::CreateNotifyEvent = xcb::cast_event(&event);
                if ! event.override_redirect() {
                    subscribe(&connection, event.window());
                    focuswin = focus(&connection, event.window(), focuswin, Mode::Active);
                }
            },
            xcb::DESTROY_NOTIFY => {
                let event: &xcb::DestroyNotifyEvent = xcb::cast_event(&event);
                xcb::kill_client(connection, event.window());
            },
            xcb::ENTER_NOTIFY => {
                if ENABLE_SLOPPY {
                    let event: &xcb::EnterNotifyEvent = xcb::cast_event(&event);
                    focuswin = focus(&connection, event.event(), focuswin, Mode::Active);
                }
            },
            xcb::MAP_NOTIFY => {
                let event: &xcb::MapNotifyEvent = xcb::cast_event(&event);
                if ! event.override_redirect() {
                    xcb::map_window(connection, event.window());
                    focuswin = focus(&connection, event.window(), focuswin, Mode::Active);
                }
            },
            xcb::BUTTON_PRESS => {
                if ENABLE_MOUSE {
                    let event: &xcb::ButtonPressEvent = xcb::cast_event(&event);
                    win = event.child();

                    if win == 0 || win == screen.root() {
                        connection.flush();
                        return;
                    }

                    values[0] = xcb::STACK_MODE_ABOVE;

                    xcb::configure_window(connection, win, &[(xcb::CONFIG_WINDOW_STACK_MODE as u16, values[0])]);

                    let geom = xcb::get_geometry(connection, win).get_reply().expect("An error with the X server occurred");

                    if event.detail() == 1 {
                        values[2] = 1;
                        xcb::warp_pointer(connection, xcb::NONE, win, 0, 0, 0, 0, (geom.width() / 2) as i16, (geom.height() / 2) as i16);
                    } else {
                        values[2] = 3;
                        xcb::warp_pointer(connection, xcb::NONE, win, 0, 0, 0, 0, geom.width() as i16, geom.height() as i16);
                    }
                    xcb::grab_pointer(connection, false, screen.root(), (xcb::EVENT_MASK_BUTTON_RELEASE | xcb::EVENT_MASK_BUTTON_MOTION | xcb::EVENT_MASK_POINTER_MOTION_HINT) as u16, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8, screen.root(), xcb::NONE, xcb::CURRENT_TIME);
                    connection.flush();
                }
            },
            xcb::MOTION_NOTIFY => {
                if ENABLE_MOUSE {
                    let pointer = xcb::query_pointer(connection, win).get_reply().expect("An error with the X server occurred");

                    if values[2] == 1 {
                        let geom = match xcb::get_geometry(connection, win).get_reply() {
                            Ok(g) => g,
                            Err(_) => return,
                        };

                        values[0] =
                            if (pointer.root_x() as u32 + geom.width() as u32 / 2) > screen.width_in_pixels() as u32 - (BORDERWIDTH * 2) {
                                (screen.width_in_pixels() as u32 - geom.width() as u32 - (BORDERWIDTH * 2)) as u32
                            } else {
                                (pointer.root_x() as u32 - geom.width() as u32 / 2) as u32
                            };
                        values[1] =
                            if (pointer.root_y() as u32 + geom.height() as u32 / 2) > screen.height_in_pixels() as u32 - (BORDERWIDTH * 2) {
                                (screen.height_in_pixels() as u32 - geom.height() as u32 - (BORDERWIDTH * 2)) as u32
                            } else {
                                (pointer.root_y() as u32 - geom.height() as u32 / 2) as u32
                            };

                        if pointer.root_x() as u32 > geom.width() as u32 / 2 {
                            values[0] = 0;
                        }
                        if pointer.root_y() as u32 > geom.height() as u32 / 2 {
                            values[1] = 0;
                        }

                        xcb::configure_window(connection, win, &[(xcb::CONFIG_WINDOW_X as u16, values[0]), (xcb::CONFIG_WINDOW_Y as u16, values[1])]);
                        connection.flush();
                    } else if values[2] == 3 {
                        let geom = match xcb::get_geometry(connection, win).get_reply() {
                            Ok(g) => g,
                            Err(_) => return,
                        };

                        values[0] = pointer.root_x() as u32 - geom.x() as u32;
                        values[1] = pointer.root_y() as u32 - geom.y() as u32;
                        xcb::configure_window(connection, win, &[(xcb::CONFIG_WINDOW_WIDTH as u16, values[0]), (xcb::CONFIG_WINDOW_HEIGHT as u16, values[1])]);
                    }
                }
            },
            xcb::BUTTON_RELEASE => {
                if ENABLE_MOUSE {
                    focuswin = focus(connection, win, focuswin, Mode::Active);
                    xcb::ungrab_pointer(connection, xcb::CURRENT_TIME);
                }
            },
            xcb::CONFIGURE_NOTIFY => {
                let event: &xcb::ConfigureNotifyEvent = xcb::cast_event(&event);
                if event.window() != focuswin {
                    focuswin = focus(&connection, event.window(), focuswin, Mode::Inactive);
                }
                    focuswin = focus(&connection, event.window(), focuswin, Mode::Active);
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
        xcb::grab_button(&connection, false, focuswin, (xcb::EVENT_MASK_BUTTON_PRESS | xcb::EVENT_MASK_BUTTON_RELEASE) as u16, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8, focuswin, xcb::NONE, 1, MOD as u16);
        xcb::grab_button(&connection, false, focuswin, (xcb::EVENT_MASK_BUTTON_PRESS | xcb::EVENT_MASK_BUTTON_RELEASE) as u16, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8, focuswin, xcb::NONE, 3, MOD as u16);
    }

    xcb::change_window_attributes_checked(&connection, screen.root(), &[(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY)]);

    connection.flush();

    loop {
        events_loop(&connection, &screen, focuswin);
    }

    process::exit(1);
}
