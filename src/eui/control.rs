use std::ffi::{c_char, c_void};

use crate::{eui::LayoutEx, sead::container::ListNode};

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
    pub button_index: u32,
    pub flags: u32,
    pub slide_index: i32,
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

const _: () = assert!(core::mem::size_of::<ControlBase>() == 0x28);
const _: () = assert!(core::mem::size_of::<ButtonBase>() == 0x48);
const _: () = assert!(core::mem::size_of::<ControlBaseVtable>() == 0x150);
