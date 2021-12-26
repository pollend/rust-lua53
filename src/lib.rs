// pub extern crate libc;
// extern crate bitflags;
pub extern crate libc;
// #[macro_use]
extern crate bitflags;

pub mod ffi {
    pub mod lua {
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
}

pub type Index = libc::c_int;

pub struct LuaState {
    ptr: *mut ffi::lua::lua_State,
    owned: bool
}


// pub mod ffi;
mod lua_state;

