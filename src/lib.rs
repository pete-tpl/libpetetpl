mod render_result;

use std::sync::{Arc, Mutex};
use std::ffi::CStr;

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
    return engines.len() as i32 - 1
}

#[no_mangle]
pub unsafe extern "C" fn render(handle: libc::c_uint, template: *const libc::c_char) -> *const RenderResult {
    let engines_arc = match &ENGINES {
        Some(e) => e,
        None => { return Box::into_raw(Box::new(RenderResult::new("PETE is not initialized.", -100))); },
    };
    let engines = engines_arc.lock().unwrap();
    let engine = &engines[handle as usize];

    let tpl = CStr::from_ptr(template);
    let tpl = match tpl.to_str() {
        Ok(s) => String::from(s),
        Err(e) => panic!(e),
    };
    
    match engine.render(tpl.clone(), ParameterStore::new()) {
        Ok(rendered_template) => Box::into_raw(Box::new(RenderResult::new(rendered_template.as_str(), 0))),
        Err(error) => Box::into_raw(Box::new(RenderResult::new(error.message.as_str(), -1))),
    }
}


