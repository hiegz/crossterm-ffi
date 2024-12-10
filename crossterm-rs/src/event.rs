use libc;

use crate::error::crossterm_error;

#[rustfmt::skip]
mod unformatted {

pub const CROSSTERM_NO_KEY_MODIFIERS:       u16 = 0b0000000000000000;
pub const CROSSTERM_CONTROL_KEY_MODIFIER:   u16 = 0b0000000000000001;
pub const CROSSTERM_SHIFT_KEY_MODIFIER:     u16 = 0b0000000000000010;
pub const CROSSTERM_ALT_KEY_MODIFIER:       u16 = 0b0000000000000100;
pub const CROSSTERM_CAPS_LOCK_KEY_MODIFIER: u16 = 0b0000000000001000;
pub const CROSSTERM_NUM_LOCK_KEY_MODIFIER:  u16 = 0b0000000000010000;
pub const CROSSTERM_KEYPAD_KEY_MODIFIER:    u16 = 0b0000000000100000;
pub const CROSSTERM_SUPER_KEY_MODIFIER:     u16 = 0b0000000001000000;
pub const CROSSTERM_HYPER_KEY_MODIFIER:     u16 = 0b0000000010000000;
pub const CROSSTERM_META_KEY_MODIFIER:      u16 = 0b0000000100000000;
pub const CROSSTERM_ALL_KEY_MODIFIERS:      u16 = 0b0000000111111111;

pub type crossterm_character_key_event_handler  = Option<unsafe extern "C" fn(u32, u16, *mut libc::c_void)>;
pub type crossterm_special_key_event_handler    = Option<unsafe extern "C" fn (u8, u16, *mut libc::c_void)>;
pub type crossterm_resize_event_handler         = Option<unsafe extern "C" fn(u16, u16, *mut libc::c_void)>;

}

pub use unformatted::*;

#[repr(C)]
pub enum crossterm_special_key {
    CROSSTERM_BACKSPACE_KEY = 0,
    CROSSTERM_ENTER_KEY,
    CROSSTERM_LEFT_ARROW_KEY,
    CROSSTERM_RIGHT_ARROW_KEY,
    CROSSTERM_UP_ARROW_KEY,
    CROSSTERM_DOWN_ARROW_KEY,
    CROSSTERM_HOME_KEY,
    CROSSTERM_END_KEY,
    CROSSTERM_PAGE_UP_KEY,
    CROSSTERM_PAGE_DOWN_KEY,
    CROSSTERM_TAB_KEY,
    CROSSTERM_BACKTAB_KEY,
    CROSSTERM_DELETE_KEY,
    CROSSTERM_INSERT_KEY,
    CROSSTERM_ESCAPE_KEY,

    CROSSTERM_F1_KEY = 244,
    CROSSTERM_F2_KEY,
    CROSSTERM_F3_KEY,
    CROSSTERM_F4_KEY,
    CROSSTERM_F5_KEY,
    CROSSTERM_F6_KEY,
    CROSSTERM_F7_KEY,
    CROSSTERM_F8_KEY,
    CROSSTERM_F9_KEY,
    CROSSTERM_F10_KEY,
    CROSSTERM_F11_KEY,
    CROSSTERM_F12_KEY,
}

#[repr(C)]
pub struct crossterm_event_executor {
    pub character_key_event_handler: crossterm_character_key_event_handler,
    pub special_key_event_handler: crossterm_special_key_event_handler,
    pub resize_event_handler: crossterm_resize_event_handler,

