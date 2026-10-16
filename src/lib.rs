mod eui;
mod eui_controller;
mod fsm_ext;
mod game;
mod mod_menu;
mod save;
mod save_file;
mod sead;
mod ui_framework;

use std::{
    ffi::{c_char, c_void},
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use skyline::{hooks::InlineCtx, nn::fs::OpenFile};

use crate::{
    eui::{screen_manager::BaseScreen, DrawState},
    eui_controller::EuiController,
    fsm_ext::GAMEPLAY_CONTROLLER,
    mod_menu::{ModMenuVTable, MOD_MENU_HASH},
    save::player::PlayerManager,
    sead::prim::SafeString,
    ui_framework::ButtonState,
};

pub fn test_spawn_mod_menu(_btn: &mut ButtonState, _ctrl: EuiController) {
    /*unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let scene_mgr_pp = (text_base + 0x32cd3f8) as *const *const u64;
        let scene_mgr_p = *scene_mgr_pp;
        let scene_mgr = *scene_mgr_p;

        println!("[Mod] scene_mgr = {scene_mgr:#X}");

        let find_by_hash: extern "C" fn(u64, u32) -> u64 =
            std::mem::transmute(text_base + 0x7c5b74);

        let wrapper = find_by_hash(scene_mgr, MOD_MENU_HASH);
        if wrapper == 0 {
            println!("[Mod] FindByMurmurHash returned null");
            return;
        }
        println!("[Mod] wrapper = {wrapper:#X}");

        let wrapper_vtable = *(wrapper as *const *const u64);
        let get_screen: extern "C" fn(u64, u64) -> u64 =
            std::mem::transmute(*wrapper_vtable.add(2));

        let type_ptr = &raw const MOD_MENU_TYPE_INFO_PTR as u64;
        let screen = get_screen(wrapper, type_ptr);
        println!("[Mod] screen = {screen:#X}");
        if screen == 0 {
            println!("[Mod] type check failed");
            return;
        }

        let open_fn: extern "C" fn(u64, i32) = std::mem::transmute(text_base + 0x8d9654);
        open_fn(screen, 1);
        println!("[Mod] open called");
    }*/

    let root_ptr = GAMEPLAY_CONTROLLER.load(Ordering::SeqCst);

    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let state_machine_ptr = root_ptr + 0x20;

        if state_machine_ptr == 0 {
            println!("[Mod] State machine pointer is null");
            return;
        }

        println!("[Mod] Gameplay controller: {root_ptr:#X}, State machine {state_machine_ptr:#X}");

        let change_state_fn: extern "C" fn(u64, u32) = std::mem::transmute(text_base + 0x445bc8);

        let state_id = 0x88;

        println!(
            "[Tomodachi] Warp Initiated w/ base {text_base:#X}: {:#X} -> State {state_id:#X}",
            root_ptr
        );

        change_state_fn(state_machine_ptr, state_id);
    }
}

extern "C" fn state_88_enter(context: u64) {
    println!("custom state enter");
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let find_by_hash: extern "C" fn(u64, u32) -> *mut BaseScreen<ModMenuVTable> =
            std::mem::transmute(text_base + 0x7c5b74);

        let scene_mgr = **((text_base + 0x32cd3f8) as *const *const u64);

        let mod_menu = find_by_hash(scene_mgr, MOD_MENU_HASH);
        let island_menu = find_by_hash(scene_mgr, 0x51861076);

        if !island_menu.is_null() {
            // let's just assume for now that it did get the proper one so we skip RTTI guard
            (*island_menu).close(DrawState::Closing);
        }

        if !mod_menu.is_null() {
            (*mod_menu).open(DrawState::Opening);
        }
    }
}

