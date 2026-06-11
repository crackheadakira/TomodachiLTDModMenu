// FUN_7101d87824, String indexing thing
// FUN_7100606ec4 dynamically sets MaxComponentCount

static MOD_SAVE_FILES: &[&str] = &["Mod.sav"];

pub const ORIGINAL_SIZE: usize = 0x9b0;
pub const NEW_SIZE: usize = ORIGINAL_SIZE + (8 * MOD_SAVE_FILES.len());

use skyline::{
    hooks::InlineCtx,
    libc::malloc,
    nn::{self, os::Tick, TimeSpan},
};
use std::ffi::{c_char, CStr, CString};

static mut NEW_TABLE: *mut *const c_char = std::ptr::null_mut();

unsafe fn patch_cmp_immediate(address: u64, new_limit: u32) {
    let instr = (address as *const u32).read();
    let new_instr = (instr & !0x003FFC00) | ((new_limit & 0xFFF) << 10);

    skyline::patching::patch_pointer(address as *const u8, &new_instr).unwrap();
    println!("[SaveSystem] Patched CMP @ {address:#x} -> #{new_limit}");
}

unsafe fn patch_adrp_add(adrp_addr: u64, add_addr: u64, new_target: u64) {
    let adrp = (adrp_addr as *const u32).read();
    let add = (add_addr as *const u32).read();

    let rd = adrp & 0x1F;
    let pc_page = adrp_addr & !0xFFF;
    let target_page = new_target & !0xFFF;
    let page_delta = target_page.wrapping_sub(pc_page) >> 12;

    let immlo = (page_delta & 0x3) as u32;
    let immhi = ((page_delta >> 2) & 0x7FFFF) as u32;
    let new_adrp = 0x90000000u32 | (immlo << 29) | (immhi << 5) | rd;

    let page_off = (new_target & 0xFFF) as u32;
    let new_add = (add & 0xFFC003FF) | (page_off << 10);

    skyline::patching::patch_pointer(adrp_addr as *const u8, &new_adrp).unwrap();
    skyline::patching::patch_pointer(add_addr as *const u8, &new_add).unwrap();

    println!("[SaveSystem] Patched ADRP+ADD @ {adrp_addr:#x} -> {new_target:#x}");
}

unsafe fn build_clean_extended_table(text_base: u64) -> u64 {
    let orig_table = (text_base + 0x31fef80) as *const *const c_char;

    let static_table_destination = (text_base + 0x4aee60 + (21 * 8)) as *mut *const c_char;

    std::ptr::copy_nonoverlapping(orig_table, static_table_destination, 3);

    for (i, name) in MOD_SAVE_FILES.iter().enumerate() {
        let save_name_ptr = CString::new(*name).unwrap().into_raw();
        *static_table_destination.add(3 + i) = save_name_ptr;
    }

    static_table_destination as u64
}

unsafe fn flush_cache(addr: u64, size: u64) {
    let start = addr as usize & !63usize;
    let end = (addr + size + 63) as usize & !63usize;

    let mut p = start;
    while p < end {
        std::arch::asm!("dc cvau, {}", in(reg) p, options(nostack));
        p += 64;
    }

    std::arch::asm!("dsb ish", options(nostack));

    p = start;
    while p < end {
        std::arch::asm!("ic ivau, {}", in(reg) p, options(nostack));
        p += 64;
    }

    std::arch::asm!("dsb ish", "isb", options(nostack));
}

