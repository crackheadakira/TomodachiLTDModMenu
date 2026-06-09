pub mod player;

use std::ffi::c_void;

use crate::sead::{prim::WFixedSafeString64, thread::CriticalSection};

#[repr(C)]
pub struct SaveFlag<T> {
    pub vtable: *const c_void,
    pub current_value: T,
    pub previous_value: T,
    pub last_updated: i32,
    pub is_dirty: bool,
    pub cs: CriticalSection,
}

const _: () = assert!(core::mem::size_of::<SaveFlag<bool>>() == 0x58);
const _: () = assert!(core::mem::size_of::<SaveFlag<i32>>() == 0x58);
const _: () = assert!(core::mem::size_of::<SaveFlag<u64>>() == 0x60);
const _: () = assert!(core::mem::size_of::<SaveFlag<WFixedSafeString64>>() == 0x170);
