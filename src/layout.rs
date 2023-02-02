use x11rb::protocol::xproto::{Rectangle, Window};

trait Layout {
    fn add_window(&mut self, win: Window) -> Vec<(Window, Rectangle)>;
    fn remove_window(&mut self, win: Window) -> Vec<(Window, Rectangle)>;
}
