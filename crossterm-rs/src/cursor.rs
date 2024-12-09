use crate::error::crossterm_error;
use crate::stream::crossterm_stream;

#[no_mangle]
pub unsafe extern "C" fn crossterm_show_cursor(stream: *mut crossterm_stream) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::Show);
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_hide_cursor(stream: *mut crossterm_stream) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::Hide);
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}
