use std::ffi::CStr;

pub unsafe fn ccharptr_to_string(ccharptr: *const libc::c_char) -> Result<String, std::str::Utf8Error> {
    let cstr = CStr::from_ptr(ccharptr);
    
    match cstr.to_str() {
        Ok(s) => Ok(String::from(s)),
        Err(e) => Err(e),
    }
}