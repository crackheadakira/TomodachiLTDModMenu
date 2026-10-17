use std::ffi::c_void;

#[repr(C)]
pub struct ResourceArchive {
    pub vtable: *const c_void,
    pub state: u32,
    pub asset_info: *const c_void,
    pub unk_18: *const ResourceArchive,
    pub resource_binder: *const c_void,
}

const _: () = assert!(core::mem::size_of::<ResourceArchive>() == 0x28);
