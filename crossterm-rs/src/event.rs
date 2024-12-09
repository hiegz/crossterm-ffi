use libc;

use crate::error::crossterm_error;

#[rustfmt::skip]
mod unformatted {

pub const CROSSTERM_NO_KEY_MODIFIERS:        u16 = 0b0000000000000000;
pub const CROSSTERM_CONTROL_KEY_MODIFIER:    u16 = 0b0000000000000001;
pub const CROSSTERM_SHIFT_KEY_MODIFIER:      u16 = 0b0000000000000010;
pub const CROSSTERM_ALT_KEY_MODIFIER:        u16 = 0b0000000000000100;
pub const CROSSTERM_CAPS_LOCK_KEY_MODIFIER:  u16 = 0b0000000000001000;
pub const CROSSTERM_NUM_LOCK_KEY_MODIFIER:   u16 = 0b0000000000010000;
pub const CROSSTERM_KEYPAD_KEY_MODIFIER:     u16 = 0b0000000000100000;
pub const CROSSTERM_SUPER_KEY_MODIFIER:      u16 = 0b0000000001000000;
pub const CROSSTERM_HYPER_KEY_MODIFIER:      u16 = 0b0000000010000000;
pub const CROSSTERM_META_KEY_MODIFIER:       u16 = 0b0000000100000000;
pub const CROSSTERM_ALL_KEY_MODIFIERS:       u16 = 0b0000000111111111;

pub type crossterm_character_key_event_handler   = Option<unsafe extern "C" fn(u32, u16, *mut libc::c_void)>;
pub type crossterm_f_key_event_handler           = Option<unsafe extern "C" fn (u8, u16, *mut libc::c_void)>;
pub type crossterm_backspace_key_event_handler   = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_enter_key_event_handler       = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_left_arrow_key_event_handler  = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_right_arrow_key_event_handler = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_up_arrow_key_event_handler    = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_down_arrow_key_event_handler  = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_home_key_event_handler        = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_end_key_event_handler         = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_page_up_key_event_handler     = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_page_down_key_event_handler   = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_tab_key_event_handler         = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_backtab_key_event_handler     = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_delete_key_event_handler      = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_insert_key_event_handler      = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_escape_key_event_handler      = Option<unsafe extern "C" fn     (u16, *mut libc::c_void)>;
pub type crossterm_resize_event_handler          = Option<unsafe extern "C" fn(u16, u16, *mut libc::c_void)>;

#[repr(C)]
pub struct crossterm_event_executor {
    pub character_key_event_handler:         crossterm_character_key_event_handler,
    pub character_key_event_handler_token:   *mut libc::c_void,
    pub f_key_event_handler:                 crossterm_f_key_event_handler,
    pub f_key_event_handler_token:           *mut libc::c_void,
    pub backspace_key_event_handler:         crossterm_backspace_key_event_handler,
    pub backspace_key_event_handler_token:   *mut libc::c_void,
    pub enter_key_event_handler:             crossterm_enter_key_event_handler,
    pub enter_key_event_handler_token:       *mut libc::c_void,
    pub left_arrow_key_event_handler:        crossterm_left_arrow_key_event_handler,
    pub left_arrow_key_event_handler_token:  *mut libc::c_void,   
    pub right_arrow_key_event_handler:       crossterm_right_arrow_key_event_handler,
    pub right_arrow_key_event_handler_token: *mut libc::c_void,
    pub up_arrow_key_event_handler:          crossterm_up_arrow_key_event_handler,
    pub up_arrow_key_event_handler_token:    *mut libc::c_void,
    pub down_arrow_key_event_handler:        crossterm_down_arrow_key_event_handler,
    pub down_arrow_key_event_handler_token:  *mut libc::c_void,
    pub home_key_event_handler:              crossterm_home_key_event_handler,
    pub home_key_event_handler_token:        *mut libc::c_void,
    pub end_key_event_handler:               crossterm_end_key_event_handler,
    pub end_key_event_handler_token:         *mut libc::c_void,
    pub page_up_key_event_handler:           crossterm_page_up_key_event_handler,
    pub page_up_key_event_handler_token:     *mut libc::c_void,
    pub page_down_key_event_handler:         crossterm_page_down_key_event_handler,
    pub page_down_key_event_handler_token:   *mut libc::c_void,
    pub tab_key_event_handler:               crossterm_tab_key_event_handler,
    pub tab_key_event_handler_token:         *mut libc::c_void,
    pub backtab_key_event_handler:           crossterm_backtab_key_event_handler,
    pub backtab_key_event_handler_token:     *mut libc::c_void,
    pub delete_key_event_handler:            crossterm_delete_key_event_handler,
    pub delete_key_event_handler_token:      *mut libc::c_void,
    pub insert_key_event_handler:            crossterm_insert_key_event_handler,
    pub insert_key_event_handler_token:      *mut libc::c_void,
    pub escape_key_event_handler:            crossterm_escape_key_event_handler,
    pub escape_key_event_handler_token:      *mut libc::c_void,
    pub resize_event_handler:                crossterm_resize_event_handler,
    pub resize_event_handler_token:          *mut libc::c_void,
}

}

pub use unformatted::*;