extern "C" fn state_88_update(context: u64) {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let manager_ptr_ptr = (context + 0xa8) as *const *const u64;

        if !manager_ptr_ptr.is_null() {
            let manager = *manager_ptr_ptr;

            if !manager.is_null() {
                let ui_update_fn: extern "C" fn(u64) = std::mem::transmute(text_base + 0x1fa49b4);

                ui_update_fn(manager as u64);

                let find_by_hash: extern "C" fn(u64, u32) -> *mut BaseScreen<ModMenuVTable> =
                    std::mem::transmute(text_base + 0x7c5b74);
                let scene_mgr = **((text_base + 0x32cd3f8) as *const *const u64);
                let mod_menu = find_by_hash(scene_mgr, MOD_MENU_HASH);
                let check_validator: extern "C" fn(*mut BaseScreen<_>) -> u32 =
                    std::mem::transmute(text_base + 0x395b8c);

                if (check_validator(mod_menu) & 1) != 0 {
                    let exit_wrapper_fn: extern "C" fn(u64, u64, u8) =
                        std::mem::transmute(text_base + 0x8aec48);

                    let zero_vec_ptr = text_base + 0x32cb150;
                    exit_wrapper_fn(context, zero_vec_ptr, 1);
                }
            }
        }
    }
}

extern "C" fn state_88_exit(context: u64) {
    println!("custom state exit");
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let find_by_hash: extern "C" fn(u64, u32) -> *mut BaseScreen<ModMenuVTable> =
            std::mem::transmute(text_base + 0x7c5b74);

        let scene_mgr = **((text_base + 0x32cd3f8) as *const *const u64);

        let mod_menu = find_by_hash(scene_mgr, MOD_MENU_HASH);
        let island_menu = find_by_hash(scene_mgr, 0x51861076);

        if !mod_menu.is_null() {
            (*mod_menu).close(DrawState::Closing);
        }

        if !island_menu.is_null() {
            (*island_menu).open(DrawState::Opening);
        }
    }
}

pub fn give_money(money_count: u32) {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let global_manager_ptr = *((text_base + 0x396D0F0) as *const u64);

        if global_manager_ptr == 0 {
            println!("[Mod] Global Manager is null.");
            return;
        }

        let player_state = *((global_manager_ptr + 0x88) as *const u64);

        if player_state == 0 {
            println!("[Mod] Player State is null.");
            return;
        }

        let wallet_obj = player_state + 0xC20;

        let vtable = *(wallet_obj as *const u64);

        let get_money_ptr: extern "C" fn(u64) -> *mut u32 =
            std::mem::transmute(*((vtable + 0x20) as *const u64));

        let money_address = get_money_ptr(wallet_obj);

        if !money_address.is_null() {
            let current_money = *money_address;
            println!("[Mod] Current Money: {current_money}");

            *money_address = 99999;
            println!("[Mod] Injected 99999 into the wallet!");
        }
    }
}

static SCREEN_HEAP: AtomicU64 = AtomicU64::new(0);

#[skyline::hook(offset = 0x5ea080)]
unsafe fn save_write_hook(save_manager: *const u8, stream: *const u8) -> bool {
    let result = call_original!(save_manager, stream);
    println!(
        "[saveProperty::deserializedDataStreamRead] called with manager={:p} stream={:p} result={}",
        save_manager, stream, result
    );
    result
}

#[skyline::hook(offset = 0x250798)]
unsafe fn sharc_archive_res_get_file_impl(
    this: *const u8,
    file_path: *const SafeString,
    file_info: *const u8,
) -> *const c_void {
    if !file_path.is_null() {
        let file_path = unsafe { &*file_path };

        println!("[sead::SharcArhiveRes::getFileImpl_] searching for {file_path:?}");
    }

    call_original!(this, file_path, file_info)
}

#[skyline::hook(offset = 0x218e7ac)]
unsafe fn sharc_archive_res_try_get_file_path_impl(
    this: *const u8,
    out_file_path: *const SafeString,
    entry_id: i32,
) -> bool {
    let result = call_original!(this, out_file_path, entry_id);

    if !out_file_path.is_null() {
        let file_path = unsafe { &*out_file_path };

        println!("[sead::SharcArhiveRes::tryGetFilePathImpl_] searching for {file_path:?}");
    }

    result
}

