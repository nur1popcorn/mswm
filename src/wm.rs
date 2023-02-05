use std::alloc::Layout;
use std::cmp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use x11rb::connection::Connection;
use x11rb::errors::{ReplyError, ReplyOrIdError};
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::COPY_DEPTH_FROM_PARENT;

use crate::config::*;
use crate::keybind::KeyBind;
use crate::layout::{FibonacciLayout, WindowLayout};

pub struct WM {
    conn: RustConnection,
    screen_num: usize,

    move_flag: bool,
    window: Option<(Window, i16, i16, i32, i32, i32, i32)>,
    focused: Option<Window>,

    gc: Gcontext,
    sequence_ignore: BinaryHeap<Reverse<u16>>,
    window_map: HashMap<Window, Window>,
    window_map_reverse: HashMap<Window, Window>,
}

impl WM {

    pub fn create_wm(conn: RustConnection, screen_num: usize) -> Result<Self, ReplyOrIdError> {
        let screen = &conn.setup().roots[screen_num];

        KeyBind::get_keymap(&conn);

        let change = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::POINTER_MOTION |
                        EventMask::BUTTON_PRESS |
                        EventMask::BUTTON_RELEASE |
                        EventMask::SUBSTRUCTURE_NOTIFY |
                        EventMask::SUBSTRUCTURE_REDIRECT);
        // only one X client can select substructure redirection.
        conn.change_window_attributes(screen.root, &change)?.check()?;

        // create the graphics context
        let gc = conn.generate_id()?;
        let font = conn.generate_id()?;
        conn.open_font(font, b"9x15")?;
        conn.create_gc(gc, screen.root, &CreateGCAux::new()
            .graphics_exposures(0)
            .background(screen.black_pixel)
            .font(font))?;
        conn.close_font(font)?;

