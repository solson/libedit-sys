extern crate libc;

use libc::{c_char, c_int};
use libc::types::common::c95::FILE;
use libc::types::os::arch::c95::wchar_t;

pub enum EditLine {}

#[repr(C)]
pub struct LineInfo {
    buffer: *const c_char,
    cursor: *const c_char,
    lastchar: *const c_char,
}

#[repr(C)]
pub struct LineInfoW {
    buffer: *const wchar_t,
    cursor: *const wchar_t,
    lastchar: *const wchar_t,
}

pub enum History {}
pub enum HistoryW {}

#[repr(C)]
pub struct HistEvent {
    num: c_int,
    str_: *const c_char,
}

#[repr(C)]
pub struct HistEventW {
    num: c_int,
    str_: *const wchar_t,
}

pub enum Tokenizer {}
pub enum TokenizerW {}

#[link(name = "edit")]
extern {
    // Initialization, deinitialization, and miscellaneous functions.
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
    pub fn el_source(e: *mut EditLine, file: *const c_char) -> c_int;
    pub fn el_resize(e: *mut EditLine);
    pub fn el_cursor(e: *mut EditLine, count: c_int) -> c_int;

    // Line editing functions.
    pub fn el_gets(e: *mut EditLine, count: *mut c_int) -> *const c_char;
    pub fn el_getc(e: *mut EditLine, ch: *mut c_char) -> c_int;
    pub fn el_push(e: *mut EditLine, str_: *const c_char);
    pub fn el_parse(e: *mut EditLine, argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn el_set(e: *mut EditLine, op: c_int, ...) -> c_int;
    pub fn el_get(e: *mut EditLine, op: c_int, ...) -> c_int;
    pub fn el_line(e: *mut EditLine) -> *const LineInfo;
    pub fn el_insertstr(e: *mut EditLine, str_: *const c_char) -> c_int;
    pub fn el_deletestr(e: *mut EditLine, count: c_int);

    // History functions.
    pub fn history_init() -> *mut History;
    pub fn history_end(h: *mut History);
    pub fn history(h: *mut History, ev: *mut HistEvent, op: c_int, ...) -> c_int;

    // Tokenization functions.
    pub fn tok_init(ifs: *const c_char) -> *mut Tokenizer;
    pub fn tok_end(t: *mut Tokenizer);
    pub fn tok_reset(t: *mut Tokenizer);
    pub fn tok_line(t: *mut Tokenizer,
                    li: *const LineInfo,
                    argc: *mut c_int,
                    argv: *mut *const *const c_char,
                    cursorc: *mut c_int,
                    cursoro: *mut c_int) -> c_int;
    pub fn tok_str(t: *mut Tokenizer,
                   li: *const LineInfo,
                   argc: *mut c_int,
                   argv: *mut *const *const c_char) -> c_int;

    // Wide character line editing functions.
    pub fn el_wgets(e: *mut EditLine, count: *mut c_int) -> *const wchar_t;
    pub fn el_wgetc(e: *mut EditLine, ch: *mut wchar_t) -> c_int;
    pub fn el_wpush(e: *mut EditLine, str_: *const wchar_t);
    pub fn el_wparse(e: *mut EditLine, argc: c_int, argv: *const *const wchar_t) -> c_int;
    pub fn el_wset(e: *mut EditLine, op: c_int, ...) -> c_int;
    pub fn el_wget(e: *mut EditLine, op: c_int, ...) -> c_int;
    pub fn el_wline(e: *mut EditLine) -> *const LineInfoW;
    pub fn el_winsertstr(e: *mut EditLine, str_: *const wchar_t) -> c_int;
    pub fn el_wdeletestr(e: *mut EditLine, count: c_int);

    // Wide character history functions.
    pub fn history_winit() -> *mut HistoryW;
    pub fn history_wend(h: *mut HistoryW);
    pub fn history_w(h: *mut HistoryW, ev: *mut HistEventW, op: c_int, ...) -> c_int;

    // Wide character tokenization functions.
    pub fn tok_winit(ifs: *const wchar_t) -> *mut TokenizerW;
    pub fn tok_wend(t: *mut TokenizerW);
    pub fn tok_wreset(t: *mut TokenizerW);
    pub fn tok_wline(t: *mut TokenizerW,
                     li: *const LineInfo,
                     argc: *mut c_int,
                     argv: *mut *const *const wchar_t,
                     cursorc: *mut c_int,
                     cursoro: *mut c_int) -> c_int;
    pub fn tok_wstr(t: *mut TokenizerW,
                    li: *const LineInfo,
                    argc: *mut c_int,
                    argv: *mut *const *const wchar_t) -> c_int;
}
