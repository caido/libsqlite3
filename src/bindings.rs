use std::fmt::Debug;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3([u8; 0]);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_api_routines([u8; 0]);

extern "C" {
    pub fn sqlite3_sha_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    );
}
