use std::ffi::c_void;

#[repr(C)]
pub struct Buffer<T> {
    pub count: i32,
    pub capacity: i32,
    pub array: *mut T,
}

const _: () = assert!(core::mem::size_of::<Buffer<c_void>>() == 0x10);
