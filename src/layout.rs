use x11rb::protocol::xproto::{Rectangle, Window};

trait Layout {
    fn layout(&mut self, screen: Rectangle, windows: Vec<Window>) -> Vec<(Window, Rectangle)>;
}

struct FibonacciLayout { }

impl Layout for FibonacciLayout {
    fn layout(&mut self, screen: Rectangle, windows: Vec<Window>) -> Vec<(Window, Rectangle)> {
        let mut result = Vec::with_capacity(windows.len());
        let (mut x, mut y, mut width, mut height) = (0i16, 0i16, screen.width, screen.height);
        for i in 0 .. windows.len() - 1 {
            if i % 2 == 0 {
                width /= 2;
            } else {
                height /= 2;
            }
            result.push((windows[i], Rectangle { x, y, width, height }));

            x += width as i16;
            y += height as i16;
        }
        result.push((windows[windows.len() - 1], Rectangle { x, y, width, height }));
        result
    }
}
