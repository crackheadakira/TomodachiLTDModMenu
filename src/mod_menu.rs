use std::ffi::{c_char, c_void};
use std::mem::MaybeUninit;
use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::sead::{
    container::{ListNode, OffsetList},
    heap::{Heap, IDisposer},
    thread::CriticalSection,
};
use crate::{eui::EuiController, ui_framework::ButtonState};

pub const fn murmurhash3(data: &[u8]) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;

    let len = data.len() as u32;
    let mut hash: u32 = 0;

    let mut i = 0;
    let nblocks = data.len() / 4;

    while i < nblocks {
        let base = i * 4;

        let mut k = (data[base] as u32)
            | ((data[base + 1] as u32) << 8)
            | ((data[base + 2] as u32) << 16)
            | ((data[base + 3] as u32) << 24);

        k = k.wrapping_mul(C1);
        k = k.rotate_left(15);
        k = k.wrapping_mul(C2);

        hash ^= k;
        hash = hash.rotate_left(13);
        hash = hash.wrapping_mul(5).wrapping_add(0xe6546b64);

        i += 1;
    }

    let tail_index = nblocks * 4;
    let mut k1: u32 = 0;

    let rem = data.len() & 3;
    if rem == 3 {
        k1 ^= (data[tail_index + 2] as u32) << 16;
        k1 ^= (data[tail_index + 1] as u32) << 8;
        k1 ^= data[tail_index] as u32;
    } else if rem == 2 {
        k1 ^= (data[tail_index + 1] as u32) << 8;
        k1 ^= data[tail_index] as u32;
    } else if rem == 1 {
        k1 ^= data[tail_index] as u32;
    }

    if rem != 0 {
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);
        hash ^= k1;
    }

    hash ^= len;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    hash
}

pub static MOD_MENU_OBJ_PTR: AtomicU64 = AtomicU64::new(0);
pub const MOD_MENU_HASH: u32 = murmurhash3(b"ModMenu");

#[skyline::hook(offset = 0x4a99bc)]
pub fn capture_screen_heap(scene_manager: u64, param_2: u64) {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        register_mod_menu_screen();
        let create_heap: extern "C" fn(u64, *const u64, u64, u32, u32, bool) -> u64 =
            std::mem::transmute(text_base + 0x3adad8);

        let heap_name = b"ScreenStaticHeap\0";
        let heap_name_ptr = heap_name.as_ptr() as u64;
        let new_heap = create_heap(0, &heap_name_ptr as *const _, param_2, 8, 1, false);

        *((scene_manager + 0xb8) as *mut u64) = new_heap;
        *((scene_manager + 0x144c) as *mut u8) = 0;

        let get_or_create: extern "C" fn(u64, u32) -> u64 =
            std::mem::transmute(text_base + 0x4a9a90);

        let mod_screen = get_or_create(new_heap, MOD_MENU_HASH);
        if mod_screen != 0 {
            MOD_MENU_OBJ_PTR.store(mod_screen, Ordering::SeqCst);
            println!("[Mod] Stored ModMenu pointer: {mod_screen:#X}");
        } else {
            println!("[Mod] GetOrCreateScene rejected ModMenu!");
        }

        for i in 0..89 {
            let offset = i * 8;
            let hash_ptr = (text_base + 0x2a03d28 + offset) as *const u32;
            let active_flag_ptr = (text_base + 0x2a03d2c + offset) as *const u8;

            let hash = *hash_ptr;
            let screen = get_or_create(new_heap, hash);

            if screen != 0 && *active_flag_ptr != 0 {
                let vtable = *(screen as *const *const u64);
                let adjust: extern "C" fn(u64, u32) = std::mem::transmute(*vtable.add(5));
                adjust(screen, 1);
            }
        }

        *((scene_manager + 0x144c) as *mut u8) = 1;

        let fun_post_init: extern "C" fn(u64) = std::mem::transmute(text_base + 0x4b3510);
        fun_post_init(scene_manager);

        let heap_vtable = *(new_heap as *const *const u64);
        let heap_adjust: extern "C" fn(u64) = std::mem::transmute(*heap_vtable.add(5));
        heap_adjust(new_heap);

        println!("[Mod] ScreenStaticHeap initialization complete and sealed.");
    }
}

extern "C" fn get_mod_menu_class_name() -> *const u8 {
    b"ScreenModMenu\0".as_ptr()
}

extern "C" fn mod_menu_allocate(param_1: u64, heap: u64) -> u64 {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        println!("[Mod] allocate heap={heap:#X}");

        let heap_alloc: extern "C" fn(u64, u64, u32) -> u64 =
            std::mem::transmute(text_base + 0x1e4c20);

        let obj = heap_alloc(0x4d0, heap, 8);

        println!("[Mod] Got obj at: {obj:#X}");
        mod_menu_ctor(obj);

        obj
    }
}

static mut MOD_MENU_FACTORY_NODE: [u64; 4] = [0u64; 4];
static mut MOD_MENU_INTERMEDIATE_VTABLE: [u64; 0x4D0] = [0; 0x4D0];

#[repr(C)]
struct UIManagerRegistryNode {
    pub magic: u32,
    pub padding: u32,
    pub factory_ptr: u64,
}

static mut MOD_MENU_REGISTRY_ENTRY: UIManagerRegistryNode = UIManagerRegistryNode {
    magic: 0x6a24df,
    padding: 0,
    factory_ptr: 0,
};

pub unsafe fn register_mod_menu_screen() {
    let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    initialize_vtable(text_base);
    initialize_secondary_vtable(text_base);
    initialize_type_info(text_base);

    let register: extern "C" fn(*const u64) = std::mem::transmute(text_base + 0x5f5ce0);

    let ptr_to_factory_field = &raw mut MOD_MENU_REGISTRY_ENTRY.factory_ptr;

    register(ptr_to_factory_field);

    MOD_MENU_FACTORY_NODE[0] = text_base + 0x2320408; // FUN_7102320408 base dtor
    MOD_MENU_FACTORY_NODE[1] = text_base + 0x20a74d4; // FUN_71020a74d4 dtor+delete
    MOD_MENU_FACTORY_NODE[2] = get_mod_menu_class_name as *const () as u64;
    MOD_MENU_FACTORY_NODE[3] = mod_menu_allocate as *const () as u64;

    MOD_MENU_REGISTRY_ENTRY.factory_ptr = MOD_MENU_FACTORY_NODE.as_ptr() as u64;

    let second_register: extern "C" fn(u64, *mut u64, u64) =
        std::mem::transmute(text_base + 0x2c44a0);

    second_register(
        text_base + 0x2320408,
        ptr_to_factory_field,
        text_base + 0x3023fb8,
    );

    println!("[Mod] ScreenModMenu registered");
}

static mut MOD_MENU_SECONDARY_VTABLE: [u64; 21] = [0u64; 21];

pub unsafe fn initialize_secondary_vtable(text_base: u64) {
    MOD_MENU_SECONDARY_VTABLE[0] = mod_menu_secondary_vtable_destructor1 as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[1] = mod_menu_secondary_vtable_destructor2 as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[2] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[3] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[4] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[5] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[6] = stub_zero as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[7] = stub_zero as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[8] = text_base + 0x215c590;
    MOD_MENU_SECONDARY_VTABLE[9] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[10] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[11] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[12] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[13] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[14] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[15] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[16] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[17] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[18] = stub as *const () as u64;
    MOD_MENU_SECONDARY_VTABLE[19] = stub as *const () as u64;
}