fn pack_key_modifiers(
    modifiers: crossterm::event::KeyModifiers,
    state: crossterm::event::KeyEventState,
) -> u16 {
    use crossterm::event::KeyEventState;
    use crossterm::event::KeyModifiers;

    let mut res = CROSSTERM_NO_KEY_MODIFIERS;
    if modifiers.contains(KeyModifiers::CONTROL) {
        res |= CROSSTERM_CONTROL_KEY_MODIFIER;
    }
    if modifiers.contains(KeyModifiers::SHIFT) {
        res |= CROSSTERM_SHIFT_KEY_MODIFIER;
    }
    if modifiers.contains(KeyModifiers::ALT) {
        res |= CROSSTERM_ALT_KEY_MODIFIER;
    }
    if modifiers.contains(KeyModifiers::SUPER) {
        res |= CROSSTERM_SUPER_KEY_MODIFIER;
    }
    if modifiers.contains(KeyModifiers::HYPER) {
        res |= CROSSTERM_HYPER_KEY_MODIFIER;
    }
    if modifiers.contains(KeyModifiers::META) {
        res |= CROSSTERM_META_KEY_MODIFIER;
    }
    if state.contains(KeyEventState::KEYPAD) {
        res |= CROSSTERM_KEYPAD_KEY_MODIFIER;
    }
    if state.contains(KeyEventState::CAPS_LOCK) {
        res |= CROSSTERM_CAPS_LOCK_KEY_MODIFIER;
    }
    if state.contains(KeyEventState::NUM_LOCK) {
        res |= CROSSTERM_NUM_LOCK_KEY_MODIFIER;
    }
    return res;
}

impl crossterm_event_executor {
    pub unsafe fn execute_key_event(&self, event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode::*;

        match event.code {
            Backspace => self.execute_backspace_key_event(event),
            Enter => self.execute_enter_key_event(event),
            Left => self.execute_left_arrow_key_event(event),
            Right => self.execute_right_arrow_key_event(event),
            Up => self.execute_up_arrow_key_event(event),
            Down => self.execute_down_arrow_key_event(event),
            Home => self.execute_home_key_event(event),
            End => self.execute_end_key_event(event),
            PageUp => self.execute_page_up_key_event(event),
            PageDown => self.execute_page_down_key_event(event),
            Tab => self.execute_tab_key_event(event),
            BackTab => self.execute_backtab_key_event(event),
            Delete => self.execute_delete_key_event(event),
            Insert => self.execute_insert_key_event(event),
            F(n) => self.execute_f_key_event(n, event),
            Char(ch) => self.execute_character_key_event(ch, event),
            Esc => self.execute_escape_key_event(event),
            Null => {}
            CapsLock => {}
            ScrollLock => {}
            NumLock => {}
            PrintScreen => {}
            Pause => {}
            Menu => {}
            KeypadBegin => {}
            Media(_) => {}
            Modifier(_) => {}
        }
    }

    pub unsafe fn execute_backspace_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Backspace);
        if let Some(handler) = &self.backspace_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.backspace_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_enter_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Enter);
        if let Some(handler) = &self.enter_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.enter_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_left_arrow_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Left);
        if let Some(handler) = &self.left_arrow_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.left_arrow_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_right_arrow_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Right);
        if let Some(handler) = &self.right_arrow_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.right_arrow_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_up_arrow_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Up);
        if let Some(handler) = &self.up_arrow_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.up_arrow_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_down_arrow_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Down);
        if let Some(handler) = &self.down_arrow_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.down_arrow_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_home_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Home);
        if let Some(handler) = &self.home_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.home_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_end_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::End);
        if let Some(handler) = &self.end_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.end_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_page_up_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::PageUp);
        if let Some(handler) = &self.page_up_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.page_up_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_page_down_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::PageDown);
        if let Some(handler) = &self.page_down_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.page_down_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_tab_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Tab);
        if let Some(handler) = &self.tab_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.tab_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_backtab_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::BackTab);
        if let Some(handler) = &self.backtab_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.backtab_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_delete_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Delete);
        if let Some(handler) = &self.delete_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.delete_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_insert_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Insert);
        if let Some(handler) = &self.insert_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.insert_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_f_key_event(&self, n: u8, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::F(n));
        if let Some(handler) = &self.f_key_event_handler {
            (handler)(
                n,
                pack_key_modifiers(event.modifiers, event.state),
                self.f_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_character_key_event(
        &self,
        character: char,
        event: crossterm::event::KeyEvent,
    ) {
        assert_eq!(event.code, crossterm::event::KeyCode::Char(character));
        if let Some(handler) = &self.character_key_event_handler {
            (handler)(
                character as u32,
                pack_key_modifiers(event.modifiers, event.state),
                self.character_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_escape_key_event(&self, event: crossterm::event::KeyEvent) {
        assert_eq!(event.code, crossterm::event::KeyCode::Esc);
        if let Some(handler) = &self.escape_key_event_handler {
            (handler)(
                pack_key_modifiers(event.modifiers, event.state),
                self.escape_key_event_handler_token,
            );
        }
    }

    pub unsafe fn execute_resize_event(&self, rows: u16, cols: u16) {
        if let Some(handler) = &self.resize_event_handler {
            (handler)(rows, cols, self.resize_event_handler_token);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_event_executor_init(
    event_executor: *mut crossterm_event_executor,
) {
    libc::memset(
        event_executor as *mut libc::c_void,
        0x00,
        size_of::<crossterm_event_executor>(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_event_execute(
    event_executor: *mut crossterm_event_executor,
) -> libc::c_int {
    use crossterm::event::Event::*;

    let ret = crossterm::event::read();
    match ret {
        Ok(ev) => match ev {
            FocusGained => {}
            FocusLost => {}
            Key(kev) => (&*event_executor).execute_key_event(kev),
            Mouse(_) => {}
            Paste(_) => {}
            Resize(rows, cols) => (&*event_executor).execute_resize_event(rows, cols),
        },
        Err(err) => return -(crossterm_error::from(err) as i32),
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_event_poll(is_available: *mut libc::c_int) -> libc::c_int {
    let ret = crossterm::event::poll(std::time::Duration::from_secs(0));
    match ret {
        Ok(available) => *is_available = available.into(),
        Err(err) => return -(crossterm_error::from(err) as i32),
    }
    return 0;
}
