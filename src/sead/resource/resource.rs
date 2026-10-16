use std::ffi::c_void;

#[repr(C)]
pub struct Resource {
    pub vtable: *const c_void,
}

#[repr(C, packed)]
pub struct DirectResource {
    pub base: Resource,
    pub file_data: *const c_void,
    pub raw_size: u32,
    pub alignment: i32,
    pub setting_flag: u32,
}

const _: () = assert!(core::mem::size_of::<Resource>() == 0x8);
const _: () = assert!(core::mem::size_of::<DirectResource>() == 0x1c);
