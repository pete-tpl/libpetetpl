#[repr(C)]
pub struct RenderResult {
    pub parsed_template: *const libc::c_char,
    pub parsed_template_len: libc::c_uint,
    pub response_code: libc::c_uint
}

impl RenderResult {
    pub fn new() -> RenderResult {
        RenderResult {
            parsed_template: std::ptr::null(),
            parsed_template_len: 123,
            response_code: 1
        }
    }
}