fn mod_menu_secondary_vtable_destructor1(secondary_this: u64) {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let flag_380 = (secondary_this + 0x380) as *mut u8;
        if *flag_380 != 0 {
            *flag_380 = 0;
        }

        let flag_378 = (secondary_this + 0x378) as *mut u8;
        if *flag_378 != 0 {
            *flag_378 = 0;
        }

        let obj_base = secondary_this - 0x128;

        let inter_base = MOD_MENU_INTERMEDIATE_VTABLE.as_ptr() as u64;

        *(secondary_this as *mut u64) = inter_base + 0x4c0;

        *(obj_base as *mut u64) = inter_base + 0x10;

        let base_dtor: extern "C" fn(u64) = std::mem::transmute(text_base + 0x8a7878);
    }
}

fn mod_menu_secondary_vtable_destructor2(secondary_this: u64) {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        mod_menu_secondary_vtable_destructor1(secondary_this);

        let op_delete: extern "C" fn(u64) = std::mem::transmute(text_base + 0x3e510);
        op_delete(secondary_this);
    }
}

extern "C" fn mod_menu_ctor(obj: u64) {
    println!("[ModMenuConstructor] Start");
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let base_ctor: extern "C" fn(u64) = std::mem::transmute(text_base + 0x76e8c4);

        println!("[ModMenuConstructor] calling base_ctor");
        base_ctor(obj);

        let inter_base = MOD_MENU_INTERMEDIATE_VTABLE.as_ptr() as u64;
        *(obj as *mut u64) = inter_base + 0x10;
        *((obj + 0x128) as *mut u64) = inter_base + 0x4c0;

        let init_pane_list: extern "C" fn(u64) = std::mem::transmute(text_base + 0x897194);

        println!("[ModMenuConstructor] calling init_pane_list");
        init_pane_list(obj + 0x348);

        *((obj + 0x4a4) as *mut u32) = 0;
        *((obj + 0x4a8) as *mut u8) = 0;
        *((obj + 0x488) as *mut u8) = 1;
        *((obj + 0x489) as *mut u8) = 0;
        *((obj + 0x4cc) as *mut u32) = 0;
        *((obj + 0x48a) as *mut u8) = 0;
        *((obj + 0x490) as *mut u8) = 0;
        *((obj + 0x494) as *mut i32) = -1;

        *(obj as *mut u64) = MOD_MENU_VTABLE.as_ptr() as u64;

        *((obj + 0x498) as *mut u64) = 0;
        *((obj + 0x128) as *mut u64) = MOD_MENU_SECONDARY_VTABLE.as_ptr() as u64 + 0x10;

        *((obj + 0x4a0) as *mut u8) = 0;
        *((obj + 0x4b0) as *mut u64) = 0;
        *((obj + 0x4b8) as *mut u64) = 0;
        *((obj + 0x4c0) as *mut u64) = 0;
        *((obj + 0x4c8) as *mut u8) = 0;
        *((obj + 0x4c9) as *mut u8) = 0;
        *((obj + 0x4cc) as *mut u32) = 0;

        println!("[ModMenuConstructor] Done");
    }
}

#[repr(C)]
pub struct ModMenuVTable {
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

    pub do_after_build_layout: extern "C" fn(u64),

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

    pub app_do_initialize: extern "C" fn(*mut ScreenModMenu),
    pub app_open_start: extern "C" fn(*mut ScreenModMenu),
    pub app_open_end: extern "C" fn(), // ret

    pub app_close_start: extern "C" fn(*mut ScreenModMenu),
    pub app_close_end: extern "C" fn(u64),
    pub app_do_update: extern "C" fn(*mut ScreenModMenu), // TODO: double-check

    pub is_deselect_box_cursor_on_close: extern "C" fn() -> bool, // mov w0, wzr -> ret
    pub unk_0x3d0: extern "C" fn(),                               // ret
    pub unk_0x3d8: extern "C" fn(),                               // ret
    pub unk_0x3e0: extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64),
    pub unk_0x3e8: extern "C" fn(u64),

    pub app_button_on_start: extern "C" fn(), // ret
    pub app_button_on_end: extern "C" fn(),   // ret

    pub app_button_off_start: extern "C" fn(), // ret
    pub app_button_off_end: extern "C" fn(),   // ret

    pub app_button_down_start: extern "C" fn(u64, u64), // ret
    pub app_button_down_end: extern "C" fn(),           // ret

    pub app_button_cancel_start: extern "C" fn(), // ret
    pub app_button_cancel_end: extern "C" fn(),   // ret

    pub unk_0x430: extern "C" fn(), // ret
    pub unk_0x438: extern "C" fn(), // ret
    pub unk_0x440: extern "C" fn(),
    pub unk_0x448: extern "C" fn(u64, u64, u64),
    pub unk_0x450: extern "C" fn(u64),     // mov w0, 0x1 -> ret
    pub unk_0x458: extern "C" fn() -> u64, // mov x0, xzr -> ret

    pub on_process_command: extern "C" fn() -> u64, // ret
    pub is_scene_mgr_thing: extern "C" fn(),        // ret
    pub initialize_scene_mgr_thing: extern "C" fn(u64, u32),
    pub unk_0x478: extern "C" fn(u64),

    pub app_open_start_2: extern "C" fn(u64), // eui::screen
    pub set_component_closed_state: extern "C" fn(u64, i32, u32),

    pub unk_0x490: extern "C" fn(*mut ScreenModMenu, u32),
    pub unk_0x498: extern "C" fn(*mut ScreenModMenu),
}

extern "C" fn stub_max() -> u64 {
    0xffffffff
}

extern "C" fn stub_one() -> u64 {
    1
}

extern "C" fn stub_zero() -> u64 {
    0
}

extern "C" fn stub_true() -> bool {
    true
}

extern "C" fn stub_false() -> bool {
    false
}

extern "C" fn stub() {}

pub static mut MOD_MENU_VTABLE: MaybeUninit<ModMenuVTable> = MaybeUninit::uninit();