pub fn setup_custom_save() {
    unsafe {
        println!("[SaveSystem] Installing hooks...");
        skyline::install_hooks!(
            tick_save_machines_reimplementation,
            write_polling_loop_reimplementation,
            save_data_setup_hook,
            save_data_create_instance_hook,
            debug_state_machine_hook,
            save_all_hook
        );

        println!("[SaveSystem] Hooks installed.");

        let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

        // build_new_table(text_base);
        let embedded_table_addr = build_clean_extended_table(text_base);

        let adrp_add_pairs = [
            (0xa36540, 0xa36544),
            (0x1d87824, 0x1d87828),
            (0x1d87920, 0x1d87924),
            (0x1d87b48, 0x1d87b4c),
        ];

        for (adrp_off, add_off) in adrp_add_pairs {
            patch_adrp_add(
                text_base + adrp_off,
                text_base + add_off,
                embedded_table_addr,
            );
        }

        let cmp_offsets = [0xa3653c, 0xa36600, 0x1d87830, 0x1d8791c, 0x1d87b64];

        for off in cmp_offsets {
            patch_cmp_immediate(text_base + off, 3 + MOD_SAVE_FILES.len() as u32);
        }

        for (off, _) in adrp_add_pairs {
            flush_cache(text_base + off, 0x200);
        }

        for off in cmp_offsets {
            flush_cache(text_base + off, 0x200);
        }
    }
}

#[skyline::hook(offset = 0xa36454)]
unsafe fn debug_state_machine_hook(this: *mut u8) {
    let comp_id = *(this.add(0x8) as *const u32);

    println!("[State::Read] State machine running with ID {comp_id}");

    call_original!(this);
}

#[skyline::hook(offset = 0x1d7c270)]
unsafe fn save_all_hook(this: *mut u8) {
    let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

    let trigger_save_machine: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x1d87b80);
    let is_save_done: extern "C" fn(*mut u8) -> bool = std::mem::transmute(text_base + 0x1d87b90);

    let base = this.add(ORIGINAL_SIZE) as *mut *mut u8;

    let mod_sav = *(this.add(ORIGINAL_SIZE) as *mut *mut u8);
    for i in 0..MOD_SAVE_FILES.len() {
        let mod_sav = *base.add(i);

        if mod_sav.is_null() {
            continue;
        }

        trigger_save_machine(mod_sav);
    }

    call_original!(this);

    let mut all_done = false;

    let check_thread_mgr: extern "C" fn(*mut u8) -> bool =
        std::mem::transmute(text_base + 0x4aefcc);
    let g_thread_manager = (text_base + 0x528b6f8) as *mut u8;
    let b_var1 = check_thread_mgr(g_thread_manager);

    while !all_done {
        all_done = true;

        for i in 0..MOD_SAVE_FILES.len() {
            let mod_sav = base.add(i);

            if mod_sav.is_null() {
                continue;
            }

            if !is_save_done(*mod_sav) {
                all_done = false;
            }
        }

        if !all_done {
            if b_var1 {
                let tick_fn: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x5e1f40);
                tick_fn(this);
            } else {
                let g_sleep_time = *((text_base + 0x32cb2f0) as *const i64);
                let to_timespan: extern "C" fn(i64) -> i64 =
                    std::mem::transmute(text_base + 0x25ccfc0);
                let sleep: extern "C" fn(i64) = std::mem::transmute(text_base + 0x25cd0f0);

                let ts = to_timespan(g_sleep_time / 100000);
                sleep(ts);
            }
        }
    }
}

