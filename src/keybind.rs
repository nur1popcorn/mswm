use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;
use x11rb::connection::Connection;
use x11rb::errors::{ReplyError, ReplyOrIdError};
use x11rb::protocol::xproto::{ConnectionExt, GrabMode, KeyButMask, Keycode, ModMask, Window};
use x11rb::rust_connection::RustConnection;
use xkbcommon::xkb;
use crate::wm::WM;

static KEY_MAP: Mutex<Option<HashMap<String, u16>>> = Mutex::new(None);

pub fn init_keymap(conn: &RustConnection) -> Result<(), Box<dyn Error>> {
    let setup = conn.setup();
    let keyboard_mapping = conn.get_keyboard_mapping(
        setup.min_keycode, setup.max_keycode - setup.min_keycode + 1)?.reply()?;

    let mut keymap = HashMap::new();
    let keysym_count = keyboard_mapping.keysyms_per_keycode as usize;
    for i in 0 .. keyboard_mapping.keysyms.len() / keysym_count {
        for j in 0 .. keyboard_mapping.keysyms_per_keycode {
            let keysym = keyboard_mapping.keysyms[j as usize + i * keysym_count];
            if keysym > 0 {
                keymap.insert(xkb::keysym_get_name(keysym), (setup.min_keycode as u16) + (i as u16));
            }
        }
    }
    *KEY_MAP.lock()? = Some(keymap);
    Ok(())
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct KeyBind {
    mask: u16,
    key: u16
}

impl From<&str> for KeyBind {
    fn from(value: &str) -> Self {
        let mut keybind = KeyBind::default();
        let opt_keymap = KEY_MAP.lock().unwrap();
        let keymap = opt_keymap.as_ref().unwrap();
        for k in value.split("+") {
            match &k[..] {
                "SHIFT"   => { keybind.mask |= ModMask::SHIFT;   },
                "LOCK"    => { keybind.mask |= ModMask::LOCK;    },
                "CONTROL" => { keybind.mask |= ModMask::CONTROL; },
                "M1"      => { keybind.mask |= ModMask::M1;      },
                "M2"      => { keybind.mask |= ModMask::M2;      },
                "M3"      => { keybind.mask |= ModMask::M3;      },
                "M4"      => { keybind.mask |= ModMask::M4;      },
                "M5"      => { keybind.mask |= ModMask::M5;      },
                "ANY"     => { keybind.mask |= ModMask::ANY;     },
                _         => { keybind.key = keymap[k]; }
            }
        }
        keybind
    }
}

pub trait KeyHandler {
    fn grab_keys(&self, conn: &RustConnection, win: Window) -> Result<(), ReplyError>;
    fn handle_key_bind(&self, wm: &mut WM, mask: KeyButMask, key: Keycode) -> Result<(), ReplyOrIdError>;
}

pub struct KeyBindHandler<F> where
    F: Fn(&mut WM) -> Result<(), ReplyOrIdError> + 'static
{
    bind_map: HashMap<KeyBind, Box<F>>
}

impl <F> KeyBindHandler<F> where
    F: Fn(&mut WM) -> Result<(), ReplyOrIdError> + 'static
{
    pub fn new(map: HashMap<&str, F>) -> Self {
        let mut bind_map = HashMap::with_capacity(map.len());
        for (k, v) in map {
            bind_map.insert(KeyBind::from(k), Box::new(v));
        }
        Self { bind_map }
    }
}

impl <F> KeyHandler for KeyBindHandler<F> where
    F: Fn(&mut WM) -> Result<(), ReplyOrIdError> + 'static
{
    fn grab_keys(&self, conn: &RustConnection, win: Window) -> Result<(), ReplyError> {
        for (k, _) in &self.bind_map {
            conn.grab_key(
                false, win,
                ModMask::from(k.mask),
                k.key as u8,
                GrabMode::ASYNC,
                GrabMode::ASYNC
            )?;
        }
        Ok(())
    }

    fn handle_key_bind(&self, wm: &mut WM, mask: KeyButMask, key: Keycode) -> Result<(), ReplyOrIdError> {
        let pressed = KeyBind { mask: mask.into(), key: key as u16 };
        self.bind_map.get(&pressed).map(|f| f(wm));
        Ok(())
    }
}
