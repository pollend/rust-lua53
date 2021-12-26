use libc::{c_void, c_int};
use std::{mem, ptr};
use ffi;
use ffi::lua::{size_t};
use ::LuaState;
use ::Index;

unsafe extern fn alloc_func(_: *mut c_void, ptr: *mut c_void, old_size: size_t, new_size: size_t) -> *mut c_void {
    // In GCC and MSVC, malloc uses an alignment calculated roughly by:
    //   max(2 * sizeof(size_t), alignof(long double))
    // The stable high-level API used here does not expose alignment directly, so
    // we get as close as possible by using usize to determine alignment. Lua
    // seems unlikely to require 16-byte alignment for any of its purposes.
  
    #[inline]
    fn divide_size(size: size_t) -> usize {
      ((1 + (size - 1)) as usize) / mem::size_of::<usize>()
    }
  
    let ptr = ptr as *mut usize;
    if new_size == 0 {
      // if new_size is 0, act like free()
      if !ptr.is_null() {
        // Lua promises to provide the correct old_size
        drop(Vec::<usize>::from_raw_parts(ptr, 0, divide_size(old_size)));
      }
      ptr::null_mut()
    } else {
      // otherwise, act like realloc()
      let mut vec;
      if ptr.is_null() {
        // old_size is a type indicator, not used here
        vec = Vec::<usize>::with_capacity(divide_size(new_size));
      } else {
        // Lua promises to provide the correct old_size
        if new_size > old_size {
          // resulting capacity should be new_size
          vec = Vec::<usize>::from_raw_parts(ptr, 0, divide_size(old_size));
          vec.reserve_exact(divide_size(new_size));
        } else {
          // Lua assumes this will never fail
          vec = Vec::<usize>::from_raw_parts(ptr, divide_size(new_size), divide_size(old_size));
          vec.shrink_to_fit();
        }
      }
      let res = vec.as_mut_ptr();
      mem::forget(vec); // don't deallocate
      res as *mut c_void
    }
  }
  

impl LuaState {
    pub fn new() -> LuaState {
        unsafe {
            let state = ffi::lua::lua_newstate(Some(alloc_func), ptr::null_mut());
            LuaState { ptr: state, owned: true }
        }
    }

    pub fn new_thread(state: &LuaState) -> LuaState {
        unsafe{
            let new_state = ffi::lua::lua_newthread(state.ptr);
            LuaState { ptr: new_state, owned: true }
        }
    }

    pub fn from_ptr(state: *mut ffi::lua::lua_State) {
        LuaState {ptr: state, owned: false};
    }

    pub fn reset_thread(&mut self) -> c_int {
        unsafe { return ffi::lua::lua_resetthread(self.ptr); }
    }
    
    pub fn get_state(&mut self) -> *mut ffi::lua::lua_State {
        return self.ptr;
    }
    
    //===========================================================================
    // Basic stack manipulation
    //===========================================================================
    pub fn absindex(&mut self, idx: Index) {
        unsafe { ffi::lua::lua_absindex(self.ptr, idx); }
    }

    pub fn gettop(&mut self) -> Index {
        unsafe { return ffi::lua::lua_gettop(self.ptr); }
    }

    pub fn settop(&mut self, idx: Index) {
        unsafe { return ffi::lua::lua_settop(self.ptr, idx); }
    }

    pub fn pushvalue(&mut self, idx: Index) {
        unsafe { return ffi::lua::lua_pushvalue(self.ptr, idx); }
    }

    pub fn rotate(&mut self, idx: Index, n: c_int) {
        unsafe { return ffi::lua::lua_rotate(self.ptr, idx, n); }
    }

    pub fn copy(&mut self, from_index: Index, to_index: Index) {
        unsafe { return ffi::lua::lua_copy(self.ptr, from_index, to_index); }
    }

    pub fn checkstack(&mut self, n: c_int) {
        unsafe { return ffi::lua::lua_settop(self.ptr, n); }
    }

    pub fn xmove(&mut self, to_state: LuaState, n: c_int) {
        unsafe { return ffi::lua::lua_xmove(self.ptr, to_state.ptr, n)}
    }

    //===========================================================================
    // access functions (stack -> C)
    //===========================================================================
    pub fn isnumber(&mut self) {
        
    } 
    pub fn isstring(&mut self) {
        
    } 
    pub fn iscfunction(&mut self) {
        
    } 
    pub fn isinteger(&mut self) {
        
    } 
    pub fn isuserdata(&mut self) {
        
    } 
    // pub fn type(&mut self) {
        
    // } 
    pub fn typename(&mut self) {
        
    } 
}

impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe {
            if self.owned {
                ffi::lua::lua_close(self.ptr);
            }
        }
    }
}