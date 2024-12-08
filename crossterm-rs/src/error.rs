use libc;

#[repr(C)]
pub enum crossterm_error {
    CROSSTERM_SUCCESS = 0,
    CROSSTERM_EUNDEF,
    CROSSTERM_EOS,
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
