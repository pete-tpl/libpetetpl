mod render_result;

use std::sync::{Arc, Mutex};
use std::ffi::{ CStr, CString};

use libc;

use pete_core::engine::Engine;
use pete_core::parameter::ParameterStore;

use crate::render_result::RenderResult;

static mut ENGINES: Option<Arc<Mutex<Vec<Engine>>>> = None;

#[no_mangle]
pub unsafe extern "C" fn init() {
    ENGINES = Some(Arc::new(Mutex::new(Vec::new())));
}

#[no_mangle]
pub unsafe extern "C" fn create_new() -> libc::c_int {
    let engines_arc = match &ENGINES {
        Some(e) => e,
        None => return -1,
    };
    let mut engines = engines_arc.lock().unwrap();
    engines.push(Engine::new());
    return engines.len() as i32
}

#[no_mangle]
pub unsafe extern "C" fn render(handle: libc::c_uint, template: *const libc::c_char) -> *const RenderResult {
    let mut result = RenderResult::new();
    let engines_arc = match &ENGINES {
        Some(e) => e,
        None => { return std::ptr::null(); },
    };
    let engines = engines_arc.lock().unwrap();
    let engine = &engines[handle as usize];

    let tpl = CStr::from_ptr(template);
    let tpl = match tpl.to_str() {
        Ok(s) => String::from(s),
        Err(e) => panic!(e),
    };
    
    match engine.render(tpl.clone(), ParameterStore::new()) {
        Ok(s) => {
            let c_template = CString::from_vec_unchecked(Vec::from(s.as_bytes()));
            result.parsed_template = c_template.into_raw();
            result.response_code = 2;
            result.parsed_template_len = 222;
        },
        Err(e) => panic!(e),
    };

    Box::into_raw(Box::new(result))
}


