mod eui;
mod fsm_ext;
mod mod_menu;
mod ui_framework;

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

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
        *((context + 0x20) as *mut i32) = 0;
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

#[skyline::main(name = "tomodachi-mod-menu")]
pub fn main() {
    fsm_ext::init();

    skyline::install_hooks!(build_scene_controller, crate::mod_menu::capture_screen_heap);
    fsm_ext::register_menu(0x88, Some(state_88_enter), None, None);

    install_buttons! {
        "Island_Menu_00", "L_ResidentList_00" => test_spawn_mod_menu,
    }
}
