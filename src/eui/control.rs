use std::ffi::{c_char, c_void};

use crate::{
    eui::{screen_manager::BaseScreen, LayoutEx},
    sead::{
        container::{ListNode, PtrArray},
        thread::CriticalSection,
    },
};

#[derive(Debug)]
#[repr(C)]
pub struct ControlBase {
    pub vtable: *const ControlBaseVtable,
    pub list_node: ListNode,
    pub name: *mut c_char,
    pub layout: *mut LayoutEx,
}

impl ControlBase {
    pub fn get_layout<'a>(&self) -> Option<&'a mut LayoutEx> {
        if !self.layout.is_null() {
            unsafe { Some(&mut *self.layout) }
        } else {
            None
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ButtonBase {
    pub base: ControlBase,
    pub button_list_node: ListNode,
    pub state: u8,
    pub pad_39: u8,
    pub process_state: u8,
    pub unk_3b: u8,
    pub button_index: u8,
    pub unk_3d: u8,
    pub unk_3e: u8,
    pub unk_3f: u8,
    pub flags: u32,
    pub slide_index: i32,
}

impl ButtonBase {
    pub fn is_clicked(&self) -> bool {
        if (self.state & 0xfe) != 4 {
            let mut remaining = self.unk_3e as usize;

            if remaining == 0 {
                return false;
            }

            if self.process_state != 2 {
                let mut ptr = &self.unk_3b as *const u8;
                loop {
                    remaining -= 1;
                    if remaining == 0 {
                        return false;
                    }
                    unsafe {
                        if *ptr == 2 {
                            return remaining != 0;
                        }
                        ptr = ptr.add(1);
                    }
                }
            }
        }
        true
    }
}

#[repr(C)]
pub struct ControlBaseVtable {
    pub get_class_name: extern "C" fn() -> *const c_char,
    pub get_runtime_type_info: extern "C" fn() -> *const c_void,
    pub dtor_1: extern "C" fn(),
    pub dtor_2: extern "C" fn(u64),
    pub update: extern "C" fn(u64),

    pub on: extern "C" fn(u64),
    pub off: extern "C" fn(u64),
    pub down: extern "C" fn(u64),
    pub cancel: extern "C" fn(u64),

    pub force_off: extern "C" fn(u64),
    pub force_on: extern "C" fn(u64),
    pub force_down: extern "C" fn(u64),

    pub set_active: extern "C" fn(u64, u32),

    pub process_on: extern "C" fn(u64) -> bool,
    pub process_off: extern "C" fn(u64) -> bool,
    pub process_down: extern "C" fn(u64) -> bool,
    pub process_cancel: extern "C" fn(u64) -> bool,

    pub update_on: extern "C" fn(u64) -> bool,
    pub update_off: extern "C" fn(u64) -> bool,
    pub update_down: extern "C" fn(u64) -> bool,
    pub update_cancel: extern "C" fn(u64) -> bool,

    pub start_on: extern "C" fn(u64) -> bool,
    pub start_off: extern "C" fn(u64) -> bool,
    pub start_down: extern "C" fn(u64) -> bool,
    pub start_cancel: extern "C" fn(u64) -> bool,

    pub finish_on: extern "C" fn(u64),
    pub finish_off: extern "C" fn(u64),
    pub finish_down: extern "C" fn(u64),
    pub finish_cancel: extern "C" fn(u64),

    pub change_state: extern "C" fn(u64, u32),
    pub force_change_state: extern "C" fn(u64, u32),

    pub build: extern "C" fn(u64, u64, u64),
    pub hit_test: extern "C" fn(u64, *const f32),

    pub start_drag: extern "C" fn(u64),
    pub update_drag: extern "C" fn(u64),
    pub finish_drag: extern "C" fn(u64),

    pub unk_120: extern "C" fn(u64) -> bool,

    pub play_disable_anim: extern "C" fn(u64, bool),
    pub set_disable_anim_direct: extern "C" fn(u64, bool),

    pub activate_by_box_cursor_node: extern "C" fn(u64),
    pub inactivate_by_box_cursor_node: extern "C" fn(u64),

    pub build_state_anim: extern "C" fn(u64, u64, u64),
}

#[repr(C)]
pub struct ButtonHitCallback {
    pub vtable: *const c_void,
    pub next_button_hit_callback: *mut ButtonHitCallback,
    pub button_hit_handler: *mut ButtonHitHandler,
    pub screen_index: i32,
    pub unk_1c: u8,
    pub processing: u8,
    pub is_enabled: bool,
    pub unk_1f: u8,
}

#[repr(C)]
pub struct ButtonHitHandler {
    pub processing_callbacks: PtrArray<ButtonHitCallback>,
    pub canceled_callbacks: PtrArray<ButtonHitCallback>,
    pub reserved_callbacks: PtrArray<ButtonHitCallback>,
    pub root_callback: *mut ButtonHitCallback,
    pub screen: *mut BaseScreen,
    pub cs: *mut CriticalSection,
    pub unk_48: i32,
    pub pad_4c: u32,
}

#[repr(C)]
pub struct ButtonGroup {
    pub vtable: *const c_void,
    pub active_list: ListNode,
    pub update_list: ListNode,
    pub current_button: *mut ButtonBase,
    pub unk_30: *mut ButtonBase,
    pub unk_38: *mut ButtonBase,
    pub unk_40: i32,
    pub node_id: u32,
}

impl ButtonGroup {
    pub unsafe fn find_button_by_id(&self, id: i32) -> Option<&mut ButtonBase> {
        let sentinel = &self.active_list as *const ListNode as *mut ListNode;
        let mut node = (*sentinel).next;

        while node != sentinel {
            let button_index = *((node as *const u8).add(0x3c) as *const i32);
            if button_index == id {
                return Some(&mut *((node as *mut u8).sub(0x8) as *mut ButtonBase));
            }
            node = (*node).next;
        }
        None
    }
}

const _: () = assert!(core::mem::size_of::<ButtonGroup>() == 0x48);
const _: () = assert!(core::mem::size_of::<ControlBase>() == 0x28);
const _: () = assert!(core::mem::size_of::<ButtonBase>() == 0x48);
const _: () = assert!(core::mem::size_of::<ControlBaseVtable>() == 0x150);
const _: () = assert!(core::mem::size_of::<ButtonHitCallback>() == 0x20);
const _: () = assert!(core::mem::size_of::<ButtonHitHandler>() == 0x50);
