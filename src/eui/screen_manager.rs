use std::ffi::c_void;

use crate::{
    eui::{ButtonBase, ButtonGroup, ButtonHitHandler, LayoutEx},
    sead::{
        container::{Buffer, ListNode, OffsetList, PtrArray},
        heap::{Heap, IDisposer},
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

#[repr(C)]
pub struct BaseScreenVtable<T> {
    pub destructor1: extern "C" fn(u64),
    pub destructor2: extern "C" fn(u64),

    pub check_derived_runtime_type_info: extern "C" fn(u64, u64) -> u64,
    pub get_runtime_type_info: extern "C" fn() -> u64,

    pub is_enable_control: extern "C" fn(u64) -> u64,
    pub open: extern "C" fn(u64, i32) -> u64,  // eui::screen
    pub close: extern "C" fn(u64, i32) -> u64, // eui::screen
    pub get_ui_controller: extern "C" fn() -> u64,

    pub adjust_box_cursor: extern "C" fn(u64, u64),
    pub create_box_cursor_node: extern "C" fn(u64, u64),

    pub replace_parts_layout_name: extern "C" fn(u64, u64) -> u64, // eui::screen
    pub unk_0x5b: extern "C" fn(),                                 // eui::screen

    pub set_animator_state: extern "C" fn(),
    pub do_create_letter_anim_control: extern "C" fn(u64, u64) -> u64, // eui::screen
    pub do_create_number_anim_control: extern "C" fn(u64, u64) -> u64, // eui::screen

    pub post_initialize: extern "C" fn(), // eui::screen
    pub initialize: extern "C" fn(u64, f32, u64, u64, u64, u64, u64, u32, u64, u64), // eui::screen
    pub update: extern "C" fn(u64),
    pub draw: extern "C" fn(u64, u64),

    pub unk_0x98: extern "C" fn(), // eui::screen
    pub unk_0xa0: extern "C" fn(), // eui::screen

    pub get_layout_name: extern "C" fn(u64) -> u64,

    pub get_message_name: extern "C" fn(u64) -> u64, // eui::screen
    pub get_message_name_2: extern "C" fn(u64) -> u64, // eui::screen
    pub is_play_parts_in_out: extern "C" fn() -> u64, // eui::screen

    pub is_disallow_hit_lower_screen_on_button_hit: extern "C" fn() -> u64,

    pub do_create_layout: extern "C" fn(u64, u64) -> u64, // eui::screen
    pub do_create_draw_info_ex: extern "C" fn(u64, u64) -> u64, // eui::screen
    pub do_create_button_group: extern "C" fn(u64, u64) -> u64, // eui::screen

    pub do_after_build_layout: extern "C" fn(*mut T),

    pub do_setup_draw_info: extern "C" fn(u64), // eui::screen
    pub do_create_ui_controller: extern "C" fn() -> u64, // eui::screen
    pub do_create_resource_accessor: extern "C" fn(u64, u64) -> u64, // eui::screen

    pub do_create_tag_processor: extern "C" fn(u64, u64) -> u64, // eui::screen
    pub do_build_layout: extern "C" fn(u64, u64, u64),           // eui::screen
    pub do_build_layout_impl_: extern "C" fn(u64, u64, u64, u64, u64) -> u32, // eui::screen
    pub do_load_resource: extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64), // eui::screen
    pub do_create_slide_list_control: extern "C" fn(u64, u64, u64, u64) -> u64, // eui::screen
    pub do_initialize: extern "C" fn(u64),                       // eui::screen
    pub do_update: extern "C" fn(u64),                           // eui::screen
    pub update_button: extern "C" fn(u64),                       // eui::screen
    pub get_animation_step: extern "C" fn(u64) -> u32,           // eui::screen

    pub do_draw: extern "C" fn(u64, u64),

    pub do_open_start: extern "C" fn(u64),  // eui::screen
    pub do_open_end: extern "C" fn(u64),    // eui::screen
    pub do_close_start: extern "C" fn(u64), // eui::screen
    pub do_close_end: extern "C" fn(u64),   // eui::screen

    pub do_button_on_start: extern "C" fn(u64), // eui::screen
    pub do_button_on_end: extern "C" fn(u64),   // eui::screen
    pub do_button_off_start: extern "C" fn(u64), // eui::screen
    pub do_button_off_end: extern "C" fn(u64),  // eui::screen
    pub do_button_down_start: extern "C" fn(u64), // eui::screen
    pub do_button_down_end: extern "C" fn(u64, u64), // eui::screen
    pub do_button_cancel_start: extern "C" fn(u64), // eui::screen
    pub do_button_cancel_end: extern "C" fn(u64), // eui::screen

    pub unk_0x1b8: extern "C" fn() -> u64, // eui::screen

    pub create_asset_info_reader: extern "C" fn() -> u64,
    pub get_s_link_property_count: extern "C" fn() -> u64,
    pub set_s_link_property_def: extern "C" fn(),
    pub get_e_link_property_count: extern "C" fn() -> u64,
    pub set_e_link_property_def: extern "C" fn(),

    pub copy_controls: extern "C" fn(u64, u64, u64, u64) -> i32, // eui::screen
    pub is_line_feed_by_character_height: extern "C" fn() -> bool, // eui::screen

    pub unk_0x1f8: extern "C" fn(), // eui::screen
    pub unk_0x200: extern "C" fn(),

    pub update_control: extern "C" fn(u64),
    pub open_start: extern "C" fn(u64, u64),

    pub is_open_end: extern "C" fn(u64) -> bool, // eui::screen
    pub open_end: extern "C" fn(u64),            // eui::screen
    pub close_start: extern "C" fn(u64, u64),    // eui::screen
    pub is_close_end: extern "C" fn(u64) -> bool, // eui::screen
    pub close_end: extern "C" fn(u64),           // eui::screen

    pub is_force_glb_mtx_dirty: extern "C" fn() -> bool,
    pub update_animator: extern "C" fn(u64),
    pub register_controller: extern "C" fn(),
    pub unregister_controller: extern "C" fn(),

    pub setup_pane_after_build: extern "C" fn(u64, f32, u32, u32, u32, i32), // TODO: double-check
    pub do_initialize_layout: extern "C" fn(u64, u64),

    pub count_effect_link_pane: extern "C" fn(u64, u64, u64),
    pub create_effect_link_user: extern "C" fn(u64, u64, u32, u64, u64, u64, u64, u64), // eui::screen
    pub create_sound_link_2_user: extern "C" fn(u64, u64) -> u64, // eui::screen
    pub invoke_sound_link_2_event: extern "C" fn(u64, u64),       // eui::screen

    pub invoke_sound_link_2_button_event: extern "C" fn(u64, u64, u64) -> bool,
    pub invoke_sound_link_2_anim_play_event: extern "C" fn(f32, u64, u64, u64, u32),

    pub unk_2xa0: extern "C" fn(u64, u64) -> u32,
    pub unk_2xa8: extern "C" fn(u64) -> u8,  // eui::screen
    pub unk_2xb0: extern "C" fn(u64) -> u8,  // eui::screen
    pub unk_2xb8: extern "C" fn(u64) -> u64, // eui::screen
    pub unk_2xc0: extern "C" fn(u64) -> u8,  // eui::screen
    pub unk_2xc8: extern "C" fn(u64) -> u8,  // eui::screen

    pub unk_2xd0: extern "C" fn(u64) -> u64,
    pub unk_0x2d8: extern "C" fn(u64) -> u32,
    pub unk_0x2e0: extern "C" fn() -> u64, // mov w0, wzr -> ret

    pub unk_0x2e8: extern "C" fn(u64) -> u8, // eui::screen
    pub unk_0x2f0: extern "C" fn(u64) -> u8, // eui::screen
    pub unk_0x2f8: extern "C" fn() -> u64,   // eui::screen, mov x0, xzr -> ret
    pub unk_0x300: extern "C" fn(u64, u64),

    pub handle_input: extern "C" fn(u64),  // eui::screen
    pub on_state_change: extern "C" fn(),  // eui::screen
    pub unk_0x318: extern "C" fn() -> u64, // returns a pointer to string that says "N_CameraMove_00"

    pub app_finish_open: extern "C" fn(u64, u64),
    pub unk_0x328: extern "C" fn() -> u64, // mov w0, 0xffffffff -> ret
    pub unk_0x330: extern "C" fn() -> u64, // mov w0, 0x1 -> ret
    pub unk_0x338: extern "C" fn(u64) -> u8, // mov w0, [x0, 0x298] -> ret
    pub unk_0x340: extern "C" fn() -> u64, // mov w0, wzr -> ret
    pub unk_0x348: extern "C" fn() -> u64, // mov w0, 0x1 -> ret
    pub unk_0x350: extern "C" fn() -> u64, // mov w0, wzr -> ret
    pub unk_0x358: extern "C" fn() -> u64, // mov w0, wzr -> ret
    pub unk_0x360: extern "C" fn() -> u64, // mov w0, wzr -> ret

    pub app_setup_draw_info: extern "C" fn() -> u64, // mov w0, wzr -> ret

    pub unk_0x370: extern "C" fn() -> u64, // mov w0, wzr -> ret
    pub unk_0x378: extern "C" fn(u64) -> f32, // ldr s0, [x0, 0x320] -> fmov s1, 0x3f000000 -> fmul s0, s0, s1 -> ret
    pub unk_0x380: extern "C" fn(u64) -> u32, // ldr s0, [x0, 0x320] -> ret
    pub unk_0x388: extern "C" fn(u64) -> f32, // ldr s0, [x0, 0x324] -> fmov s1, 0x3f000000 -> fmul s0, s0, s1 -> ret
    pub unk_0x390: extern "C" fn(u64) -> u32, // ldr s0, [x0, 0x324] -> ret

    pub app_do_initialize: extern "C" fn(*mut T),
    pub app_open_start: extern "C" fn(*mut T),
    pub app_open_end: extern "C" fn(), // ret

    pub app_close_start: extern "C" fn(*mut T),
    pub app_close_end: extern "C" fn(*mut T),
    pub app_do_update: extern "C" fn(*mut T),

    pub is_deselect_box_cursor_on_close: extern "C" fn() -> bool, // mov w0, wzr -> ret
    pub unk_0x3d0: extern "C" fn(),                               // ret
    pub unk_0x3d8: extern "C" fn(),                               // ret
    pub unk_0x3e0: extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64),

    pub app_button_on_start: extern "C" fn(u64), // ret
    pub app_button_on_end: extern "C" fn(),      // ret

    pub app_button_off_start: extern "C" fn(), // ret
    pub app_button_off_end: extern "C" fn(),   // ret

    pub app_button_down_start: extern "C" fn(*mut T, *mut ButtonBase),
    pub app_button_down_end: extern "C" fn(), // ret

    pub app_button_cancel_start: extern "C" fn(), // ret
    pub app_button_cancel_end: extern "C" fn(),   // ret

    pub unk_0x428: extern "C" fn(), // ret,
    pub unk_0x430: extern "C" fn(), // ret
    pub unk_0x438: extern "C" fn(), // ret
    pub unk_0x440: extern "C" fn(),
    pub unk_0x448: extern "C" fn(u64, u64, u64),
    pub unk_0x450: extern "C" fn(u64),     // mov w0, 0x1 -> ret
    pub unk_0x458: extern "C" fn() -> u64, // mov x0, xzr -> ret

    pub get_state_machine: extern "C" fn() -> u64, // ret
    pub register_states: extern "C" fn(),          // ret
    pub change_state: extern "C" fn(u64, u32),
    pub trigger_scene_transition: extern "C" fn(u64),

    pub play_screen_open_audio: extern "C" fn(u64), // eui::screen
}

