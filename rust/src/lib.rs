use std::{ffi::CString, panic};
use c_func::*;

mod c_func;

#[allow(non_snake_case)]
#[no_mangle]
pub fn RS_Main() {
    panic::set_hook(Box::new(|x| {
        unsafe {
            let string = CString::new(x.to_string()).unwrap();

            COM_SetComErrno(-1);
            COM_DieNoFormat(string.as_ptr());
        }
    }));

    panic!("panic");
}