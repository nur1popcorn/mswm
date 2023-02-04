use x11rb::protocol::xproto::{Rectangle, Window};
use crate::config::TOP_BAR_HEIGHT;

pub trait Layout2 {
    fn layout(self, screen: Rectangle, windows: Vec<Window>) -> Vec<(Window, Rectangle)>;
}

pub struct FibonacciLayout { }

impl Layout2 for FibonacciLayout {
    fn layout(self, screen: Rectangle, windows: Vec<Window>) -> Vec<(Window, Rectangle)> {
        let mut result = Vec::with_capacity(windows.len());
        let (mut x, mut y, mut width, mut height) = (0i16, TOP_BAR_HEIGHT as i16, screen.width, screen.height - TOP_BAR_HEIGHT);
        for i in 1 .. windows.len() - 1 {
            if i % 2 == 0 {
                width /= 2;
            } else {
                height /= 2;
            }
            result.push((windows[windows.len() - i], Rectangle { x, y, width, height }));

            if i % 2 == 0 {
                x += width as i16;
            } else {
                y += height as i16;
            }
        }
        result.push((windows[0], Rectangle { x, y, width, height }));
        result
    }
}
