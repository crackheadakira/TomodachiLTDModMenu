use std::ffi::c_void;

#[repr(C)]
pub struct PtrArray<T> {
    pub count: i32,
    pub capacity: i32,
    pub array: *mut *mut T,
}

const _: () = assert!(core::mem::size_of::<PtrArray<c_void>>() == 0x10);