        Ok(Self {
            conn,
            screen_num,
            move_flag: false,
            window: None,
            focused: None,
            gc,
            sequence_ignore: BinaryHeap::new(),
            window_map: HashMap::new(),
            window_map_reverse: HashMap::new(),
        })
    }

    pub fn scan(&mut self) -> Result<(), ReplyOrIdError> {
        let screen = &self.conn.setup().roots[self.screen_num];
        let children = self.conn.query_tree(screen.root)?.reply()?.children;
        for win in children {
            let attr = self.conn.get_window_attributes(win)?.reply()?;
            if attr.map_state != MapState::UNMAPPED && !attr.override_redirect {
                self.manage(win)?;
            }
        }
        Ok(())
    }

    fn manage(&mut self, win: Window) -> Result<(), ReplyOrIdError> {
        let screen = &self.conn.setup().roots[self.screen_num];
        let geom = self.conn.get_geometry(win)?.reply()?;
        let frame_win = self.conn.generate_id()?;
        self.window_map.insert(win, frame_win);
        self.window_map_reverse.insert(frame_win, win);

        let win_aux = CreateWindowAux::new()
            .event_mask(EventMask::ENTER_WINDOW |
                        EventMask::LEAVE_WINDOW |
                        EventMask::SUBSTRUCTURE_NOTIFY |
                        EventMask::SUBSTRUCTURE_REDIRECT)
            .background_pixel(screen.white_pixel);
        self.conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame_win,
            screen.root,
            geom.x,
            geom.y + (TOP_BAR_HEIGHT as i16),
            geom.width,
            geom.height,
            1,
            WindowClass::INPUT_OUTPUT,
            0,
            &win_aux,
        )?;

        self.conn.grab_server()?;
        let cookie = self.conn.reparent_window(win, frame_win, 0, 0)?;
        self.sequence_ignore.push(
            Reverse(cookie.sequence_number() as u16));
        self.conn.map_window(win)?;
        self.conn.map_window(frame_win)?;
        self.grab_buttons(win)?;
        self.conn.ungrab_server()?;
        self.conn.flush()?;
        Ok(())
    }

    fn unmanage(&mut self, win: Window) -> Result<(), ReplyError> {
        if let Some(parent) = self.window_map.remove(&win) {
            self.window_map_reverse.remove(&parent);
            let screen = &self.conn.setup().roots[self.screen_num];
            self.conn.reparent_window(win, screen.root, 0, 0)?;
            self.conn.unmap_window(parent)?;
            self.conn.destroy_window(parent)?;
            self.conn.flush()?;
        }
        Ok(())
    }

    fn handle_configure_request(&self, event: ConfigureRequestEvent) -> Result<(), ReplyError> {
        self.conn.configure_window(
            event.window,
            &ConfigureWindowAux::from_configure_request(&event),
        )?;
        self.conn.flush()?;
        Ok(())
    }

    fn do_layout(&mut self, layout: impl WindowLayout) -> Result<(), ReplyError> {
        let screen = &self.conn.setup().roots[self.screen_num];
        let children = self.conn.query_tree(screen.root)?.reply()?.children;
        let geom = self.conn.get_geometry(screen.root)?.reply().unwrap();
        let layout = layout.layout(Rectangle { x: 0, y: 0, width: geom.width, height: geom.height }, children);
        for (win, rect) in layout {
            let config = &ConfigureWindowAux::new()
                .width(rect.width as u32)
                .height(rect.height as u32);
            if let Some(parent) = self.window_map_reverse.get(&win) {
                self.conn.configure_window(*parent, &config)?;
            }
            self.conn.configure_window(win, &config
                .x(rect.x as i32)
                .y(rect.y as i32))?;
        }
        self.draw_top_bar()?;
        self.conn.flush()?;
        Ok(())
    }

    fn handle_button_press(&mut self, event: ButtonPressEvent) -> Result<(), ReplyError> {
        self.do_layout(FibonacciLayout { })?;

        self.move_flag = event.detail == MOVE_BUTTON;
        let state: u16 = event.state.into();
        let mask: u16 = MOD_MASK.into();
        if (state & mask) != 0 && (self.move_flag || event.detail == RESIZE_BUTTON) {
            if let Some(window) = self.window_map.get(&event.event) {
                let geom = self.conn.get_geometry(*window)?.reply().unwrap();
                self.window = Some((
                    event.event,
                    event.event_x,
                    event.event_y,
                    geom.x as i32,
                    geom.y as i32,
                    geom.width as i32,
                    geom.height as i32,
                ));
            }
        }
        Ok(())
    }

    fn handle_button_release(&mut self, event: ButtonReleaseEvent) {
        if ( self.move_flag && event.detail == MOVE_BUTTON) ||
           (!self.move_flag && event.detail == RESIZE_BUTTON) {
            self.window = None;
        }
    }

    fn handle_motion_notify(&self, event: MotionNotifyEvent) -> Result<(), ReplyError> {
        if let Some((window, x_offset, y_offset, w_x, w_y, width, height)) = self.window {
            let (x, y) = (event.root_x - x_offset, event.root_y - y_offset);
            let (x, y) = (x as i32, y as i32);
            if self.move_flag {
                // TODO: nicify if statements
                if let Some(parent) = self.window_map.get(&window) {
                    self.conn.configure_window(*parent, &ConfigureWindowAux::new().x(x).y(y))?;
                }
            } else {
                let (width, height) = (cmp::max(width + x - w_x, MIN_WIN_WIDTH), cmp::max(height + y - w_y, MIN_WIN_WIDTH));
                let (width, height) = (width as u32, height as u32);
                let config = ConfigureWindowAux::new().width(width).height(height);
                // TODO: nicify if statements
                if let Some(parent) = self.window_map.get(&window) {
                    self.conn.configure_window(*parent, &config)?;
                }
                self.conn.configure_window(window, &config)?;
            };
            self.conn.flush()?;
        }
        Ok(())
    }

    fn handle_enter_notify(&mut self, event: EnterNotifyEvent) {
        if event.child != 0 {
            // println!("{}", event.child);
            self.focused = Some(event.child);
        }
    }

    fn handle_leave_notify(&mut self, _event: LeaveNotifyEvent) {
        self.focused = None;
    }

    pub fn grab_buttons(&self, win: Window) -> Result<(), ReplyError> {
        self.conn.grab_button(
            false,
            win,
            EventMask::BUTTON_PRESS | EventMask::BUTTON_RELEASE | EventMask::POINTER_MOTION,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
            x11rb::NONE,
            x11rb::NONE,
            ButtonIndex::ANY,
            MOD_MASK,
        )?;
        Ok(())
    }

    pub fn draw_top_bar(&self) -> Result<(), ReplyError> {
        let root = self.conn.setup().roots[self.screen_num].root;
        let geom = self.conn.get_geometry(root)?.reply().unwrap();
        self.conn.change_gc(self.gc, &ChangeGCAux::new().foreground(TOP_BAR_COLOR))?;
        self.conn.poly_fill_rectangle(root, self.gc, &[
            Rectangle { x: 0, y: 0, width: geom.width, height: TOP_BAR_HEIGHT },
        ])?;
        self.conn.change_gc(self.gc, &ChangeGCAux::new()
            .foreground(TEXT_COLOR)
            .background(TOP_BAR_COLOR))?;

        if let Some(win) = self.focused {
            let p = self
                .conn
                .get_property(
                    false,
                    win,
                    AtomEnum::WM_NAME,
                    AtomEnum::STRING,
                    0,
                    u32::MAX,
                )?
                .reply()?;
            let p = p.value;
            self.conn.image_text8(
                root,
                self.gc,
                TOP_BAR_TEXT_OFFSET,
                TOP_BAR_HEIGHT as i16 - 4,
                String::from_utf8(p).unwrap().as_bytes()
            )?;
        }
        else {
            self.conn.image_text8(
                root,
                self.gc,
                TOP_BAR_TEXT_OFFSET,
                TOP_BAR_HEIGHT as i16 - 4,
                "MSWM".as_bytes()
            )?;
        }

        self.conn.flush()?;
        Ok(())
    }

    pub fn should_execute(&mut self, event: &Event) -> bool {
        if let Some(seq) = event.wire_sequence_number() {
            while let Some(&Reverse(ignore)) = self.sequence_ignore.peek() {
                if ignore.wrapping_sub(seq) <= u16::MAX / 2 {
                    return ignore != seq
                }
                self.sequence_ignore.pop();
            }
        }
        true
    }

    pub fn handle_events(&mut self) -> Result<(), ReplyOrIdError> {
        let mut event_opt = Some(self.conn.wait_for_event()?);
        while let Some(event) = &event_opt {
            if self.should_execute(&event) {
                match event {
                    Event::ConfigureRequest(event) => self.handle_configure_request(*event)?,
                    Event::ButtonPress(event) => self.handle_button_press(*event)?,
                    Event::ButtonRelease(event) => self.handle_button_release(*event),
                    Event::MotionNotify(event) => self.handle_motion_notify(*event)?,
                    Event::EnterNotify(event) => self.handle_enter_notify(*event),
                    Event::LeaveNotify(event) => self.handle_leave_notify(*event),
                    Event::MapRequest(event) => self.manage(event.window)?,
                    Event::UnmapNotify(event) => self.unmanage(event.window)?,
                    _ => {}
                }
            }
            // check if more events are already available.
            event_opt = self.conn.poll_for_event()?
        }
        self.draw_top_bar()?;
        Ok(())
    }
}