#[skyline::hook(offset = 0x2508c0)]
unsafe fn sharc_archive_res_convert_path_to_entry_id(
    this: *const u8,
    file_path: *const SafeString,
) -> i32 {
    let result = call_original!(this, file_path);

    if !file_path.is_null() {
        let file_path = unsafe { &*file_path };

        println!(
            "[sead::SharcArhiveRes::convertPathToEntryIDImpl_] got id {result} for {file_path:?}"
        );
    }

    result
}

#[skyline::hook(offset = 0x218e1c8)]
unsafe fn resource_factory_try_create_with_decomp(
    this: *const u8,
    load_arg: *const LoadArg,
    decompressor: *const c_void,
) -> *const c_void {
    if !load_arg.is_null() {
        let load_arg = unsafe { &*load_arg };

        println!(
            "[sead::DirectResourceFactoryBase::tryCreateWithDecomp] creating {:?}",
            load_arg.path
        );
    }

    call_original!(this, load_arg, decompressor)
}

#[skyline::hook(offset = 0x28a24c)]
unsafe fn resource_factory_try_create(this: *const u8, load_arg: *const LoadArg) -> *const c_void {
    if !load_arg.is_null() {
        let load_arg = unsafe { &*load_arg };

        println!(
            "[sead::DirectResourceFactoryBase::tryCreate] creating {:?}",
            load_arg.path
        );
    }

    call_original!(this, load_arg)
}

#[skyline::hook(offset = 0x57f398)]
unsafe fn resource_factory_create(this: *const u8, load_arg: *const LoadArg) -> *const c_void {
    if !load_arg.is_null() {
        let load_arg = unsafe { &*load_arg };

        println!(
            "[sead::DirectResourceFactoryBase::create] creating {:?}",
            load_arg.path
        );
    }

    call_original!(this, load_arg)
}

#[skyline::hook(replace = skyline::nn::fs::OpenFile)]
unsafe fn open_file_hook(
    arg1: *mut skyline::nn::fs::FileHandle,
    path: *const u8,
    arg2: i32,
) -> u32 {
    if !path.is_null() {
        if let Ok(path_str) = std::ffi::CStr::from_ptr(path as *const c_char).to_str() {
            println!("[OpenFile] {path_str}");
        }
    }
    call_original!(arg1, path, arg2)
}

#[skyline::main(name = "tomodachi-mod-menu")]
pub fn main() {
    /*save_file::setup_custom_save();
    fsm_ext::init();*/

    skyline::install_hooks!(
        // crate::mod_menu::capture_screen_heap,
        // save_write_hook,
        // sharc_archive_res_get_file_impl,
        // sharc_archive_res_try_get_file_path_impl,
        // sharc_archive_res_convert_path_to_entry_id,
        resource_factory_create,
        resource_factory_try_create,
        resource_factory_try_create_with_decomp,
        // open_file_hook,
    );

    /*fsm_ext::register_menu(
        0x88,
        Some(state_88_enter),
        Some(state_88_update),
        Some(state_88_exit),
    );*/

    /*install_buttons! {
        "Island_Menu_00", "L_ResidentList_00" => test_spawn_mod_menu,
    }*/
}

#[repr(C)]
#[derive(Debug)]
struct LoadArg {
    pub path: SafeString,
    pub instance_heap: *const c_void,
    pub load_data_heap: *const c_void,
    pub instance_alignment: i32,
    pub load_data_alignment: i32,
    pub load_data_buffer: *const u8,
    pub load_data_buffer_size: u32,
    pub load_data_buffer_alignment: i32,
    pub factory: *const c_void,
    pub device: *const c_void,
    pub div_size: u32,
    pub assert_on_alloc_fail: bool,
    pub has_tried_create_with_decomp: *const bool,
}
