use crate::error::crossterm_error;
use crate::stream::crossterm_stream;

#[no_mangle]
pub unsafe extern "C" fn crossterm_enter_alternate_screen(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    let ret = crossterm::execute!(*stream, crossterm::terminal::EnterAlternateScreen);
    if let Err(err) = ret {
        if let Some(eos) = err.raw_os_error() {
            *libc::__errno_location() = eos;
            return -(crossterm_error::CROSSTERM_EOS as i32);
        } else {
            return -(crossterm_error::CROSSTERM_EUNDEF as i32);
        }
    } else {
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_leave_alternate_screen(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    let ret = crossterm::execute!(*stream, crossterm::terminal::LeaveAlternateScreen);
    if let Err(err) = ret {
        if let Some(eos) = err.raw_os_error() {
            *libc::__errno_location() = eos;
            return -(crossterm_error::CROSSTERM_EOS as i32);
        } else {
            return -(crossterm_error::CROSSTERM_EUNDEF as i32);
        }
    } else {
        return 0;
    }
}
