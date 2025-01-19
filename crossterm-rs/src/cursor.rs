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

#[no_mangle]
pub unsafe extern "C" fn crossterm_save_cursor_position(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::SavePosition);
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_restore_cursor_position(
    stream: *mut crossterm_stream,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::RestorePosition);
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_up(
    stream: *mut crossterm_stream,
    nrows: u16,
) -> libc::c_int {
    if nrows == 0 {
        return 0;
    }
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::MoveUp(nrows));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_down(
    stream: *mut crossterm_stream,
    nrows: u16,
) -> libc::c_int {
    if nrows == 0 {
        return 0;
    }
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::MoveDown(nrows));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_left(
    stream: *mut crossterm_stream,
    ncols: u16,
) -> libc::c_int {
    if ncols == 0 {
        return 0;
    }
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::MoveLeft(ncols));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_right(
    stream: *mut crossterm_stream,
    ncols: u16,
) -> libc::c_int {
    if ncols == 0 {
        return 0;
    }
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::MoveRight(ncols));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_to(
    stream: *mut crossterm_stream,
    row: u16,
    col: u16,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::MoveTo(row, col));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_to_row(
    stream: *mut crossterm_stream,
    row: u16,
) -> libc::c_int {
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::MoveToRow(row));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_to_next_line(
    stream: *mut crossterm_stream,
    nlines: u16,
) -> libc::c_int {
    if nlines == 0 {
        return 0;
    }
    let ret = crossterm::queue!((&mut *stream), crossterm::cursor::MoveToNextLine(nlines));
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_move_cursor_to_previous_line(
    stream: *mut crossterm_stream,
    nlines: u16,
) -> libc::c_int {
    if nlines == 0 {
        return 0;
    }
    let ret = crossterm::queue!(
        (&mut *stream),
        crossterm::cursor::MoveToPreviousLine(nlines)
    );
    if let Err(err) = ret {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}
