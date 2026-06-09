use std::os::raw::c_void;

#[repr(C)]
pub struct FreeList {
    pub free: *mut c_void,
    pub work: *mut c_void,
}
