use libc;

use crate::error::crossterm_error;

#[rustfmt::skip]
mod unformatted {

pub const CROSSTERM_NO_KEY_MODIFIERS:       u16 = 0b0000000000000000;
pub const CROSSTERM_SHIFT_KEY_MODIFIER:     u16 = 0b0000000000000001;
pub const CROSSTERM_CONTROL_KEY_MODIFIER:   u16 = 0b0000000000000010;
pub const CROSSTERM_ALT_KEY_MODIFIER:       u16 = 0b0000000000000100;
pub const CROSSTERM_SUPER_KEY_MODIFIER:     u16 = 0b0000000000001000;
pub const CROSSTERM_HYPER_KEY_MODIFIER:     u16 = 0b0000000000010000;
pub const CROSSTERM_META_KEY_MODIFIER:      u16 = 0b0000000000100000;
pub const CROSSTERM_KEYPAD_KEY_MODIFIER:    u16 = 0b0000000001000000;
pub const CROSSTERM_CAPS_LOCK_KEY_MODIFIER: u16 = 0b0000000010000000;
pub const CROSSTERM_NUM_LOCK_KEY_MODIFIER:  u16 = 0b0000000100000000;
#[allow(dead_code)]
pub const CROSSTERM_ALL_KEY_MODIFIERS:      u16 = 0b0000000111111111;

}

pub use unformatted::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum crossterm_event_type {
    CROSSTERM_KEY_EVENT,
    CROSSTERM_RESIZE_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum crossterm_key_type {
    CROSSTERM_CHAR_KEY = 0,
    CROSSTERM_BACKSPACE_KEY,
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
#[derive(Copy, Clone)]
pub struct crossterm_key_event {
    t: crossterm_key_type,
    code: u32,
    modifiers: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crossterm_resize_event {
    width: u16,
    height: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crossterm_event {
    t: crossterm_event_type,
    v: crossterm_event_,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union crossterm_event_ {
    key: crossterm_key_event,
    resize: crossterm_resize_event,
}

#[rustfmt::skip]
fn pack_key_modifiers(
    modifiers: crossterm::event::KeyModifiers,
    state: crossterm::event::KeyEventState,
) -> u16 {
    use crossterm::event::KeyEventState;
    use crossterm::event::KeyModifiers;

    assert_eq!(KeyModifiers::SHIFT.bits()      as u16, CROSSTERM_SHIFT_KEY_MODIFIER);
    assert_eq!(KeyModifiers::CONTROL.bits()    as u16, CROSSTERM_CONTROL_KEY_MODIFIER);
    assert_eq!(KeyModifiers::ALT.bits()        as u16, CROSSTERM_ALT_KEY_MODIFIER);
    assert_eq!(KeyModifiers::SUPER.bits()      as u16, CROSSTERM_SUPER_KEY_MODIFIER);
    assert_eq!(KeyModifiers::HYPER.bits()      as u16, CROSSTERM_HYPER_KEY_MODIFIER);
    assert_eq!(KeyModifiers::META.bits()       as u16, CROSSTERM_META_KEY_MODIFIER);

    assert_eq!(KeyEventState::KEYPAD.bits()    as u16, CROSSTERM_KEYPAD_KEY_MODIFIER    >> 6);
    assert_eq!(KeyEventState::CAPS_LOCK.bits() as u16, CROSSTERM_CAPS_LOCK_KEY_MODIFIER >> 6);
    assert_eq!(KeyEventState::NUM_LOCK.bits()  as u16, CROSSTERM_NUM_LOCK_KEY_MODIFIER  >> 6);

    let mut res = CROSSTERM_NO_KEY_MODIFIERS;
    res |= modifiers.bits() as u16;
    res |= (state.bits() as u16) << 7;
    return res;
}

#[rustfmt::skip]
unsafe fn crossterm_event_read_key(kev: crossterm::event::KeyEvent, event: *mut crossterm_event) {
    use crossterm::event::KeyCode::*;
    use crossterm_event_type::*;
    use crossterm_key_type::*;

    (*event).t = CROSSTERM_KEY_EVENT;
    (*event).v.key.modifiers = pack_key_modifiers(kev.modifiers, kev.state);

    match kev.code {
        Char(char) => {
            (*event).v.key.t = CROSSTERM_CHAR_KEY;
            (*event).v.key.code = char as u32;
        }
        Backspace   => (*event).v.key.t = CROSSTERM_BACKSPACE_KEY,
        Enter       => (*event).v.key.t = CROSSTERM_ENTER_KEY,
        Left        => (*event).v.key.t = CROSSTERM_LEFT_ARROW_KEY,
        Right       => (*event).v.key.t = CROSSTERM_RIGHT_ARROW_KEY,
        Up          => (*event).v.key.t = CROSSTERM_UP_ARROW_KEY,
        Down        => (*event).v.key.t = CROSSTERM_DOWN_ARROW_KEY,
        Home        => (*event).v.key.t = CROSSTERM_HOME_KEY,
        End         => (*event).v.key.t = CROSSTERM_END_KEY,
        PageUp      => (*event).v.key.t = CROSSTERM_PAGE_UP_KEY,
        PageDown    => (*event).v.key.t = CROSSTERM_PAGE_DOWN_KEY,
        Tab         => (*event).v.key.t = CROSSTERM_TAB_KEY,
        BackTab     => (*event).v.key.t = CROSSTERM_BACKTAB_KEY,
        Delete      => (*event).v.key.t = CROSSTERM_DELETE_KEY,
        Insert      => (*event).v.key.t = CROSSTERM_INSERT_KEY,
        Esc         => (*event).v.key.t = CROSSTERM_ESCAPE_KEY,
        F(1)        => (*event).v.key.t = CROSSTERM_F1_KEY,
        F(2)        => (*event).v.key.t = CROSSTERM_F2_KEY,
        F(3)        => (*event).v.key.t = CROSSTERM_F3_KEY,
        F(4)        => (*event).v.key.t = CROSSTERM_F4_KEY,
        F(5)        => (*event).v.key.t = CROSSTERM_F5_KEY,
        F(6)        => (*event).v.key.t = CROSSTERM_F6_KEY,
        F(7)        => (*event).v.key.t = CROSSTERM_F7_KEY,
        F(8)        => (*event).v.key.t = CROSSTERM_F8_KEY,
        F(9)        => (*event).v.key.t = CROSSTERM_F9_KEY,
        F(10)       => (*event).v.key.t = CROSSTERM_F10_KEY,
        F(11)       => (*event).v.key.t = CROSSTERM_F11_KEY,
        F(12)       => (*event).v.key.t = CROSSTERM_F12_KEY,
        F(_)        => unreachable!("F what?"),
        Null        => {}
        CapsLock    => {}
        ScrollLock  => {}
        NumLock     => {}
        PrintScreen => {}
        Pause       => {}
        Menu        => {}
        KeypadBegin => {}
        Media(_)    => {}
        Modifier(_) => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_event_read(event: *mut crossterm_event) -> libc::c_int {
    use crossterm::event::Event::*;
    use crossterm_event_type::*;

    match crossterm::event::read() {
        Ok(ev) => match ev {
            FocusGained => {}
            FocusLost => {}
            Key(kev) => crossterm_event_read_key(kev, event),
            Mouse(_) => {}
            Paste(_) => {}
            Resize(rows, cols) => {
                (*event).t = CROSSTERM_RESIZE_EVENT;
                (*event).v.resize.width = cols;
                (*event).v.resize.height = rows;
            }
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