pub unsafe fn initialize_vtable(text_base: u64) {
    MOD_MENU_VTABLE.write(ModMenuVTable {
        destructor1: mod_menu_destructor1,
        destructor2: mod_menu_destructor2,
        check_derived_runtime_type_info: mod_menu_check_rtti,
        get_runtime_type_info: mod_menu_get_rtti,
        is_enable_control: mod_menu_is_enable_control,
        open: std::mem::transmute(text_base + 0x8d9654),
        close: std::mem::transmute(text_base + 0x7ac858),
        get_ui_controller: std::mem::transmute(text_base + 0x6911c0),
        adjust_box_cursor: std::mem::transmute(text_base + 0x215c790),
        create_box_cursor_node: std::mem::transmute(text_base + 0xa82054),
        replace_parts_layout_name: std::mem::transmute(text_base + 0x231def8),
        unk_0x5b: stub,
        set_animator_state: stub,
        do_create_letter_anim_control: std::mem::transmute(text_base + 0x215b670),
        do_create_number_anim_control: std::mem::transmute(text_base + 0x231df34),
        post_initialize: stub,
        initialize: std::mem::transmute(text_base + 0x6db5ec),
        update: std::mem::transmute(text_base + 0x32c788),
        draw: std::mem::transmute(text_base + 0x231dddc),
        unk_0x98: stub,
        unk_0xa0: stub,
        get_layout_name: std::mem::transmute(text_base + 0x9554e4),
        get_message_name: std::mem::transmute(text_base + 0x5ef674),
        get_message_name_2: std::mem::transmute(text_base + 0x6dbff8),
        is_play_parts_in_out: stub_zero,
        is_disallow_hit_lower_screen_on_button_hit: stub_one,
        do_create_layout: std::mem::transmute(text_base + 0xa81ce4),
        do_create_draw_info_ex: std::mem::transmute(text_base + 0x6dbb84),
        do_create_button_group: std::mem::transmute(text_base + 0x6dbb3c),
        do_after_build_layout: std::mem::transmute(text_base + 0xa7f858),
        do_setup_draw_info: std::mem::transmute(text_base + 0x6ec2b4),
        do_create_ui_controller: stub_zero,
        do_create_resource_accessor: std::mem::transmute(text_base + 0x6dbe10),
        do_create_tag_processor: std::mem::transmute(text_base + 0xa81e90),
        do_build_layout: std::mem::transmute(text_base + 0x6c9898),
        do_build_layout_impl_: std::mem::transmute(text_base + 0x6c9980),
        do_load_resource: std::mem::transmute(text_base + 0xa81d38),
        do_create_slide_list_control: std::mem::transmute(text_base + 0x80b7ec),
        do_initialize: std::mem::transmute(text_base + 0x7493ec),
        do_update: std::mem::transmute(text_base + 0x50520),
        update_button: std::mem::transmute(text_base + 0x243a70),
        get_animation_step: std::mem::transmute(text_base + 0x6dbb20),
        do_draw: std::mem::transmute(text_base + 0x231e1ac),
        do_open_start: std::mem::transmute(text_base + 0x76477c),
        do_open_end: std::mem::transmute(text_base + 0x7426e8),
        do_close_start: std::mem::transmute(text_base + 0x796834),
        do_close_end: std::mem::transmute(text_base + 0x7a9010),
        do_button_on_start: std::mem::transmute(text_base + 0x215c648),
        do_button_on_end: std::mem::transmute(text_base + 0x215c654),
        do_button_off_start: std::mem::transmute(text_base + 0x215c660),
        do_button_off_end: std::mem::transmute(text_base + 0x215c66c),
        do_button_down_start: std::mem::transmute(text_base + 0x215c678),
        do_button_down_end: std::mem::transmute(text_base + 0x215c684),
        do_button_cancel_start: std::mem::transmute(text_base + 0x215c778),
        do_button_cancel_end: std::mem::transmute(text_base + 0x215c784),
        unk_0x1b8: stub_zero,
        create_asset_info_reader: stub_zero,
        get_s_link_property_count: stub_zero,
        set_s_link_property_def: stub,
        get_e_link_property_count: stub_zero,
        set_e_link_property_def: stub,
        copy_controls: std::mem::transmute(text_base + 0x82a2e0),
        is_line_feed_by_character_height: stub_false,
        unk_0x1f8: stub,
        unk_0x200: stub,
        update_control: std::mem::transmute(text_base + 0x231dc04),
        open_start: std::mem::transmute(text_base + 0x24717c),
        is_open_end: std::mem::transmute(text_base + 0x246b68),
        open_end: std::mem::transmute(text_base + 0x246b98),
        close_start: std::mem::transmute(text_base + 0x247370),
        is_close_end: std::mem::transmute(text_base + 0x246b30),
        close_end: std::mem::transmute(text_base + 0x246e38),

        is_force_glb_mtx_dirty: stub_false,
        update_animator: std::mem::transmute(text_base + 0x215a920),
        register_controller: stub,
        unregister_controller: stub,
        setup_pane_after_build: std::mem::transmute(text_base + 0x215a954),
        do_initialize_layout: std::mem::transmute(text_base + 0x87fedc),

        count_effect_link_pane: std::mem::transmute(text_base + 0x231e8f0),
        create_effect_link_user: std::mem::transmute(text_base + 0x7e5de8),
        create_sound_link_2_user: std::mem::transmute(text_base + 0x6dbc34),

        invoke_sound_link_2_event: std::mem::transmute(text_base + 0x4f91e8),
        invoke_sound_link_2_button_event: std::mem::transmute(text_base + 0x215c5b0),
        invoke_sound_link_2_anim_play_event: std::mem::transmute(text_base + 0x356c00),

        unk_2xa0: std::mem::transmute(text_base + 0x215b9e8),

        unk_2xa8: std::mem::transmute(text_base + 0x215baa4),
        unk_2xb0: std::mem::transmute(text_base + 0x215baf0),
        unk_2xb8: std::mem::transmute(text_base + 0x215bb40),
        unk_2xc0: std::mem::transmute(text_base + 0x1a7afbc),
        unk_2xc8: std::mem::transmute(text_base + 0x1a7afc4),

        unk_2xd0: std::mem::transmute(text_base + 0x4353e0),
        unk_0x2d8: std::mem::transmute(text_base + 0x395b8c),
        unk_0x2e0: stub_zero,

        unk_0x2e8: std::mem::transmute(text_base + 0x1a7afd4),
        unk_0x2f0: std::mem::transmute(text_base + 0x1a7afdc),
        unk_0x2f8: stub_zero,

        unk_0x300: std::mem::transmute(text_base + 0x7fbac4),
        handle_input: std::mem::transmute(text_base + 0x215acc4),
        on_state_change: stub,

        unk_0x318: std::mem::transmute(text_base + 0x6db704),

        app_finish_open: std::mem::transmute(text_base + 0x742790),

        unk_0x328: stub_max,
        unk_0x330: stub_one,
        unk_0x338: std::mem::transmute(text_base + 0x4353d8),
        unk_0x340: stub_zero,

        unk_0x348: stub_one,
        unk_0x350: stub_zero,
        unk_0x358: stub_zero,
        unk_0x360: stub_zero,

        app_setup_draw_info: stub_zero,

        unk_0x370: stub_zero,
        unk_0x378: std::mem::transmute(text_base + 0x1a7b018),
        unk_0x380: std::mem::transmute(text_base + 0x1a7b028),
        unk_0x388: std::mem::transmute(text_base + 0x1a7b030),
        unk_0x390: std::mem::transmute(text_base + 0x1a7b040),

        app_do_initialize: mod_menu_app_do_initialize,
        app_open_start: mod_menu_app_open_start,
        app_open_end: stub,
        app_close_start: mod_menu_app_close_start,
        app_close_end: mod_menu_app_close_end,
        app_do_update: mod_menu_app_do_update,

        is_deselect_box_cursor_on_close: stub_false,

        unk_0x3d0: stub,
        unk_0x3d8: stub,
        unk_0x3e0: std::mem::transmute(text_base + 0x215c4e0),
        unk_0x3e8: mod_menu_unk_0x3e8,

        app_button_on_start: stub,
        app_button_on_end: stub,

        app_button_off_start: stub,
        app_button_off_end: stub,

        app_button_down_start: my_button_click_handler,
        app_button_down_end: stub,

        app_button_cancel_start: stub,
        app_button_cancel_end: stub,

        unk_0x430: stub,
        unk_0x438: stub,
        unk_0x440: stub,
        unk_0x448: std::mem::transmute(text_base + 0x215c19c),
        unk_0x450: std::mem::transmute(text_base + 0x215c27c),
        unk_0x458: stub_zero,

        on_process_command: stub_zero,
        is_scene_mgr_thing: stub,
        initialize_scene_mgr_thing: std::mem::transmute(text_base + 0x8cb484),

        unk_0x478: std::mem::transmute(text_base + 0x7647f4),

        app_open_start_2: std::mem::transmute(text_base + 0x79692c),
        set_component_closed_state: std::mem::transmute(text_base + 0x215bcec),

        unk_0x490: mod_menu_unk_0x490,
        unk_0x498: mod_menu_unk_0x498,
    });
}

