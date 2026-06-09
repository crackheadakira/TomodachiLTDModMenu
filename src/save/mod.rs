pub mod player;

use std::ffi::c_void;

use skyline::nn;

use crate::sead::{prim::WFixedSafeString64, thread::CriticalSection};

pub unsafe fn get_global_sync_counter(save_data_manager: *mut c_void) -> i32 {
    if save_data_manager.is_null() {
        return 0;
    }

    *(save_data_manager.add(0x950) as *const i32)
}

#[repr(C)]
pub struct SaveFlag<T> {
    pub vtable: *const c_void,
    pub current_value: T,
    pub previous_value: T,
    pub last_updated: i32,
    pub is_dirty: bool,
    pub cs: CriticalSection,
}

impl<T> SaveFlag<T> {
    pub unsafe fn get(&self, save_data_manager: *mut c_void) -> *const T {
        let mutex_ptr = &self.cs.critical_section_inner as *const _ as *mut _;
        nn::os::LockMutex(mutex_ptr);

        if !save_data_manager.is_null() {
            let is_loaded = *(save_data_manager.add(0x94c) as *const i32);

            if is_loaded == 1 {
                let global_version = get_global_sync_counter(save_data_manager);
                if self.last_updated != global_version {
                    let mutable_self = self as *const Self as *mut Self;
                    (*mutable_self).last_updated = global_version;

                    core::ptr::copy_nonoverlapping(
                        &self.current_value as *const T,
                        &mut (*mutable_self).previous_value as *mut T,
                        1,
                    );
                }
            }
        }

        nn::os::UnlockMutex(mutex_ptr);
        &self.current_value as *const T
    }

    pub unsafe fn set(&self, new_value: *const T) {
        let mutex_ptr = &self.cs.critical_section_inner as *const _ as *mut _;
        nn::os::LockMutex(mutex_ptr);

        let mutable_self = self as *const Self as *mut Self;

        core::ptr::copy_nonoverlapping(
            &self.current_value as *const T,
            &mut (*mutable_self).previous_value as *mut T,
            1,
        );

        core::ptr::copy_nonoverlapping(new_value, &mut (*mutable_self).current_value as *mut T, 1);

        (*mutable_self).is_dirty = true;

        nn::os::UnlockMutex(mutex_ptr);
    }
}

const _: () = assert!(core::mem::size_of::<SaveFlag<bool>>() == 0x58);
const _: () = assert!(core::mem::size_of::<SaveFlag<i32>>() == 0x58);
const _: () = assert!(core::mem::size_of::<SaveFlag<u64>>() == 0x60);
const _: () = assert!(core::mem::size_of::<SaveFlag<WFixedSafeString64>>() == 0x170);
