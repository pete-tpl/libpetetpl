use std::ffi::{ CString};

#[repr(C)]
pub struct RenderResult {
    pub output: *const libc::c_char,
    pub error_code: libc::c_int,
}

impl RenderResult {
    pub unsafe fn new(output: &str, error_code: i16) -> RenderResult {
        let c_string = CString::from_vec_unchecked(Vec::from(output.as_bytes()));
        RenderResult {
            output: c_string.into_raw(),
            error_code: error_code as libc::c_int,
        }
    }
}