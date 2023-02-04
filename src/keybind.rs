use std::collections::HashMap;
use x11rb::connection::Connection;
use x11rb::errors::ReplyError;
use x11rb::protocol::xproto::{ConnectionExt, ModMask};
use x11rb::rust_connection::RustConnection;
use xkbcommon::xkb;
use crate::config::MOD_MASK;

#[derive(Debug)]
pub struct KeyBind {
    mask: u16,
    key: u16
}

impl KeyBind {

    pub fn get_keymap(conn: &RustConnection) -> Result<HashMap<String, u16>, ReplyError> {
        let setup = conn.setup();
        let keyboard_mapping = conn.get_keyboard_mapping(
            setup.min_keycode, setup.max_keycode - setup.min_keycode + 1)?.reply()?;

        let mut keymap = HashMap::new();
        let nkeycodes = keyboard_mapping.keysyms.len() / (keyboard_mapping.keysyms_per_keycode as usize);
        for i in 0 .. nkeycodes {
            //print!("{:?} = ", setup.min_keycode as usize + i);
            for j in 0 .. keyboard_mapping.keysyms_per_keycode {
                let keysym = keyboard_mapping.keysyms[j as usize + i * keyboard_mapping.keysyms_per_keycode as usize];
                if keysym > 0 {
                    keymap.insert(xkb::keysym_get_name(keysym), (setup.min_keycode as u16) + (i as u16));
                    //print!("{:?}, ", xkb::keysym_get_name(keysym));
                }
            }
            //println!();
        }
        println!("{:?}", KeyBind::new("CTRL+LOCK+C", &keymap));
        Ok(keymap)
    }

    pub fn new(s: &str, keymap: &HashMap<String, u16>) -> KeyBind {
        let mut binding = KeyBind { mask: 0, key: 0 };
        for key in s.split("+") {
            match &key[..] {
                "SHIFT" => { binding.mask |= ModMask::SHIFT; },
                "LOCK" => { binding.mask |= ModMask::LOCK; },
                "CTRL" => { binding.mask |= ModMask::CONTROL; },
                "M1" => { binding.mask |= ModMask::M1; },
                "M2" => { binding.mask |= ModMask::M2; },
                "M3" => { binding.mask |= ModMask::M3; },
                "M4" => { binding.mask |= ModMask::M4; },
                "M5" => { binding.mask |= ModMask::M5; },
                "ANY" => { binding.mask |= ModMask::ANY; },
                _ => { binding.key = keymap[key]; }
            }
            //println!("{:?}", binding);
        }
        binding
    }
}