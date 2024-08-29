use std::ffi::c_char;

extern "C" {
    pub fn COM_DieNoFormat(format: *const c_char);
    pub fn COM_SetComErrno(val: i32);
}