use std::os::raw::c_void;

use native_vtable::vtable;
use skyline::libc::c_char;

use crate::{
    eui::{
        box_cursor_node::BoxCursorNode,
        screen_manager::{ExtendedCloseState, ScreenMgr},
        ButtonGroup, DrawState, LayoutEx, ScreenState,
    },
    sead::{
        container::{ListNode, OffsetList},
        heap::{Heap, IDisposer, VftIDisposer},
    },
};

#[vtable(path = eui::screen::VftScreen)]
pub unsafe trait VftScreen: VftIDisposer {
    unsafe fn check_derived_runtime_type_info(&self, type_info: *const c_void) -> *const Self;
    unsafe fn get_runtime_type_info(&self) -> *const c_void;
    unsafe fn is_enable_control(&self) -> bool;
    unsafe fn open(&mut self, target_state: DrawState) -> *const Self;
    unsafe fn close(&mut self, target_state: DrawState);
    unsafe fn get_ui_controller(&self) -> *const c_void;
    unsafe fn adjust_box_cursor_node(&mut self);
    unsafe fn create_box_cursor_node(&self, heap: Heap);
    unsafe fn replace_parts_layout_name(&self, param_2: *const c_void);
}

#[repr(C)]
pub struct Screen {
    pub base: IDisposer,
    pub screen_manager: *const ScreenMgr,
    pub layout: *const LayoutEx,
    pub button_group: *const ButtonGroup,
    pub control_list: ListNode,
    pub static_control_list: ListNode,

    // TODO: create struct
    pub ui_controller: *const c_void,

    // TODO: create struct
    pub draw_info: *const c_void,

    // TODO: create struct
    pub tag_processor: *const c_void,
    pub animator_list: ListNode,
    pub box_cursor_list: OffsetList<BoxCursorNode>,
    pub unk_98: OffsetList<()>,
    pub animated_parts_screen_list: OffsetList<()>,
    pub non_animated_parts_screen_list: OffsetList<()>,

    pub heap: *const Heap,
    pub index: i32,
    pub unk_ec: i32,
    pub name: *const c_char,
    pub active_box_cursor_node: *const BoxCursorNode,
    pub reserved_active_box_cursor_node: *const BoxCursorNode,
    pub elink_user: *const c_void,
    pub slink_user: *const c_void,
    pub camera_fov: f32,
    pub draw_unit_id: u8,
    pub draw_state: DrawState,
    pub screen_state: ScreenState,
    pub extended_close_state: ExtendedCloseState,
    pub is_initialized: bool,
    pub unk_121: u8,
    pub is_force_inactive: bool,
    pub is_no_operation_button_on_se_enabled: bool,
    pub input_flags: u32,
}

const _: () = assert!(core::mem::size_of::<Screen>() == 0x128);
