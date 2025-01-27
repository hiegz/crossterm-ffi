use libc;

use crate::error::crossterm_error;
use crate::style::crossterm_style;
use crate::style::from_ffi_style_to_rust_style;

#[repr(C)]
pub struct crossterm_stream {
    context: *mut libc::c_void,
    write_fn: fn(buf: *const u8, buflen: libc::size_t, context: *mut libc::c_void) -> libc::c_long,
    flush_fn: fn(context: *mut libc::c_void) -> libc::c_int,
}

impl std::io::Write for crossterm_stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let ret = (self.write_fn)(buf.as_ptr(), buf.len(), self.context);

        if ret >= 0 {
            Ok(ret as usize)
        } else {
            unsafe {
                assert!(-ret <= i32::max_value() as i64);
                *libc::__errno_location() = -ret as i32;
            }
            Err(std::io::Error::from(std::io::ErrorKind::Other))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let ret = (self.flush_fn)(self.context);

        if ret == 0 {
            Ok(())
        } else {
            unsafe {
                *libc::__errno_location() = -ret as i32;
            }
            Err(std::io::Error::from(std::io::ErrorKind::Other))
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_stream_write(
    stream: *mut crossterm_stream,
    buf: *const u8,
    buflen: libc::size_t,
    style: *const crossterm_style,
) -> libc::c_int {
    use crossterm::style::ContentStyle;
    use crossterm::style::PrintStyledContent;

    let style = if !style.is_null() {
        from_ffi_style_to_rust_style(&*style)
    } else {
        ContentStyle::default()
    };

    let ret = std::str::from_utf8(std::slice::from_raw_parts(buf, buflen));
    if let Err(_) = ret {
        return -(crossterm_error::CROSSTERM_EINVAL as i32);
    }
    let content = ret.unwrap();

    let ret = crossterm::queue!(&mut *stream, PrintStyledContent(style.apply(content)));
    if let Err(err) = ret {
        return -(crossterm_error::from(err) as i32);
    };

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_stream_flush(stream: *mut crossterm_stream) -> libc::c_int {
    let stream = &mut *stream as &mut dyn std::io::Write;
    let result = stream.flush();
    if let Err(err) = result {
        -(crossterm_error::from(err) as i32)
    } else {
        0
    }
}
