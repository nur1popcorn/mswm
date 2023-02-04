use std::collections::HashMap;
use x11rb::connection::Connection;
use x11rb::errors::ReplyError;
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::rust_connection::RustConnection;
use xkbcommon::xkb;

pub struct KeyBind {
    mask: u16,
    key: u16
}

pub fn get_keymap(conn: &RustConnection) -> Result<HashMap<String, u16>, ReplyError> {
    let setup = conn.setup();
    let keyboard_mapping = conn.get_keyboard_mapping(
        setup.min_keycode, setup.max_keycode - setup.min_keycode + 1)?.reply()?;

    let mut keymap = HashMap::new();
    let nkeycodes = keyboard_mapping.keysyms.len() / (keyboard_mapping.keysyms_per_keycode as usize);
    for i in 0 .. nkeycodes {
        print!("{:?} = ", setup.min_keycode as usize + i);
        for j in 0 .. keyboard_mapping.keysyms_per_keycode {
            let keysym = keyboard_mapping.keysyms[j as usize + i * keyboard_mapping.keysyms_per_keycode as usize];
            if keysym > 0 {
                print!("{:?}, ", xkb::keysym_get_name(keysym));
            }
        }
        println!();
    }
    Ok(keymap)
}
