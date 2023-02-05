use x11rb::protocol::xproto::{Rectangle, Window};
use crate::config::TOP_BAR_HEIGHT;

pub trait WindowLayout {
    fn layout(self, screen: Rectangle, windows: &Vec<Window>) -> Vec<(Window, Rectangle)>;
}

pub struct FibonacciLayout;

impl WindowLayout for FibonacciLayout {
    fn layout(self, screen: Rectangle, windows: &Vec<Window>) -> Vec<(Window, Rectangle)> {
        let mut result = Vec::with_capacity(windows.len());
        if windows.len() > 0 {
            let (mut x, mut y, mut width, mut height) = (0i16, TOP_BAR_HEIGHT as i16, screen.width, screen.height - TOP_BAR_HEIGHT);
            for i in 1..windows.len() {
                if i % 2 == 0 {
                    height /= 2;
                } else {
                    width /= 2;
                }
                result.push((windows[windows.len() - i], Rectangle { x, y, width, height }));

                if i % 2 == 0 {
                    y += height as i16;
                } else {
                    x += width as i16;
                }
            }
            result.push((windows[0], Rectangle { x, y, width, height }));
        }
        result
    }
}

pub struct TreeLayout;

impl WindowLayout for TreeLayout {

    fn layout(self, screen: Rectangle, windows: Vec<Window>) -> Vec<(Window, Rectangle)> {
        let mut result = Vec::with_capacity(windows.len());

        if windows.len() > 0 {
            let mut nr_leafs = 0;
            let mut leafs: Vec<(Window, Rectangle, bool)> = Vec::with_capacity(windows.len() * 2);
            leafs.push((windows[0], Rectangle { x: 0, y: TOP_BAR_HEIGHT as i16, width: screen.width, height: screen.height - TOP_BAR_HEIGHT }, true));

            while nr_leafs < windows.len() - 1 {
                let mut root: &(Window, Rectangle, bool) = &leafs[nr_leafs].clone();
                if root.2 {
                    leafs.push((
                        root.0,
                        Rectangle {
                            x: root.1.x,
                            y: root.1.y,
                            width: root.1.width / 2,
                            height: root.1.height
                        },
                        false
                    ));
                    leafs.push((
                        windows[nr_leafs + 1],
                        Rectangle {
                            x: root.1.x + ((root.1.width / 2) as i16),
                            y: root.1.y,
                            width: root.1.width / 2,
                            height: root.1.height
                        },
                        false
                    ));
                } else {
                    leafs.push((
                        root.0,
                        Rectangle {
                            x: root.1.x,
                            y: root.1.y,
                            width: root.1.width,
                            height: root.1.height / 2
                        },
                        true
                    ));
                    leafs.push((
                        windows[nr_leafs + 1],
                        Rectangle {
                            x: root.1.x,
                            y: root.1.y + ((root.1.height / 2) as i16),
                            width: root.1.width,
                            height: root.1.height / 2
                        },
                        true
                    ));
                }
                nr_leafs += 1;
            }

            for i in nr_leafs..leafs.len() {
                result.push((leafs[i].0, leafs[i].1));
            }
        }

        result
    }
}

//TODO implement WindowLayout
