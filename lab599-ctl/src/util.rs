use std::sync::{Arc, Mutex};

/// Redirects fd 2 to a pipe during `f`, then captures any output and appends
/// each non-empty line to `errors`. ALSA/PipeWire write diagnostic noise
/// directly to fd 2, bypassing Rust's stderr — this routes it to the Logs page.
pub fn capture_stderr<F, R>(f: F, errors: &Arc<Mutex<Vec<String>>>) -> R
where
    F: FnOnce() -> R,
{
    extern "C" {
        fn dup(fd: i32) -> i32;
        fn dup2(oldfd: i32, newfd: i32) -> i32;
        fn close(fd: i32) -> i32;
        fn pipe(pipefd: *mut i32) -> i32;
    }

    unsafe {
        let mut fds = [0i32; 2]; // [read_end, write_end]
        if pipe(fds.as_mut_ptr()) != 0 {
            return f();
        }
        let [read_fd, write_fd] = fds;

        let saved = dup(2);
        dup2(write_fd, 2); // stderr now goes into the pipe
        close(write_fd); // only fd 2 holds the write end

        let result = f();

        dup2(saved, 2); // restore real stderr; write end now has 0 refs → closed
        close(saved);

        // Read everything that ALSA wrote; write end is closed so read reaches EOF.
        use std::io::Read;
        use std::os::unix::io::FromRawFd;
        let mut reader = std::fs::File::from_raw_fd(read_fd);
        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf);
        if !buf.trim().is_empty() {
            if let Ok(mut q) = errors.lock() {
                for line in buf.lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        q.push(format!("ALSA: {line}"));
                    }
                }
            }
        }

        result
    }
}

/// Redirects fd 2 to /dev/null for the duration of `f`.
/// Use for device enumeration where ALSA probe noise is expected and not useful.
pub fn suppress_stderr<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    extern "C" {
        fn dup(fd: i32) -> i32;
        fn dup2(oldfd: i32, newfd: i32) -> i32;
        fn close(fd: i32) -> i32;
        fn open(path: *const i8, flags: i32, ...) -> i32;
    }
    const O_WRONLY: i32 = 1;

    unsafe {
        let saved = dup(2);
        let devnull = open(c"/dev/null".as_ptr(), O_WRONLY);
        dup2(devnull, 2);
        close(devnull);
        let result = f();
        dup2(saved, 2);
        close(saved);
        result
    }
}
