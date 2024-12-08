use libc;

#[repr(C)]
pub enum crossterm_error {
    CROSSTERM_SUCCESS = 0,
    CROSSTERM_EUNDEF,
    CROSSTERM_EOS,
}

fn to_const_char_ptr(str: &'static str) -> *const libc::c_char {
    return str.as_ptr() as *const libc::c_char;
}

impl From<std::io::Error> for crossterm_error {
    fn from(err: std::io::Error) -> crossterm_error {
        if let Some(eos) = err.raw_os_error() {
            unsafe {
                *libc::__errno_location() = eos;
            }
            return crossterm_error::CROSSTERM_EOS;
        } else {
            return crossterm_error::CROSSTERM_EUNDEF;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_strerror(error: crossterm_error) -> *const libc::c_char {
    use crossterm_error::*;
    match error {
        CROSSTERM_SUCCESS => to_const_char_ptr("success"),
        CROSSTERM_EOS => libc::strerror(*libc::__errno_location()),
        _ => to_const_char_ptr("undefined error"),
    }
}

