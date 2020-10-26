use std::slice;

use libc;

use pete_core::parameter::{Parameter, ParameterStore};

use crate::ffi;

const TYPE_INT: u16 = 1;
const TYPE_FLOAT: u16 = 2;
const TYPE_STRING: u16 = 5;

#[repr(C)]
pub struct Param {
    pub name: *const libc::c_char,
    pub param_type: u16,
    pub value_float: f64,
    pub value_int: i64,
    pub value_string: *const libc::c_char,
}

impl Param {
    pub unsafe fn fetch(paramsc: *const libc::c_uint, paramsv: *const Param) -> Result<ParameterStore, String> {
        let mut result = ParameterStore::new();
        let params: &[Param] = slice::from_raw_parts(paramsv, paramsc as usize);
        
        for param in params {
            let name = match ffi::ccharptr_to_string(param.name) {
                Ok(s) => s,
                Err(e) => { return Err(format!("Failed to parse parameters: {}", e)) },
            };
            
            
            let fetched_param = match param.param_type {
                TYPE_INT => Some(Parameter::new_from_int(param.value_int as i128)),
                TYPE_FLOAT => Some(Parameter::new_from_float(param.value_float)),
                TYPE_STRING => {
                    let val = match ffi::ccharptr_to_string(param.value_string) {
                        Ok(s) => s,
                        Err(e) => { return Err(format!("Failed to parse parameter {}: {}", name, e)); },
                    };
                    Some(Parameter::new_from_string(val))
                },
                _ => None
            };
            
            match fetched_param {
                Some(p) => { result.insert(name, p); },
                _ => {},
            }
            
        }

        Ok(result)
    }
}