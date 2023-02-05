mod wm;
mod config;
mod layout;
mod keybind;

use std::collections::HashMap;
use crate::wm::WM;

use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::ErrorKind;
use crate::config::spawn_program;
use crate::keybind::{init_keymap, KeyBindHandler, make_action};
use crate::layout::FibonacciLayout;

fn main() {
    let (conn, screen_num) = x11rb::connect(None)
        .expect("Failed to connect to the X11 server");
    init_keymap(&conn).unwrap();

    let key_handler = KeyBindHandler::new(HashMap::from([
        ("M4+x",            make_action(|wm| wm.apply_layout(FibonacciLayout{}))),
        ("M4+f",            make_action(|wm| wm.stack_inc())),
        ("M4+g",            make_action(|wm| wm.stack_dec())),
        ("M4+SHIFT+c",      make_action(|wm| wm.kill_focused())),
        ("M4+SHIFT+Return", make_action(|_|  spawn_program("xterm")))
    ]));

    let wm = WM::create_wm(conn, screen_num, &key_handler);
    if let Err(ReplyOrIdError::X11Error(error)) = wm {
        if error.error_kind == ErrorKind::Access {
            panic!("There is already a window manager present");
        } else {
            panic!("An error occurred while trying to become wm");
        }
    }

    let mut wm = wm.unwrap();
    wm.scan(&key_handler).unwrap();
    loop { wm.handle_events(&key_handler).unwrap(); }
}
