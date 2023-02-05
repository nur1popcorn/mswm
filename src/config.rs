use x11rb::protocol::xproto::{ModMask, Button};

pub const MOD_MASK: ModMask = ModMask::M2;
pub const MOVE_BUTTON: Button = 0x1;
pub const RESIZE_BUTTON: Button = 0x3;

pub const TEXT_COLOR: u32 = 0xfffafafa;
pub const TOP_BAR_COLOR: u32 = 0xff224488;

pub const TOP_BAR_HEIGHT: u16 = 20;
pub const TOP_BAR_TEXT_OFFSET: i16 = 4;

pub const MIN_WIN_WIDTH: i32 = 10;
