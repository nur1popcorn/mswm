mod wm;
mod config;
mod layout;

use crate::wm::WM;

use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::ErrorKind;

fn main() {
    let (conn, screen_num) = x11rb::connect(None)
        .expect("Failed to connect to the X11 server");
    let wm = WM::create_wm(conn, screen_num);
    if let Err(ReplyOrIdError::X11Error(error)) = wm {
        if error.error_kind == ErrorKind::Access {
            panic!("There is already a window manager present");
        } else {
            panic!("An error occurred while trying to become wm");
        }
    }

    let mut wm = wm.unwrap();
    wm.scan().unwrap();
    loop { wm.handle_events().unwrap(); }
}