extern "C" fn mod_menu_app_do_initialize(this: *mut ScreenModMenu) {
    let this_ptr = this;
    let this = unsafe { &mut *this };

    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let layout = this.base.layout_panes as u64;

        this.is_initialized = true;

        this.unk_48a = [true; 8];

        let find_anim: extern "C" fn(u64, *const u8, i32) -> u64 =
            std::mem::transmute(text_base + 0x48a84);

        let in_bg = find_anim(layout, b"InFromBG\0".as_ptr(), 0);
        this.anim_in_from_bg = in_bg as *const c_void;

        let out_bg = find_anim(layout, b"OutToBG\0".as_ptr(), 0);
        this.anim_out_to_bg = out_bg as *const c_void;

        let short_in = find_anim(layout, b"ShortIn\0".as_ptr(), 0);
        this.anim_short_in = short_in as *const c_void;

        let in_after = find_anim(layout, b"InAfter\0".as_ptr(), 0);

        println!(
            "[Mod] All animations bound. In: {:#X}, Out: {:#X}, Short: {:#X}",
            in_bg, out_bg, short_in
        );
    }
}

extern "C" fn mod_menu_app_open_start(this: *mut ScreenModMenu) {
    let this_ptr = this;
    let this = unsafe { &mut *this };

    println!("[Mod] AppOpenStart entered!");
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let mut layout_msg_buf = [0u64; 2];
        let scene_manager = (this.base.scene_manager) as *mut u64 as u64;
        let layout_manager = this.base.layout_manager as *mut u64 as u64;

        let load_text_from_mal: extern "C" fn(
            u64,
            f32,
            u64,
            *const *const u8,
            *const *const u8,
            u64,
        ) -> bool = std::mem::transmute(text_base + 0x6eb968);

        let text_pane = b"T_Text_00\0".as_ptr();
        let text_id = b"L_ModBtn_03\0".as_ptr();

        let component_6 = get_layout_node(layout_manager, 6);

        if component_6 != 0 {
            load_text_from_mal(
                this_ptr as u64,
                0.0,
                *((component_6 + 0x20) as *const u64),
                &text_pane,
                &text_id,
                0,
            );
        }

        this.anim_frame_counter = 0;

        let get_anim_length: extern "C" fn(u64) -> u16 = std::mem::transmute(text_base + 0x243a60);

        // inlined FUN_71020a604c
        for i in 0..9i32 {
            let comp = get_layout_node(layout_manager, i);
            if comp != 0 {
                let scene_manager = *((comp + 0x20) as *const u64);
                if scene_manager != 0 {
                    let anim_ctrl = *((scene_manager + 0x70) as *const u64);
                    if anim_ctrl != 0 {
                        let length = get_anim_length(anim_ctrl);
                        *((anim_ctrl + 0x20) as *mut f32) = (length as f32) * 0.5;
                    }
                }
            }
        }

        ((*this.base.base_idisposer.vtable).set_component_closed_state)(this_ptr as u64, 7, 0);

        if this.unk_4c8 == 0 {
            if this.pending_action_id == -1 {
                let fun_5e44: extern "C" fn(u64) = std::mem::transmute(text_base + 0x20a5e44);
                fun_5e44(this_ptr as u64);

                ((*this.base.base_idisposer.vtable).set_component_closed_state)(
                    this_ptr as u64,
                    0,
                    this.unk_48a[0] as u32,
                );
                ((*this.base.base_idisposer.vtable).set_component_closed_state)(
                    this_ptr as u64,
                    6,
                    this.unk_48a[6] as u32,
                );
            }

            this.unk_4c8 = 0;
        }

        let priorities = [2, 1, 0, 6, 5, 4, 7, 3, 8];
        let mut final_id = -1i32;
        for &id in &priorities {
            let comp = get_layout_node(layout_manager, id);
            if comp != 0 {
                let flags = *((comp + 0x40) as *const u8);
                if (flags >> 2 & 1) != 0 {
                    final_id = id;
                    break;
                }
            }
        }

        if final_id != -1 {
            let set_initial_focus: extern "C" fn(u64, i32) =
                std::mem::transmute(text_base + 0x8a8cf8);
            set_initial_focus(this_ptr as u64, final_id);
        }
    }
}

unsafe fn get_layout_node(list_manager: u64, target_id: i32) -> u64 {
    if list_manager == 0 {
        return 0;
    }

    let mut current_node = *((list_manager + 0x10) as *const u64);

    let tail_sentinel = list_manager + 8;

    while current_node != tail_sentinel && current_node != 0 {
        let current_id = *((current_node + 0x3c) as *const i32);

        if current_id == target_id {
            return current_node - 8;
        }

        current_node = *((current_node + 8) as *const u64);
    }

    0
}

extern "C" fn mod_menu_app_close_start(this: *mut ScreenModMenu) {
    println!("[Mod] appCloseStart");
    let this_ptr = this;
    let this = unsafe { &mut *this };

    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let find_anim: extern "C" fn(u64, *const u8) -> u64 =
            std::mem::transmute(text_base + 0x8b18a4);

        let layout_panes = this.base.layout_panes as u64;
        let anim = find_anim(layout_panes, b"InAfter\0".as_ptr());

        if anim != 0 {
            let speed = *((anim + 0x50) as *const f32);

            if speed != 0.0 {
                let vtable = *(anim as *const *const u64);
                let play: extern "C" fn(u64) = std::mem::transmute(*vtable.add(0xd8 / 8));
                play(anim);
            }
        }
    }
}

extern "C" fn mod_menu_app_close_end(this: u64) {
    println!("[Mod] appCloseEnd");
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let fn1: extern "C" fn(u64) = std::mem::transmute(text_base + 0x215b4f0);
        let fn2: extern "C" fn(u64) = std::mem::transmute(text_base + 0x215b5b0);
        fn1(this);
        fn2(this);

        *((this + 0x489) as *mut u8) = 0;
    }
}

