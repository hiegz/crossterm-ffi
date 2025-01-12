use libc;

use crate::error::crossterm_error;
use crate::style::crossterm_style;
use crate::style::from_ffi_style_to_rust_style;
use crate::uint21_t::crossterm_uint21_t;

#[repr(C)]
pub struct crossterm_stream {
    stream: *mut libc::FILE,
}

impl std::io::Write for crossterm_stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            let rt = libc::fwrite(
                buf.as_ptr() as *const libc::c_void,
                size_of::<u8>(),
                buf.len(),
                self.stream,
            );

            if rt < buf.len() {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(rt)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unsafe {
            let rt = libc::fflush(self.stream);
            if rt == libc::EOF {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_stream_new(file: *mut libc::FILE) -> *mut crossterm_stream {
    let stream = libc::malloc(size_of::<crossterm_stream>()) as *mut crossterm_stream;
    if stream.is_null() {
        return std::ptr::null_mut();
    }
    (*stream).stream = file;
    return stream;
}

#[no_mangle]
pub unsafe extern "C" fn crossterm_stream_write(
    stream: *mut crossterm_stream,
    buf: *const crossterm_uint21_t,
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

    let mut content = String::default();
    for i in 0..buflen {
        let ret = std::char::from_u32(*buf.offset(i as isize));
        if let None = ret {
            return -(crossterm_error::CROSSTERM_EINVAL as i32);
        }
        content.push(ret.unwrap());
    }

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

#[no_mangle]
pub unsafe extern "C" fn crossterm_stream_free(stream: *mut crossterm_stream) {
    libc::free(stream as *mut libc::c_void);
}