#[repr(C, packed)]
pub struct BaseScreen<V = BaseScreenVtable<c_void>> {
    // Inherited from IDisposer
    pub vtable: *const V,
    pub disposer_heap: *mut Heap,
    pub disposer_list_node: ListNode,

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

const _: () = assert!(core::mem::size_of::<BaseScreen>() == 0x348);
const _: () = assert!(core::mem::size_of::<ScreenFactory>() == 0x30);
const _: () = assert!(core::mem::size_of::<ScreenFactoryVtable>() == 0xc0);
const _: () = assert!(core::mem::size_of::<ScreenInfo>() == 0x38);
const _: () = assert!(core::mem::size_of::<ScreenManager>() == 0x1470);

impl<V> BaseScreen<V> {
    #[inline(always)]
    pub fn as_ptr(&self) -> u64 {
        self as *const _ as u64
    }

    #[inline(always)]
    pub fn vtable(&self) -> &BaseScreenVtable<u64> {
        unsafe { &*(self.vtable as *const BaseScreenVtable<u64>) }
    }

    pub fn close(&self, target_state: DrawState) -> u64 {
        (self.vtable().close)(self.as_ptr(), target_state as i32)
    }

    pub fn open(&self, target_state: DrawState) -> u64 {
        (self.vtable().open)(self.as_ptr(), target_state as i32)
    }

    pub fn get_ui_controller(&self) -> u64 {
        (self.vtable().get_ui_controller)()
    }

    pub fn is_visible(&self) -> bool {
        self.screen_state == ScreenState::Opened && self.draw_state > DrawState::Closing
    }
}