#[skyline::hook(offset = 0x5e1f40)]
unsafe fn tick_save_machines_reimplementation(this: *mut u8) {
    let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    let tick_machine_fn: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x5e1ffc);

    let mii = *(this.add(0x60) as *mut *mut u8);
    let player = *(this.add(0x68) as *mut *mut u8);
    let map = *(this.add(0x70) as *mut *mut u8);

    let mod_base = *(this.add(ORIGINAL_SIZE) as *mut *mut u8);
    let mod_count = MOD_SAVE_FILES.len();

    let is_machine_idle_or_ready = |machine_ptr: *mut u8| -> bool {
        let load_machine_current_state = *(machine_ptr.add(0x10) as *const i32);
        let save_machine_current_state = *(machine_ptr.add(0x68) as *const i32);

        (load_machine_current_state == 4 || load_machine_current_state == 0)
            && (save_machine_current_state == 0 || save_machine_current_state == 3)
    };

    tick_machine_fn(mii);

    if is_machine_idle_or_ready(mii) {
        tick_machine_fn(player);

        if is_machine_idle_or_ready(player) {
            tick_machine_fn(map);

            if is_machine_idle_or_ready(map) {
                for i in 0..mod_count {
                    let mod_sav = mod_base.add(i);
                    if !mod_sav.is_null() {
                        tick_machine_fn(mod_sav);
                    }
                }
            }
        }
    }

    let is_loaded_flag = *this.add(0x94C);
    if is_loaded_flag != 1 {
        let fun_71005e3288: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x5e3288);

        let save_prop_mgr_ptr = *((text_base + 0x52979b0) as *const *mut u8);
        fun_71005e3288(save_prop_mgr_ptr);

        let light_event_type = this.add(0x978) as *mut _;

        let signal_light_event: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x25ccfd0);
        let clear_light_event: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x25ccfe0);

        signal_light_event(light_event_type);
        clear_light_event(light_event_type);
    }
}

// this seems to call "SaveState::Build"
#[skyline::hook(offset = 0x4aee60)]
unsafe fn write_polling_loop_reimplementation(this: *mut u8) {
    let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    let set_write_flag_fn: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x4aefbc);
    let check_thread_mgr_fn: extern "C" fn(*mut u8) -> bool =
        std::mem::transmute(text_base + 0x4aefcc);

    let mii = *(this.add(0x60) as *mut *mut u8);
    let player = *(this.add(0x68) as *mut *mut u8);
    let map = *(this.add(0x70) as *mut *mut u8);

    let mod_base = *(this.add(ORIGINAL_SIZE) as *mut *mut u8);
    let mod_count = MOD_SAVE_FILES.len();

    let g_thread_manager = (text_base + 0x528b6f8) as *mut u8;
    let b_var_1 = check_thread_mgr_fn(g_thread_manager);

    set_write_flag_fn(mii);
    set_write_flag_fn(player);
    set_write_flag_fn(map);

    for i in 0..mod_count {
        let m = mod_base.add(i);
        if !m.is_null() {
            set_write_flag_fn(m);
        }
    }

    let is_any_machine_busy = || -> bool {
        // while LoadMachine currentState isn't "LoadState::Done" or 0xC0 is true
        let mii_busy = (*(mii.add(0xc0)) & 1) != 0 || *(mii.add(0x10) as *const i32) != 4;
        let player_busy = (*(player.add(0xc0)) & 1) != 0 || *(player.add(0x10) as *const i32) != 4;
        let map_busy = (*(map.add(0xc0)) & 1) != 0 || *(map.add(0x10) as *const i32) != 4;

        let mut mod_busy = false;

        for i in 0..mod_count {
            let m = mod_base.add(i);
            if m.is_null() {
                continue;
            }

            mod_busy |= (*(m.add(0xc0)) & 1) != 0 || *(m.add(0x10) as *const i32) != 4;
        }

        mii_busy || player_busy || map_busy || mod_busy
    };

    if b_var_1 {
        while is_any_machine_busy() {
            tick_save_machines_reimplementation(this)
        }
    } else {
        let system_tick_frequency = *((text_base + 0x528b7d8) as *const Tick);
        let convert_to_timespan: extern "C" fn(u64) -> TimeSpan =
            std::mem::transmute(text_base + 0x25ccfc0);

        while is_any_machine_busy() {
            let timespan = convert_to_timespan(system_tick_frequency / 100);
            nn::os::SleepThread(timespan);
        }
    }

    if !b_var_1 {
        let fun_71005f8478: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x5f8478);
        fun_71005f8478(this.add(0x958));
        return;
    }

    tick_save_machines_reimplementation(this);
}