extern "C" fn mod_menu_btn_0(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 0 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_1(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 1 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_2(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 2 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_3(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 3 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_4(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 4 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_5(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 5 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_6(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 6 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_7(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 7 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn mod_menu_btn_8(this: u64, button_ptr: u64) {
    println!("[ModMenu] Button 8 pressed, button_ptr={button_ptr:#X}");
}

extern "C" fn my_button_click_handler(this: u64, button_ptr: u64) {
    let button_id = unsafe { *((button_ptr + 0x44) as *const i32) };

    match button_id {
        0 => mod_menu_btn_2(this, button_ptr),
        1 => mod_menu_btn_1(this, button_ptr),
        2 => mod_menu_btn_0(this, button_ptr),
        3 => mod_menu_btn_7(this, button_ptr),
        4 => mod_menu_btn_5(this, button_ptr),
        5 => mod_menu_btn_4(this, button_ptr),
        6 => mod_menu_btn_3(this, button_ptr),
        7 => mod_menu_btn_6(this, button_ptr),
        8 => mod_menu_btn_8(this, button_ptr),
        _ => {}
    }
}

extern "C" fn mod_menu_app_do_update(this: *mut ScreenModMenu) {
    let this_ptr = this;
    let this = unsafe { &mut *this };

    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let get_layout_node_with_ptr: extern "C" fn(*mut LayoutManager, i32) -> *const u8 =
            std::mem::transmute(text_base + 0x8040b0);
        let fun_7100243a60: extern "C" fn(*const u8) -> i32 =
            std::mem::transmute(text_base + 0x243a60);
        let fun_710215b208: extern "C" fn(*mut ScreenModMenu, i32) -> f32 =
            std::mem::transmute(text_base + 0x215b208);
        let is_node_clicked: extern "C" fn(*const u8) -> bool =
            std::mem::transmute(text_base + 0x230a754);
        let fun_710090d3c0: extern "C" fn(u32, *mut BaseScreen<ModMenuVTable>, i32) =
            std::mem::transmute(text_base + 0x90d3c0);
        let fun_71008cb6bc: extern "C" fn(u64, i32) -> u32 =
            std::mem::transmute(text_base + 0x8cb6bc);
        let fun_7102309cfc: extern "C" fn(u64) -> bool = std::mem::transmute(text_base + 0x2309cfc);
        let fun_710215b490: extern "C" fn(u64, i32) -> u32 =
            std::mem::transmute(text_base + 0x215b490);

        let fun_710231d85c: extern "C" fn(*mut ScreenModMenu) -> bool =
            std::mem::transmute(text_base + 0x231d85c);
        let fun_710230b308: extern "C" fn(u64) -> u32 = std::mem::transmute(text_base + 0x230b308);
        let get_ui_controller: extern "C" fn() -> *const u8 =
            std::mem::transmute(text_base + 0x6911c0);
        let fun_710215ad58: extern "C" fn(*mut ScreenModMenu) =
            std::mem::transmute(text_base + 0x215ad58);

        let g_system_ptr = *((text_base + 0x396d1c0) as *const *mut u64);

        let i_var_8 = if g_system_ptr.is_null() {
            2
        } else {
            let fun_7100264fa0: extern "C" fn(*mut u64) -> i32 =
                std::mem::transmute(text_base + 0x264fa0);

            fun_7100264fa0(g_system_ptr)
        };

        this.anim_frame_counter += i_var_8;

        for i in 0..9usize {
            unsafe {
                let node = get_layout_node_with_ptr(this.base.layout_manager, i as i32);

                if !node.is_null() {
                    let ptr1 = *((node as *const u8).add(0x20) as *const *const u8);

                    if !ptr1.is_null() {
                        let l_var_17 = *(ptr1.add(0x70) as *const *const u8);

                        if !l_var_17.is_null() {
                            let u_var_7 = fun_7100243a60(l_var_17);

                            let bit_check = (0x6f >> (i & 0x3f)) & 1;

                            if bit_check != 0 && u_var_7 != 0 {
                                let button_id = ENTRIES[i].id;

                                let f_var_18 = fun_710215b208(this_ptr, button_id);

                                let should_process = if i == 2 {
                                    f_var_18 == 3.0
                                } else {
                                    f_var_18 == 1.0
                                };

                                if should_process {
                                    let float_val = *(l_var_17.add(0x20) as *const f32);

                                    if (u_var_7 as f32) <= float_val + (i_var_8 as f32) {
                                        let pad_40_byte = *(node as *const u8).add(0x40);
                                        if (pad_40_byte >> 4) & 1 != 0 {
                                            is_node_clicked(node);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if this.pending_action_id >= 0 && (this.base.unk_11e == 2 && this.base.close_intent > -1) {
            let timer = this.action_timer;

            let dt_val = *((g_system_ptr as *const u8).add(0x10) as *const i32);

            let mut should_fire = false;

            if timer < 0 {
                let next_timer = timer + dt_val;
                if next_timer >= 0 {
                    this.action_timer = 0;
                    should_fire = true;
                } else {
                    this.action_timer = next_timer;
                }
            } else if timer == 0 {
                should_fire = true;
            } else {
                let next_timer = timer - dt_val;
                if next_timer <= 0 {
                    this.action_timer = 0;
                    should_fire = true;
                } else {
                    this.action_timer = next_timer;
                }
            }

            if should_fire {
                let fun_71020a5c90: extern "C" fn(*mut u64, u64) =
                    std::mem::transmute(text_base + 0x20a5c90);
                fun_71020a5c90(this_ptr as *mut u64, this.pending_action_id as u64);
                this.pending_action_id = -1;
            }
        }

        let mut input_free = true;
        for i in 0..9 {
            let chunk = *this.navigation_map.data_buffer.add(i);
            let i_var_8 = *((chunk as *const u8).add(0xc) as *const i32);

            if i_var_8 >= 0 {
                let node = get_layout_node_with_ptr(this.base.layout_manager, i as i32);

                if !node.is_null() {
                    let u_var_9 = fun_71008cb6bc(this_ptr as u64, i_var_8);

                    if (u_var_9 & 1) != 0 {
                        let f_var_18 = fun_710215b208(this_ptr, i_var_8);
                        let b_var_6 = fun_7102309cfc(node as u64);

                        if f_var_18 >= 2.0 && b_var_6 {
                            let vtable = *(node as *const *const u64);
                            let vfunc_0x128: extern "C" fn(*const u8, i32) =
                                std::mem::transmute(*vtable.add(0x128 / 8));

                            vfunc_0x128(node, 0);
                        }

                        input_free = false;
                        break;
                    } else {
                        let u_var_9_alt = fun_710215b490(this_ptr as u64, i_var_8);

                        if (u_var_9_alt & 1) != 0 {
                            for j in 0..=8 {
                                let sub_node =
                                    get_layout_node_with_ptr(this.base.layout_manager, j);
                                if !sub_node.is_null() {
                                    let sub_vtable = *(sub_node as *const *const u64);
                                    let vfunc_0x60: extern "C" fn(*const u8, i32) =
                                        std::mem::transmute(*sub_vtable.add(0x60 / 8));

                                    vfunc_0x60(sub_node, 1);
                                }
                            }

                            ((*this.base.base_idisposer.vtable).set_component_closed_state)(
                                this_ptr as u64,
                                i as i32,
                                1,
                            );
                        }
                    }
                }
            }
        }

        let mut b_var_5 = false;
        if this.pending_action_id == -1 {
            b_var_5 = input_free;
        }

        this.is_input_enabled = b_var_5;
        input_free = this.base.unk_11e == 2 && this.base.close_intent > -1;

        // closes when X is clicked, as that is the button that it was spawned with
        if input_free
            && fun_710231d85c(this_ptr)
            && (fun_710230b308(this.base.layout_manager as u64) & 1) == 0
            && this.is_input_enabled
            && this.transition_state == 0
        {
            let ui_controller = get_ui_controller();

            if !ui_controller.is_null() {
                let btn_state = *(ui_controller.add(8) as *const u8);

                if (btn_state >> 3) & 1 != 0 {
                    fun_710215ad58(this_ptr);
                    ((*this.base.base_idisposer.vtable).close)(this_ptr as u64, -1);
                }
            }
        }

        let g_clock_manager_ptr = *((text_base + 0x32cbdc0) as *const *const u8);
        let clock_manager = g_clock_manager_ptr;

        if !clock_manager.is_null() {
            let minute = *(clock_manager.add(0x144) as *const u32);
            let hour = *(clock_manager.add(0x140) as *const u32);
            let day = *(clock_manager.add(0x138) as *const u32);

            let current_minutes = minute + (hour * 60);

            let needs_update = this.unk_4a8 == 0
                || this.unk_4a4 != current_minutes
                || this.unk_4a0 == 0
                || this.unk_49c != day;

            if needs_update {
                let ptr_dat = text_base + 0x32cb2e8;
                let pu_var_13 = **(ptr_dat as *const *mut *mut u32);

                if !pu_var_13.is_null() {
                    let u_var_1 = *pu_var_13.add(3);
                    let mut u_var_9 = *pu_var_13 ^ (*pu_var_13 << 11);

                    *pu_var_13 = *pu_var_13.add(1);
                    *pu_var_13.add(1) = *pu_var_13.add(2);

                    u_var_9 = (u_var_9 >> 8) ^ (u_var_1 >> 19) ^ u_var_9 ^ u_var_1;
                    *pu_var_13.add(2) = u_var_1;
                    *pu_var_13.add(3) = u_var_9;
                }

                if this.unk_4a8 == 0 {
                    this.unk_4a8 = 1;
                }
                this.unk_4a4 = current_minutes;

                if this.unk_4a0 == 0 {
                    this.unk_4a0 = 1;
                }
                this.unk_49c = day;
            }
        }

        let is_ready = this.base.unk_11e == 2 && this.base.close_intent > -1;

        if is_ready && !fun_710231d85c(this_ptr) {
            if this.unk_48a[5] {
                let f_var_17 = fun_710215b208(this_ptr, 4);
                if f_var_17 > 0.0 {
                    fun_710090d3c0(0, core::ptr::addr_of_mut!(this.base), 4);
                }
            }
        }
    }
}

/*
extern "C" fn mod_menu_app_do_update(this: u64) {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let list_manager = *((this + 0x30) as *const u64);

        let mut focus_id: i32 = -1;
        let focus_mgr = *((text_base + 0x32cb360) as *const u64);

        if focus_mgr != 0 {
            let get_focus: extern "C" fn() -> i32 = std::mem::transmute(text_base + 0x1ccff80);
            focus_id = get_focus();
        }

        let timer_src = *((text_base + 0x396d1c0) as *const u64);

        let step = if timer_src == 0 {
            2
        } else {
            let f: extern "C" fn(u64) -> i32 = std::mem::transmute(text_base + 0x264fa0);
            f(timer_src)
        };

        *((this + 0x4cc) as *mut i32) += step;

        let get_anim_len: extern "C" fn(u64) -> u16 = std::mem::transmute(text_base + 0x243a60);

        let get_anim_step: extern "C" fn(u64, i32) -> f32 =
            std::mem::transmute(text_base + 0x215b208);

        let is_clicked: extern "C" fn(u64) -> bool = std::mem::transmute(text_base + 0x230a754);

        let mut interacted = false;

        for i in 0..9 {
            let node = get_layout_node(list_manager, i);
            if node == 0 {
                continue;
            }

            let layout = *((node + 0x20) as *const u64);
            if layout == 0 {
                continue;
            }

            let anim = *((layout + 0x70) as *const u64);
            if anim == 0 {
                continue;
            }

            let anim_len = get_anim_len(anim) as f32;
            if anim_len == 0.0 {
                continue;
            }

            let btn_id = ENTRIES[i as usize].id;
            let step_val = get_anim_step(this, btn_id);

            let trigger = if i == 2 { 3.0 } else { 1.0 };
            if step_val != trigger {
                continue;
            }

            if (*((node + 0x40) as *const u8) >> 4 & 1) != 0 {
                if is_clicked(node) {
                    interacted = true;
                }
            }

            let frame = *((anim + 0x20) as *const f32);

            if anim_len <= frame + step as f32 {
                if focus_id > 0 {
                    let t = *((this + 0x4cc) as *const i32) as f32 / anim_len;

                    let frac_flag = if t != t.floor() {
                        if t >= 0.0 {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    if focus_id <= (frac_flag + t as i32) {
                        let flag = if i == 2 { 0x40800000 } else { 0x40000000 };

                        let dispatch: extern "C" fn(u32, u64, u32) =
                            std::mem::transmute(text_base + 0x90d3c0);

                        dispatch(flag, this, btn_id as u32);
                    }
                }
            }
        }

        let pending = *((this + 0x494) as *const i32);

        if pending != -1 {
            let mut counter = *((this + 0x498) as *const i32);

            let delta_src = *((text_base + 0x396d1c0) as *const u64);
            let delta = *((delta_src + 0x10) as *const i32);

            if counter >= 0 {
                counter -= delta;
                if counter <= 0 {
                    counter = 0;
                }
            } else {
                counter += delta;
                if counter >= 0 {
                    counter = 0;
                }
            }

            *((this + 0x498) as *mut i32) = counter;

            if counter == 0 {
                let fire: extern "C" fn(u64, u64) = std::mem::transmute(text_base + 0x20a5c90);

                fire(this, pending as u64);
                *((this + 0x494) as *mut i32) = -1;
            }
        }

        let nav_array = *((this + 0x350) as *const u64);
        let mut allow_input = 1u8;

        for i in 0..9 {
            let slot = *((nav_array + (i * 8) as u64) as *const u64);
            let id = *((slot + 0xc) as *const i32);
            if id < 0 {
                continue;
            }

            let node = get_layout_node(list_manager, id);
            if node == 0 {
                continue;
            }

            let nav_to: extern "C" fn(u64, i32) -> bool = std::mem::transmute(text_base + 0x8cb6bc);

            if nav_to(this, id) {
                let step_val = get_anim_step(this, id);

                if step_val >= 2.0 {
                    let can_defocus: extern "C" fn(u64) -> bool =
                        std::mem::transmute(text_base + 0x2309cfc);

                    if can_defocus(node) {
                        let vt = *(node as *const *const u64);
                        let unfocus: extern "C" fn(u64, i32) =
                            std::mem::transmute(*vt.add(0x128 / 8));
                        unfocus(node, 0);
                    }
                }

                allow_input = 0;
                break;
            }

            let alt: extern "C" fn(u64, i32) -> bool = std::mem::transmute(text_base + 0x215b490);

            if alt(this, id) {
                for j in 0..9 {
                    let n = get_layout_node(list_manager, j);
                    if n != 0 {
                        let vt = *(n as *const *const u64);
                        let set_focus: extern "C" fn(u64, bool) =
                            std::mem::transmute(*vt.add(0x60 / 8));
                        set_focus(n, true);
                    }
                }

                let vt = *(this as *const *const u64);
                let fn_488: extern "C" fn(u64, i32, i32) = std::mem::transmute(*vt.add(0x488 / 8));

                fn_488(this, i, 1);

                allow_input = 0;
                break;
            }
        }

        *((this + 0x488) as *mut u8) = if *((this + 0x494) as *const i32) == -1 {
            allow_input
        } else {
            0
        };

        let is_active: extern "C" fn(u64) -> bool = std::mem::transmute(text_base + 0x8145e8);

        let is_ready: extern "C" fn(u64) -> bool = std::mem::transmute(text_base + 0x231d85c);

        let nav_block: extern "C" fn(u64) -> u32 = std::mem::transmute(text_base + 0x230b308);

        let get_ui: extern "C" fn() -> *const u64 = std::mem::transmute(text_base + 0x6911c0);

        if is_active(this)
            && is_ready(this)
            && (nav_block(list_manager) & 1) == 0
            && *((this + 0x488) as *const u8) != 0
            && *((this + 0x489) as *const u8) == 0
        {
            let ui = get_ui();

            if !ui.is_null() && ((*ui.add(1) >> 1) & 1) != 0 {
                let play_close: extern "C" fn(u64) = std::mem::transmute(text_base + 0x215ad58);
                play_close(this);

                let vt = *(this as *const *const u64);
                let close: extern "C" fn(u64, u32) = std::mem::transmute(*vt.add(0x30 / 8));

                close(this, 0xffffffff);
            }
        }
    }
}
 */

extern "C" fn mod_menu_destructor1(this: u64) {
    println!("[Mod] Destructor1");
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let inter_base = MOD_MENU_INTERMEDIATE_VTABLE.as_ptr() as u64;

        *(this as *mut u64) = inter_base + 0x10;
        *((this + 0x25 * 8) as *mut u64) = inter_base + 0x4C0;

        let base_dtor: extern "C" fn(u64) = std::mem::transmute(text_base + 0x8a7878);
        base_dtor(this);
    }
}

extern "C" fn mod_menu_destructor2(this: u64) {
    println!("[Mod] Destructor2");
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        mod_menu_destructor1(this);

        let op_delete: extern "C" fn(u64) = std::mem::transmute(text_base + 0x3e510);
        op_delete(this);
    }
}

static mut MOD_MENU_TYPE_INFO_GUARD: u32 = 0;
pub static mut MOD_MENU_TYPE_INFO_PTR: u64 = 0;
static mut MOD_MENU_TYPE_INFO_DATA: [u64; 4] = [0u64; 4];

pub unsafe fn initialize_type_info(text_base: u64) {
    let base_type_info = text_base + 0x32cd380;
    MOD_MENU_TYPE_INFO_DATA[0] = base_type_info + 0x10;
    MOD_MENU_TYPE_INFO_PTR = MOD_MENU_TYPE_INFO_DATA.as_ptr() as u64;
}

extern "C" fn mod_menu_check_rtti(this: u64, target_type: u64) -> u64 {
    unsafe {
        let my_type = &raw const MOD_MENU_TYPE_INFO_PTR as u64;
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let base_eui_type = *((text_base + 0x32cd388) as *const u64);
        let base_type2 = *((text_base + 0x32cd3a0) as *const u64);
        let root_type = *((text_base + 0x32cb110) as *const u64);

        if target_type == my_type
            || target_type == base_eui_type
            || target_type == base_type2
            || target_type == root_type
        {
            return this;
        }
        0
    }
}

extern "C" fn mod_menu_get_rtti() -> u64 {
    unsafe { &raw const MOD_MENU_TYPE_INFO_PTR as u64 }
}

extern "C" fn mod_menu_is_enable_control(this: u64) -> u64 {
    unsafe {
        if *((this + 0x488) as *const u8) == 0 {
            return 0;
        }

        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let inner: extern "C" fn(u64) -> u64 = std::mem::transmute(text_base + 0x244518);

        inner(this)
    }
}

extern "C" fn mod_menu_unk_0x3e8(this: u64) {
    unsafe {
        if *((this + 0x4c9) as *const u8) == 0 {
            return;
        }

        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let update_layout_scales: extern "C" fn(u64) = std::mem::transmute(text_base + 0x20a604c);
        update_layout_scales(this);

        *((this + 0x4c9) as *mut u8) = 0;
    }
}

extern "C" fn mod_menu_unk_0x490(this: *mut ScreenModMenu, focus_flags: u32) {
    println!("[Mod] unk_0x490");
    let this_ptr = this;
    let this = unsafe { &mut *this };

    unsafe {
        if 0 < this.navigation_map.count {
            let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

            for i in 0..this.navigation_map.count {
                ((*this.base.base_idisposer.vtable).set_component_closed_state)(
                    this_ptr as u64,
                    i,
                    (focus_flags & 1),
                );
            }
        }
    }
}

#[repr(C)]
pub struct MenuButtonMap {
    name: *const u8,
    id: i32,
    neighbor: i32,
}

unsafe impl Sync for MenuButtonMap {}
unsafe impl Send for MenuButtonMap {}

static ENTRIES: [MenuButtonMap; 9] = [
    MenuButtonMap {
        name: "L_ModBtn_02\0".as_ptr(),
        id: 0,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_01\0".as_ptr(),
        id: 1,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_00\0".as_ptr(),
        id: 2,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_07\0".as_ptr(),
        id: 3,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_05\0".as_ptr(),
        id: 4,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_04\0".as_ptr(),
        id: 5,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_03\0".as_ptr(),
        id: 6,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_06\0".as_ptr(),
        id: 7,
        neighbor: -1,
    },
    MenuButtonMap {
        name: "L_ModBtn_08\0".as_ptr(),
        id: 8,
        neighbor: -1,
    },
];

extern "C" fn mod_menu_unk_0x498(this: *mut ScreenModMenu) {
    let this_ptr = this;
    let this = unsafe { &mut *this };

    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        for i in 0..9 {
            if this.navigation_map.count < this.navigation_map.capacity {
                let chunk = this.navigation_map.free_list_head;

                if !chunk.is_null() {
                    this.navigation_map.free_list_head = (*chunk) as *mut u64;
                }

                *chunk = ENTRIES[i].name as u64;
                let packed_ids = (ENTRIES[i].id as u64) | ((ENTRIES[i].neighbor as u64) << 32);
                *(chunk.add(1)) = packed_ids;

                if this.navigation_map.count < this.navigation_map.capacity {
                    unsafe {
                        let idx = this.navigation_map.count as usize;

                        *this.navigation_map.data_buffer.add(idx) = chunk as u64;
                    }
                    this.navigation_map.count += 1;
                }
            }

            let layout_manager = unsafe { &mut *this.base.layout_manager };
            let layout_manager = unsafe { &mut *this.base.layout_manager };

            let sentinel_ptr = core::ptr::addr_of!(layout_manager.start_end) as *mut ListNode;
            let mut current_ptr = layout_manager.start_end.next;

            if current_ptr != sentinel_ptr && !current_ptr.is_null() {
                'search: loop {
                    unsafe {
                        let raw_node = current_ptr as *mut u8;

                        let pane_name_ptr = *(raw_node.add(0x10) as *const *const u8);

                        if !pane_name_ptr.is_null() {
                            let mut char_idx = 0;
                            loop {
                                let expected_char = *ENTRIES[i].name.add(char_idx);
                                let actual_char = *pane_name_ptr.add(char_idx);

                                if expected_char != actual_char {
                                    break;
                                }

                                if char_idx > 0x3e || expected_char == b'\0' {
                                    let out_idx_addr = raw_node.add(0x3c) as *mut i32;
                                    *out_idx_addr = i as i32;
                                    break 'search;
                                }
                                char_idx += 1;
                            }
                        }

                        current_ptr = (*current_ptr).next;
                    }

                    if current_ptr == sentinel_ptr {
                        break;
                    }
                }
            }
        }

        let link_hardware: extern "C" fn(u64, u32, i32, i32) =
            std::mem::transmute(text_base + 0x84682c);
        let setup_hardware: extern "C" fn(u64, u32, i32, i32, u8) =
            std::mem::transmute(text_base + 0x8b847c);

        /*
        link_hardware(this_ptr as u64, 0, 2, 8);
        setup_hardware(this_ptr as u64, 0, 2, 8, 0);
        */

        let visual_order: [i32; 9] = [2, 1, 0, 6, 5, 4, 7, 3, 8];
        for i in 0..9usize {
            let current = visual_order[i];
            let next = visual_order[(i + 1) % 9];
            link_hardware(this_ptr as u64, 1, current, next);
        }

        setup_hardware(this_ptr as u64, 1, 2, 8, 0);
    }
}

/*extern "C" fn mod_menu_unk_0x498(this: *mut ScreenModMenu) {
    let this_ptr = this;
    let this = unsafe { &mut *this };

    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        for i in 0..9 {
            let current_count = *((this + 0x348) as *const i32);
            let max_count = *((this + 0x34c) as *const i32);

            if current_count < max_count {
                let pool_head = (this + 0x358) as *mut *mut u64;
                let chunk = *pool_head;

                if !chunk.is_null() {
                    *pool_head = *chunk as *mut u64;
                }

                *chunk = ENTRIES[i].name as u64;
                let packed_ids = (ENTRIES[i].id as u64) | ((ENTRIES[i].neighbor as u64) << 32);
                *(chunk.add(1)) = packed_ids;

                let array_base = *((this + 0x350) as *const u64);
                *((array_base + (current_count as u64 * 8)) as *mut u64) = chunk as u64;
                *((this + 0x348) as *mut i32) = current_count + 1;
            }

            let list_manager = *((this + 0x30) as *const u64);
            let list_tail = list_manager + 8;
            let mut current_node = *((list_manager + 0x10) as *const u64);

            if current_node != list_tail {
                'search: while current_node != list_tail && current_node != 0 {
                    let pane_name_ptr = *((current_node + 0x10) as *const *const u8);

                    if !pane_name_ptr.is_null() {
                        let mut char_idx = 0;
                        loop {
                            let expected_char = *ENTRIES[i].name.add(char_idx);
                            let actual_char = *pane_name_ptr.add(char_idx);

                            if expected_char != actual_char {
                                break;
                            }

                            if char_idx > 0x3e || expected_char == b'\0' {
                                *((current_node + 0x3c) as *mut i32) = i as i32;
                                break 'search;
                            }
                            char_idx += 1;
                        }
                    }
                    current_node = *((current_node + 8) as *const u64);
                }
            }
        }

        let link_hardware: extern "C" fn(u64, u32, i32, i32) =
            std::mem::transmute(text_base + 0x84682c);
        let setup_hardware: extern "C" fn(u64, u32, i32, i32, u8) =
            std::mem::transmute(text_base + 0x8b847c);

        let visual_order: [i32; 9] = [2, 1, 0, 6, 5, 4, 7, 3, 8];
        for i in 0..9usize {
            let current = visual_order[i];
            let next = visual_order[(i + 1) % 9];
            link_hardware(this_ptr as u64, 1, current, next);
        }

        setup_hardware(this_ptr as u64, 1, 2, 8, 0);
    }
}*/

#[repr(C)]
pub struct LayoutManager {
    pub vtable: *const c_void,
    pub start_end: ListNode,
    pub count: i32,
    pub offset: i32,
}

#[repr(C)]
pub struct SomeKindOfListMap {
    pub count: i32,
    pub capacity: i32,
    pub data_buffer: *mut u64,
    pub free_list_head: *mut u64,
    pub free_list_tail: *mut u64,
    pub nodes: [ListNode; 12],
    pub objects: [*const c_void; 12],
}

#[repr(C, packed)]
pub struct BaseScreen<V> {
    pub base_idisposer: IDisposer<V>,
    pub scene_manager: *mut c_void,
    pub layout_panes: *mut c_void,
    pub layout_manager: *mut LayoutManager,
    pub render_node: ListNode,
    pub update_node: ListNode,
    pub pad_58: [u8; 24],
    pub event_node: ListNode,
    pub child_list_1: OffsetList<c_void>,
    pub node_4: ListNode,
    pub unk_a8: u32,
    pub screen_id: u32,
    pub node_5: ListNode,
    pub unk_c0: u64,
    pub child_list_2: OffsetList<c_void>,
    pub parent_heap: *const c_void,
    pub unk_e8: i32,
    pub pad_ec: u32,
    pub ui_allocator: *const c_void,
    pub currently_focused_node: *const c_void,
    pub pad_100: [u8; 24],
    pub camera_fov: f32,
    pub input_mode: u8,
    pub close_intent: i8,
    pub unk_11e: u8,
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

#[repr(C)]
pub struct ScreenModMenu {
    pub base: BaseScreen<ModMenuVTable>,
    pub navigation_map: SomeKindOfListMap,
    pub is_input_enabled: bool,
    pub transition_state: u8,
    pub unk_48a: [bool; 8],
    pub is_initialized: bool,

    pub pad_494: u8,

    pub pending_action_id: i32,
    pub action_timer: i32,

    pub unk_49c: u32,
    pub unk_4a0: u32,
    pub unk_4a4: u32,
    pub unk_4a8: u32,
    pub pad_4a9: [u8; 2],

    pub anim_in_from_bg: *const c_void,
    pub anim_out_to_bg: *const c_void,
    pub anim_short_in: *const c_void,

    pub unk_4c8: u8,
    pub pad_4c9: [u8; 3],

    pub anim_frame_counter: i32,
}

const _: () = assert!(core::mem::size_of::<BaseScreen<c_void>>() == 0x348);
const _: () = assert!(core::mem::size_of::<ScreenModMenu>() == 0x4D0);
