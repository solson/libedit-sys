extern crate libc;

use libc::{c_char, c_int};
use libc::types::common::c95::FILE;

pub enum EditLine {}

#[link(name = "edit")]
extern {
    pub fn el_init(prog: *const c_char,
                   fin: *mut FILE,
                   fout: *mut FILE,
                   ferr: *mut FILE) -> *mut EditLine;

    pub fn el_init_fd(prog: *const c_char,
                      fin: *mut FILE,
                      fout: *mut FILE,
                      ferr: *mut FILE,
                      fdin: c_int,
                      fdout: c_int,
                      fderr: c_int) -> *mut EditLine;

    pub fn el_end(e: *mut EditLine);
    pub fn el_reset(e: *mut EditLine);
    pub fn el_gets(e: *mut EditLine, count: *mut c_int) -> *const c_char;

    // TODO: ...

    pub fn el_set(e: *mut EditLine, op: c_int, ...) -> c_int;
}
