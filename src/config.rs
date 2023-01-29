use x11rb::protocol::xproto::{ModMask, Button};

pub const MOD_MASK: ModMask = ModMask::M4;
pub const MOVE_BUTTON: Button = 0x1;
pub const RESIZE_BUTTON: Button = 0x3;