#[skyline::hook(offset = 0x601f0c)]
unsafe fn save_data_setup_hook(this: *mut u8, heap: *const u8) {
    call_original!(this, heap);
    println!(
        "[SaveSystem] save_data_setup_hook called, this={:#x}",
        this as u64
    );

    let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

    let sead_heap_allocator: extern "C" fn(usize, *const u8, i32) -> *mut u8 =
        std::mem::transmute(text_base + 0x1e4c20);

    let mod_count = MOD_SAVE_FILES.len();

    for i in 0..mod_count {
        let pg_var_3 = sead_heap_allocator(200, heap, 8);
        std::ptr::write_bytes(pg_var_3, 0, 200);

        *(pg_var_3.add(0x10) as *mut u32) = 0xffffffff;
        *(pg_var_3.add(0x18) as *mut i32) = -1;
        *(pg_var_3.add(0x1c) as *mut i32) = -1;
        *(pg_var_3.add(0x20) as *mut u32) = 0xffffffff;
        *(pg_var_3.add(0x24) as *mut u32) = 0xffffffff;
        *(pg_var_3.add(0x28) as *mut *mut u8) = std::ptr::null_mut();
        *(pg_var_3.add(0x30) as *mut u32) = 0;
        *(pg_var_3.add(0x50) as *mut *mut u8) = std::ptr::null_mut();

        *(pg_var_3.add(0x68) as *mut u32) = 0xffffffff;
        *(pg_var_3.add(0x70) as *mut u64) = 0xffffffffffffffff;
        *(pg_var_3.add(0x7c) as *mut u32) = 0xffffffff;
        *(pg_var_3.add(0x80) as *mut *mut u8) = std::ptr::null_mut();
        *(pg_var_3.add(0x88) as *mut u32) = 0;
        *(pg_var_3.add(0xa8) as *mut *mut u8) = std::ptr::null_mut();

        *(pg_var_3.add(0xc1) as *mut u8) = 0;

        *(this.add(ORIGINAL_SIZE + (8 * i)) as *mut *mut u8) = pg_var_3;
    }

    let internal_heap_ptr = *(this.add(0x38) as *const *mut u8);

    let fun_71006025a0: extern "C" fn(*mut u8, *mut u8, u32) =
        std::mem::transmute(text_base + 0x6025a0);

    for i in 0..mod_count {
        fun_71006025a0(
            *(this.add(ORIGINAL_SIZE + (8 * i)) as *mut *mut u8),
            internal_heap_ptr,
            (3 + i) as u32,
        );
    }
}

#[skyline::hook(offset = 0x5f7f7c)]
unsafe fn save_data_create_instance_hook(heap: *mut u8) -> *mut u8 {
    let text_base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;

    let g_save_data_manager_ptr = (text_base + 0x396d0f0) as *mut *mut u8;

    if !(*g_save_data_manager_ptr).is_null() {
        return *g_save_data_manager_ptr;
    }

    let operator_new: extern "C" fn(usize, *mut u8, i32) -> *mut u8 =
        std::mem::transmute(text_base + 0x1e6f40);
    let save_data_manager = operator_new(NEW_SIZE, heap, 8);

    let sead_idisposer_ctor: extern "C" fn(*mut u8, *mut u8, i32) =
        std::mem::transmute(text_base + 0x6dfa80);
    let m_static_disposer = save_data_manager.add(0x990);
    sead_idisposer_ctor(m_static_disposer, heap, 3);

    let vtable_address = *((text_base + 0x31fe2f0) as *const *mut u8);
    *(m_static_disposer as *mut *mut u8) = vtable_address;

    let dat_710396d0b8 = (text_base + 0x396d0b8) as *mut *mut u8;
    *dat_710396d0b8 = m_static_disposer;

    let fun_71005f8ed0: extern "C" fn(*mut u8) = std::mem::transmute(text_base + 0x5f8ed0);
    fun_71005f8ed0(save_data_manager);

    *g_save_data_manager_ptr = save_data_manager;

    *g_save_data_manager_ptr
}
