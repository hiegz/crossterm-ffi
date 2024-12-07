use libc;

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
pub unsafe extern "C" fn crossterm_stream_free(stream: *mut crossterm_stream) {
    libc::free(stream as *mut libc::c_void);
}
