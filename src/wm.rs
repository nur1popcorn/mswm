use x11rb::errors::ReplyError;
use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;

pub struct WM {
    conn: RustConnection,
    screen_num: usize
}

impl WM {
    pub fn create_wm(conn: RustConnection, screen_num: usize) -> Result<Self, ReplyError> {
        let screen = &conn.setup().roots[screen_num];
        let change = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT |
                        EventMask::SUBSTRUCTURE_NOTIFY);
        // only one X client can select substructure redirection.
        conn.change_window_attributes(screen.root, &change)?.check()?;
        Ok(Self { conn, screen_num })
    }
}
