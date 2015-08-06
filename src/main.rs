extern crate libc;
extern crate libedit_sys;

use libedit_sys::*;
use libc::consts::os::posix88::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};
use std::slice;

macro_rules! c_str {
    ($str_literal:expr) => (concat!($str_literal, '\0').as_ptr() as *const libc::c_char)
}

fn main() {
    unsafe {
        let stdin  = libc::fdopen(STDIN_FILENO, c_str!("r"));
        let stdout = libc::fdopen(STDOUT_FILENO, c_str!("w"));
        let stderr = libc::fdopen(STDERR_FILENO, c_str!("w"));
        let e = el_init(c_str!("libedit-sys"), stdin, stdout, stderr);

        fn prompt(e: *mut EditLine) -> *const libc::c_char {
            c_str!("libedit-sys> ")
        }

        el_set(e, 0, prompt);

        let mut len = 0;
        let line_ptr = el_gets(e, &mut len);
        let line_bytes = slice::from_raw_parts(line_ptr as *const u8, len as usize);
        let line = String::from_utf8_lossy(line_bytes);
        println!("{:?}", line);

        el_end(e);
    }
}
