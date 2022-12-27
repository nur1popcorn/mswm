use x11rb::errors::{ReplyError, ReplyOrIdError};
use x11rb::connection::Connection;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;

const MOD_KEY: Keycode = 0x85;
const MOVE_BUTTON: Button = 0x1;

pub struct WM {
    conn: RustConnection,
    screen_num: usize,
    mod_key_down: bool,
    window: Option<Window>
}

impl WM {
    pub fn create_wm(conn: RustConnection, screen_num: usize) -> Result<Self, ReplyError> {
        let screen = &conn.setup().roots[screen_num];
        let change = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::KEY_PRESS |
                        EventMask::KEY_RELEASE |
                        EventMask::SUBSTRUCTURE_REDIRECT |
                        EventMask::SUBSTRUCTURE_NOTIFY);
        // only one X client can select substructure redirection.
        conn.change_window_attributes(screen.root, &change)?.check()?;
        Ok(Self { conn, screen_num, mod_key_down: false, window: None })
    }

    pub fn scan(&self) -> Result<(), ReplyOrIdError> {
        let screen = &self.conn.setup().roots[self.screen_num];
        for win in self.conn.query_tree(screen.root)?.reply()?.children {
            let attr = self.conn.get_window_attributes(win)?.reply()?;
            if attr.map_state != MapState::UNMAPPED {
                self.manage(win)?;
            }
        }
        Ok(())
    }

    fn manage(&self, win: Window) -> Result<(), ReplyOrIdError> {
        let screen = &self.conn.setup().roots[self.screen_num];
        let geom = self.conn.get_geometry(win)?.reply()?;
        let frame_win = self.conn.generate_id()?;
        let win_aux = CreateWindowAux::new()
            .event_mask(EventMask::BUTTON_PRESS |
                        EventMask::BUTTON_RELEASE |
                        EventMask::POINTER_MOTION)
            .background_pixel(screen.white_pixel);

        self.conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame_win,
            screen.root,
            geom.x, geom.y,
            geom.width, geom.height,
            1,
            WindowClass::INPUT_OUTPUT,
            0,
            &win_aux,
        )?;

        self.conn.grab_server()?;
        self.conn.reparent_window(win, frame_win, 0, 0)?;
        self.conn.map_window(win)?;
        self.conn.map_window(frame_win)?;
        self.conn.ungrab_server()?;
        self.conn.flush()?;
        Ok(())
    }

    fn handle_key_press(&mut self, event: KeyPressEvent) {
        if event.detail == MOD_KEY { self.mod_key_down = true; }
    }

    fn handle_key_release(&mut self, event: KeyPressEvent) {
        if event.detail == MOD_KEY { self.mod_key_down = false; }
    }

    fn handle_button_press(&mut self, event: ButtonPressEvent) {
        if self.mod_key_down && event.detail == MOVE_BUTTON {
            self.window = Some(event.event);
        }
    }

    fn handle_button_release(&mut self, event: ButtonReleaseEvent) {
        if event.detail == MOVE_BUTTON { self.window = None; }
    }

    fn handle_motion_notify(&self, event: MotionNotifyEvent) -> Result<(), ReplyError> {
        if let Some(window) = self.window {
            let (x, y) = (event.root_x, event.root_y);
            let (x, y) = (x as i32, y as i32);
            self.conn.configure_window(window,
                &ConfigureWindowAux::new().x(x).y(y))?;
            self.conn.flush()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<(), ReplyOrIdError> {
        let mut event_opt = Some(self.conn.wait_for_event()?);
        while let Some(event) = event_opt {
            match event {
                Event::KeyPress(event) => self.handle_key_press(event),
                Event::KeyRelease(event) => self.handle_key_release(event),
                Event::ButtonPress(event) => self.handle_button_press(event),
                Event::ButtonRelease(event) => self.handle_button_release(event),
                Event::MotionNotify(event) => self.handle_motion_notify(event)?,
                Event::MapRequest(event) => self.manage(event.window)?,
                _ => { }
            }
            // check if more events are already available.
            event_opt = self.conn.poll_for_event()?
        }
        Ok(())
    }
}
