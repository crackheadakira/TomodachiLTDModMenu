use std::ffi::c_void;

use crate::{
    eui::{ButtonGroup, ButtonHitHandler, LayoutEx},
    sead::{
        container::{Buffer, ListNode, OffsetList, PtrArray},
        heap::IDisposer,
        prim::SafeString,
        thread::CriticalSection,
    },
};

#[derive(PartialEq, PartialOrd)]
#[repr(i8)]
pub enum DrawState {
    Closing = -1,
    None = 0,
    Opening = 1,
    OpenStart = 2,
    OpenAnim = 3,
    OpenEnd = 4,
}

#[derive(PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ScreenState {
    Closed = 0,
    Opening = 1,
    Opened = 2,
    Closing = 3,
}

#[repr(C, packed)]
pub struct BaseScreen<V = c_void> {
    pub base_idisposer: IDisposer<V>,
    pub screen_manager: *mut ScreenManager,
    pub layout: *mut LayoutEx,
    pub button_group: *mut ButtonGroup,
    pub control_list: ListNode,
    pub unk_48: ListNode,
    pub ui_controller: *const c_void,
    pub draw_info: *const c_void,
    pub tag_processor: *const c_void,
    pub animator_list: ListNode,
    pub box_cursor_list: OffsetList<c_void>,
    pub unk_98: OffsetList<c_void>,
    pub button_hit_handler: ButtonHitHandler,
    pub pad_100: [u8; 24],
    pub camera_fov: f32,
    pub input_mode: u8,
    pub draw_state: DrawState,
    pub screen_state: ScreenState,
    pub unk_11f: u8,
    pub unk_120: u8,
    pub unk_state_2: u16,
    pub is_visible: bool,
    pub input_flags: u32,
    pub secondary_vtable: *const c_void,
    pub delegate_1: [u8; 32],
    pub unk_150: u64,

    pub fixed_pool_1: [u8; 96],
    pub pool_buffer_1: *const c_void,
    pub unk_1c0: [u8; 56],

    pub fixed_pool_2: [u8; 96],
    pub pool_buffer_2: *const c_void,
    pub unk_260: [u8; 56],
    pub scale_enabled: bool,
    pub pad_299: [u8; 3],
    pub base_scale: f32,
    pub screen_lock: CriticalSection,

    pub pad_2de: [u8; 104],
}

impl<T> BaseScreen<T> {
    pub fn is_visible(&self) -> bool {
        self.screen_state == ScreenState::Opened && self.draw_state > DrawState::Closing
    }
}

// TODO: fill it in properly
#[repr(C)]
pub struct ScreenFactoryVtable {
    pub check_derived_runtime_type_info: extern "C" fn(),
    pub get_runtime_type_info: extern "C" fn(),
    pub dtor_1: extern "C" fn(),
    pub dtor_2: extern "C" fn(),
    pub create_screen: extern "C" fn(),
    pub get_screen_name: extern "C" fn(),
    pub get_screen_count: extern "C" fn(),
    pub get_draw_target_from_draw_unit_id: extern "C" fn(),
    pub get_screen_index_1: extern "C" fn(),
    pub get_screen_index_2: extern "C" fn(),
    pub get_hash: extern "C" fn(),
    pub get_draw_unit_id: extern "C" fn(),
    pub get_is_touch: extern "C" fn(),
    pub get_is_enable_control: extern "C" fn(),
    pub get_body_layout: extern "C" fn(),
    pub get_class: extern "C" fn(),
    pub get_heap_direction: extern "C" fn(),
    pub get_is_pause_lower: extern "C" fn(),
    pub get_is_lock_request_force_cancel: extern "C" fn(),
    pub get_is_ignore_pick: extern "C" fn(),
    pub unk_a0: extern "C" fn(),
    pub get_category_count: extern "C" fn(),
    pub get_category_name: extern "C" fn(),
    pub get_heap_initial_size: extern "C" fn(),
}

#[repr(C)]
pub struct ScreenFactory {
    pub vtable: *const ScreenFactoryVtable,
    pub screen_array: Buffer<ScreenInfo>,
    pub index_array: *mut i32,
    pub count: i32,
    pub capacity: i32,
    pub screen_count: i32,
    pub unk_2c: i32,
}

#[repr(C)]
pub struct ScreenInfo {
    pub name: SafeString,
    pub class: SafeString,
    pub body_layout: SafeString,
    pub archive_category_iter: [u8; 10],
    pub hash: u32,
    pub heap_initial_size: u32,
    pub draw_unit_id: u8,
    pub flags: u8,
    pub pad_33: [u8; 6],
}

#[repr(C)]
pub struct ScreenManager {
    pub vtable: *const c_void,
    pub screen_array: PtrArray<BaseScreen>,
    pub draw_unit_id_array: Buffer<u8>,
    pub screen_factory: *mut ScreenFactory,
    pub ui_2d_graphics_resource: [u8; 0x3c0], // TODO: create proper struct, maybe.
    pub arc_resource_manager: *const c_void,
    pub box_cursor_manager: *const c_void,
    pub animation_step: f32,
    pub pad_404: [u8; 0x30B],
    pub unk_cs: CriticalSection,
    pub pad_750: [u8; 0x577],
    pub cs: CriticalSection,
    pub pad_d08: [u8; 0x73E],
    pub is_active: bool,
    pub pad_1447: [u8; 5],
    pub initialized: bool,
    pub pad_144d: [u8; 3],
    pub static_disposer: IDisposer,
}

const _: () = assert!(core::mem::size_of::<BaseScreen<c_void>>() == 0x348);
const _: () = assert!(core::mem::size_of::<ScreenFactory>() == 0x30);
const _: () = assert!(core::mem::size_of::<ScreenFactoryVtable>() == 0xc0);
const _: () = assert!(core::mem::size_of::<ScreenInfo>() == 0x38);
const _: () = assert!(core::mem::size_of::<ScreenManager>() == 0x1470);
