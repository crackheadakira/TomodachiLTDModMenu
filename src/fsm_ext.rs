use lazy_static::lazy_static;
use std::ffi::{c_char, CStr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

pub static GAMEPLAY_CONTROLLER: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref MENU_REGISTRY: RwLock<Vec<CustomMenu>> = RwLock::new(Vec::new());
}

type FsmEnterFn = extern "C" fn(u64);
type FsmUpdateFn = extern "C" fn(u64);
type FsmExitFn = extern "C" fn(u64);

pub struct CustomMenu {
    pub id: u32,
    pub enter_fn: Option<FsmEnterFn>,
    pub update_fn: Option<FsmUpdateFn>,
    pub exit_fn: Option<FsmExitFn>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Pmf {
    pub ptr: u64,
    pub adj: u64,
}

#[repr(C)]
pub struct FsmDelegateState {
    pub vtable: u64,
    pub context: u64,
    pub enter_pmf: Pmf,
    pub update_pmf: Pmf,
    pub exit_pmf: Pmf,
}

#[skyline::hook(offset = 0xa661dc)]
fn capture_state_machine(param_1: u64) {
    GAMEPLAY_CONTROLLER.store(param_1, Ordering::SeqCst);
    println!("[FSM Library] Storing gameplay controller: {param_1:#X}");

    call_original!(param_1);
}

#[skyline::hook(offset = 0x4537f4)]
pub fn register_state_name_hook(fsm: u64, id: u32, name_ptr: *const *const c_char) {
    call_original!(fsm, id, name_ptr);

    unsafe {
        let name = CStr::from_ptr(*name_ptr).to_string_lossy();
        // println!("[FSM] Used FSM {fsm:#X} to register ID {id}: {name}");

        if id == 57 {
            install_framework(fsm);
        }
    }
}

unsafe fn install_framework(fsm_base: u64) {
    let original_count = *((fsm_base + 0x28) as *const u32);
    let original_table = *((fsm_base + 0x30) as *const *mut u8);

    let new_count = 256;
    let new_table = skyline::libc::malloc(256 * 0x40) as *mut u8;

    std::ptr::copy_nonoverlapping(original_table, new_table, (original_count as usize) * 0x40);

    let state_0_addr = new_table as *const FsmDelegateState;
    let vanilla_vtable = (*state_0_addr).vtable;
    let vanilla_context = GAMEPLAY_CONTROLLER.load(Ordering::SeqCst);

    let registry = MENU_REGISTRY.read().unwrap();

    let make_pmf = |func: Option<extern "C" fn(u64)>| -> Pmf {
        Pmf {
            ptr: func.map(|f| f as u64).unwrap_or(0),
            adj: 0,
        }
    };

    let make_update_pmf = |func: Option<FsmUpdateFn>| -> Pmf {
        Pmf {
            ptr: func.map(|f| f as u64).unwrap_or(0),
            adj: 0,
        }
    };

    for menu in registry.iter() {
        if menu.id >= new_count {
            continue;
        }

        let slot_addr = new_table.add((menu.id as usize) * 0x40) as *mut FsmDelegateState;

        let enter = make_pmf(menu.enter_fn);
        let update = make_update_pmf(menu.update_fn);
        let exit = make_pmf(menu.exit_fn);

        let custom_state = FsmDelegateState {
            vtable: vanilla_vtable,
            context: vanilla_context,
            enter_pmf: enter,
            update_pmf: update,
            exit_pmf: exit,
        };

        std::ptr::write(slot_addr, custom_state);

        println!("[FSM Library] Registered Native ID {:#X}", menu.id);
    }

    *((fsm_base + 0x30) as *mut *mut u8) = new_table;
    *((fsm_base + 0x28) as *mut u32) = new_count;
}

pub fn init() {
    skyline::install_hooks!(capture_state_machine, register_state_name_hook);
}

pub fn register_menu(
    id: u32,
    enter: Option<FsmEnterFn>,
    update: Option<FsmUpdateFn>,
    exit: Option<FsmExitFn>,
) {
    let mut registry = MENU_REGISTRY.write().expect("Failed to lock MENU_REGISTRY");

    if registry.iter().any(|m| m.id == id) {
        println!("[Warning] ID {id:#X} is already registered! Last mod to register wins.",);
    }

    registry.push(CustomMenu {
        id,
        enter_fn: enter,
        update_fn: update,
        exit_fn: exit,
    });

    println!("[FSM Library] Queued Native Delegate ID {id:#X} for registration.",);
}