    pub handler_data: *mut libc::c_void,
}

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
    #[rustfmt::skip]
    pub unsafe fn maybe_execute_key_event(&self, event: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode::*;
        use crossterm_special_key::*;

        let modifiers = pack_key_modifiers(event.modifiers, event.state);

        match event.code {
            Backspace => self.maybe_execute_special_key_event(CROSSTERM_BACKSPACE_KEY   as u8, modifiers),
            Enter     => self.maybe_execute_special_key_event(CROSSTERM_ENTER_KEY       as u8, modifiers),
            Left      => self.maybe_execute_special_key_event(CROSSTERM_LEFT_ARROW_KEY  as u8, modifiers),
            Right     => self.maybe_execute_special_key_event(CROSSTERM_RIGHT_ARROW_KEY as u8, modifiers),
            Up        => self.maybe_execute_special_key_event(CROSSTERM_UP_ARROW_KEY    as u8, modifiers),
            Down      => self.maybe_execute_special_key_event(CROSSTERM_DOWN_ARROW_KEY  as u8, modifiers),
            Home      => self.maybe_execute_special_key_event(CROSSTERM_HOME_KEY        as u8, modifiers),
            End       => self.maybe_execute_special_key_event(CROSSTERM_END_KEY         as u8, modifiers),
            PageUp    => self.maybe_execute_special_key_event(CROSSTERM_PAGE_UP_KEY     as u8, modifiers),
            PageDown  => self.maybe_execute_special_key_event(CROSSTERM_PAGE_DOWN_KEY   as u8, modifiers),
            Tab       => self.maybe_execute_special_key_event(CROSSTERM_TAB_KEY         as u8, modifiers),
            BackTab   => self.maybe_execute_special_key_event(CROSSTERM_BACKTAB_KEY     as u8, modifiers),
            Delete    => self.maybe_execute_special_key_event(CROSSTERM_DELETE_KEY      as u8, modifiers),
            Insert    => self.maybe_execute_special_key_event(CROSSTERM_INSERT_KEY      as u8, modifiers),
            Esc       => self.maybe_execute_special_key_event(CROSSTERM_ESCAPE_KEY      as u8, modifiers),
            F(1)      => self.maybe_execute_special_key_event(CROSSTERM_F1_KEY          as u8, modifiers),
            F(2)      => self.maybe_execute_special_key_event(CROSSTERM_F2_KEY          as u8, modifiers),
            F(3)      => self.maybe_execute_special_key_event(CROSSTERM_F3_KEY          as u8, modifiers),
            F(4)      => self.maybe_execute_special_key_event(CROSSTERM_F4_KEY          as u8, modifiers),
            F(5)      => self.maybe_execute_special_key_event(CROSSTERM_F5_KEY          as u8, modifiers),
            F(6)      => self.maybe_execute_special_key_event(CROSSTERM_F6_KEY          as u8, modifiers),
            F(7)      => self.maybe_execute_special_key_event(CROSSTERM_F7_KEY          as u8, modifiers),
            F(8)      => self.maybe_execute_special_key_event(CROSSTERM_F8_KEY          as u8, modifiers),
            F(9)      => self.maybe_execute_special_key_event(CROSSTERM_F9_KEY          as u8, modifiers),
            F(10)     => self.maybe_execute_special_key_event(CROSSTERM_F10_KEY         as u8, modifiers),
            F(11)     => self.maybe_execute_special_key_event(CROSSTERM_F11_KEY         as u8, modifiers),
            F(12)     => self.maybe_execute_special_key_event(CROSSTERM_F12_KEY         as u8, modifiers),
            F(_)      => unreachable!("F what?"),
            Char(ch)  => self.maybe_execute_character_key_event(ch as u32, modifiers),
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

    pub unsafe fn maybe_execute_character_key_event(&self, character: u32, modifiers: u16) {
        if let Some(handler) = &self.character_key_event_handler {
            (handler)(character, modifiers, self.handler_data);
        }
    }

    pub unsafe fn maybe_execute_special_key_event(&self, character: u8, modifiers: u16) {
        if let Some(handler) = &self.special_key_event_handler {
            (handler)(character, modifiers, self.handler_data);
        }
    }

    pub unsafe fn maybe_execute_resize_event(&self, rows: u16, cols: u16) {
        if let Some(handler) = &self.resize_event_handler {
            (handler)(rows, cols, self.handler_data);
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
            Key(kev) => (&*event_executor).maybe_execute_key_event(kev),
            Mouse(_) => {}
            Paste(_) => {}
            Resize(rows, cols) => (&*event_executor).maybe_execute_resize_event(rows, cols),
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
