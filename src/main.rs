use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::rust_connection::RustConnection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    conn.change_window_attributes(screen.root,
        &ChangeWindowAttributesAux::default().event_mask(
            EventMask::STRUCTURE_NOTIFY |
            EventMask::SUBSTRUCTURE_NOTIFY |
            EventMask::SUBSTRUCTURE_REDIRECT |
            EventMask::PROPERTY_CHANGE))?;

    conn.flush()?;
    Ok(())
}
