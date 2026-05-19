mod eui;
mod fsm_ext;
mod mod_menu;
mod patcher;
mod ui_framework;

use std::{
    ffi::c_char,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use crate::{
    eui::EuiController,
    fsm_ext::GAMEPLAY_CONTROLLER,
    mod_menu::{MOD_MENU_HASH, MOD_MENU_OBJ_PTR, MOD_MENU_TYPE_INFO_PTR},
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
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        let scene_mgr_pp = (text_base + 0x32cd3f8) as *const *const u64;
        let scene_mgr = **scene_mgr_pp;

        let find_by_hash: extern "C" fn(u64, u32) -> u64 =
            std::mem::transmute(text_base + 0x7c5b74);
        let wrapper = find_by_hash(scene_mgr, MOD_MENU_HASH);

        if wrapper != 0 {
            let vtable = *(wrapper as *const *const u64);
            let get_screen: extern "C" fn(u64, u64) -> u64 = std::mem::transmute(*vtable.add(2));

            let screen = get_screen(wrapper, &raw const MOD_MENU_TYPE_INFO_PTR as u64);

            if screen != 0 {
                let open_fn: extern "C" fn(u64, i32) = std::mem::transmute(text_base + 0x8d9654);
                open_fn(screen, 1);
            }
        }
    }
}

extern "C" fn state_88_execute(context: u64) {
    unsafe {
        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        // 1. Get the manager pointer from context + 0xa8
        // We treat it as a pointer to a pointer (*const *const u64)
        let manager_ptr_ptr = (context + 0xa8) as *const *const u64;

        if !manager_ptr_ptr.is_null() {
            let manager = *manager_ptr_ptr; // This is now a *const u64

            if !manager.is_null() {
                // --- HEARTBEAT ---
                // Transmute the u64 address of the update function
                let ui_update_fn: extern "C" fn(u64) = std::mem::transmute(text_base + 0x1fa49b4);

                // Call it with the manager (cast back to u64 for the function arg)
                ui_update_fn(manager as u64);

                // --- READINESS CHECK ---
                // To check offset 0xc90, cast manager to a *const u8 so .add() moves in bytes
                let ready_flag_ptr = (manager as *const u8).add(0xc90);
                if *ready_flag_ptr == 0 {
                    return; // Assets not ready, skip logic
                }
            }
        }

        // ... rest of your logic
    }
}

extern "C" fn state_88_exit(context: u64) {}

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

#[skyline::hook(offset = 0x4a9bf8)]
fn build_scene_controller(manager: u64, heap: u64, screen_id: u32, unknown: u64) -> *mut u8 {
    unsafe {
        let lr: u64;
        std::arch::asm!("mov {}, x30", out(reg) lr);

        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        let lr_offset = lr - text_base;

        println!("[BuildSceneController] SceneManager: {manager:#X}, Heap: {heap:#X}, Screen ID: {screen_id}, Unknown: {unknown:#X}, called by {lr_offset:#X}");
    }

    call_original!(manager, heap, screen_id, unknown)
}

#[skyline::hook(offset = 0x20dec8)]
fn sead_resource_mgr_load_hook(
    manager: *mut u8,
    path_ptr: *const *const u8,
    load_arg: *const u8,
    out_id: *mut u32,
) -> u64 {
    /*if !path_ptr.is_null() {
        let path = unsafe { *path_ptr };
        if !path.is_null() {
            let path_str = unsafe { std::ffi::CStr::from_ptr(path as *const c_char) };

            unsafe {
                let mut fp: u64;
                std::arch::asm!("mov {}, x29", out(reg) fp);

                let text_base =
                    skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

                let rstb_size = unsafe { *(load_arg.add(0x18) as *const u32) };
                println!("[ResourceLoader] Trace for: {path_str:?}, RSTB: {rstb_size:#X}");

                // Walk up 4 stack frames
                for i in 0..4 {
                    if fp == 0 {
                        break;
                    }

                    // In AArch64, [fp + 8] is the Link Register (return address)
                    let lr = *((fp + 8) as *const u64);

                    // Check if this address belongs to the main game executable
                    // (Assuming the game's text section is smaller than 0x4000000 bytes / 64MB)
                    if lr >= text_base && lr < text_base + 0x4000000 {
                        let lr_offset = lr - text_base;
                        println!("  -> Frame {}: Game Offset: {:#X}", i, lr_offset);
                    } else {
                        // This will catch the 0x6E... plugin addresses
                        println!("  -> Frame {}: Plugin/Trampoline Addr: {:#X}", i, lr);
                    }

                    // Move to the previous frame pointer
                    fp = *(fp as *const u64);
                }
            }
        }
    }

    call_original!(manager, path_ptr, load_arg, out_id)*/

    if !path_ptr.is_null() {
        let path = unsafe { *path_ptr };
        if !path.is_null() {
            let path_str =
                unsafe { std::ffi::CStr::from_ptr(path as *const c_char) }.to_string_lossy();

            if path_str.contains("USen.Product.100.sarc") {
                unsafe {
                    for i in 0..16 {
                        let offset = i * 4;
                        let val = *(load_arg.add(offset) as *const u32);
                        println!("[USen] Offset +0x{:02X}: 0x{:08X} ({})", offset, val, val);
                    }
                }
            } else if path_str.contains("EUen.Product.100.sarc") {
                unsafe {
                    for i in 0..16 {
                        let offset = i * 4;
                        let val = *(load_arg.add(offset) as *const u32);
                        println!("[EUen] Offset +0x{:02X}: 0x{:08X} ({})", offset, val, val);
                    }
                }
            }
        }
    }

    call_original!(manager, path_ptr, load_arg, out_id)
}

#[skyline::hook(offset = 0x6084b0)]
fn validate_rstb_insert_hook(tree_ptr: *mut u8, hash_and_size: u64) {
    let hash = (hash_and_size & 0xFFFFFFFF) as u32;
    let size = (hash_and_size >> 32) as u32;

    println!("[RSTB BUILDER] Insert -> Hash: {hash:08X} | Size: {size} ({size:0X}) (raw: {hash_and_size:0X})");

    call_original!(tree_ptr, hash_and_size);
}

#[skyline::main(name = "tomodachi-mod-menu")]
pub fn main() {
    fsm_ext::init();

    skyline::install_hooks!(
        build_scene_controller,
        crate::mod_menu::capture_screen_heap,
        crate::patcher::rstb_parse_hook,
    );
    fsm_ext::register_menu(0x88, Some(state_88_enter), Some(state_88_execute), None);

    install_buttons! {
        "Island_Menu_00", "L_ResidentList_00" => test_spawn_mod_menu,
    }
}
