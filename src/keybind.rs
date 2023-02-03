use x11rb::errors::ReplyError;
use x11rb::protocol::xproto::{ConnectionExt, Window};
use x11rb::rust_connection::RustConnection;

use crate::wm::WM;

pub fn modify_focused<E, F>(f: F) -> impl Fn(&WM) -> Result<(), E> where
    F: Fn(&RustConnection, &Option<Window>) -> Result<(), E> + 'static,
{
    move |wm: &WM| { f(&wm.conn, &wm.focused) }
}

fn test() {
    let z = modify_focused(|conn, win| { conn.destroy_window(win.unwrap())?; Ok::<(), ReplyError>(()) });
}
