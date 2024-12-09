use crate::error::crossterm_error;
use crate::stream::crossterm_stream;

#[no_mangle]
pub unsafe extern "C" fn crossterm_enter_alternate_screen(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::terminal::EnterAlternateScreen);
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_leave_alternate_screen(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::terminal::LeaveAlternateScreen);
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_scroll_up(
    stream: *mut crossterm_stream,
    nlines: u16,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::terminal::ScrollUp(nlines));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_scroll_down(
    stream: *mut crossterm_stream,
    nlines: u16,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::terminal::ScrollDown(nlines));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_clear_all(stream: *mut crossterm_stream) -> libc::c_int {
    use crossterm::terminal::Clear;
    use crossterm::terminal::ClearType;

    let ret = crossterm::queue!((&mut *stream), Clear(ClearType::All));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_clear_purge(stream: *mut crossterm_stream) -> libc::c_int {
    use crossterm::terminal::Clear;
    use crossterm::terminal::ClearType;

    let ret = crossterm::queue!((&mut *stream), Clear(ClearType::Purge));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_clear_from_cursor_up(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    use crossterm::terminal::Clear;
    use crossterm::terminal::ClearType;

    let ret = crossterm::queue!((&mut *stream), Clear(ClearType::FromCursorUp));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_clear_from_cursor_down(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    use crossterm::terminal::Clear;
    use crossterm::terminal::ClearType;

    let ret = crossterm::queue!((&mut *stream), Clear(ClearType::FromCursorDown));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_clear_current_line(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    use crossterm::terminal::Clear;
    use crossterm::terminal::ClearType;

    let ret = crossterm::queue!((&mut *stream), Clear(ClearType::CurrentLine));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_clear_until_new_line(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    use crossterm::terminal::Clear;
    use crossterm::terminal::ClearType;

    let ret = crossterm::queue!((&mut *stream), Clear(ClearType::UntilNewLine));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}
