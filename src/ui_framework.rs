use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    ffi::{c_char, CStr},
    sync::RwLock,
};

#[derive(Debug)]
#[repr(C)]
pub struct ResPane {
    pub name: [u8; 24],
}

#[derive(Debug)]
#[repr(C)]
pub struct LivePane {
    pub vtable: *const u64,
    pub parent: *const LivePane,
    pub child_list: *const u64,
    pub next_sibling: *const u64,
    pub prev_sibling: *const u64,
    pub width: f32,
    pub height: f32,
    pub component_name: *const c_char,
}

#[derive(Debug)]
#[repr(C)]
pub struct ButtonState {
    pub vtable: *const u64,
    pub prev_node: *const ButtonState,
    pub next_node: *const ButtonState,
    pub res_pane: *const ResPane,
    pub live_pane: *const LivePane,
    pub unk_1: *const u64,
    pub unk_2: *const u64,
    pub state_flags: u8,
    pub unk_byte: u8,
    pub click_state: u8,
}

pub unsafe fn get_button_identity(
    btn: &ButtonState,
    x20_context: u64,
) -> Option<(String, String, u64)> {
    if x20_context < 0x80000000 {
        return None;
    }
    let controller_ptr = *((x20_context + 0xB0) as *const u64);

    if btn.res_pane.is_null() {
        return None;
    }

    let vtable = *(controller_ptr as *const u64);
    let get_name_fn_ptr = *((vtable + 0xa8) as *const u64);
    let get_name_fn: extern "C" fn(u64) -> *const c_char = std::mem::transmute(get_name_fn_ptr);
    let name_ptr = get_name_fn(controller_ptr);

    if name_ptr.is_null() {
        return None;
    }

    let controller_name = CStr::from_ptr(name_ptr)
        .to_string_lossy()
        .trim_end_matches(".bflyt")
        .to_string();

    let pane_name = CStr::from_ptr((*btn.res_pane).name.as_ptr() as *const c_char)
        .to_string_lossy()
        .into_owned();

    Some((controller_name, pane_name, controller_ptr))
}

#[macro_export]
macro_rules! install_buttons {
    ( $( $bflyt:literal, $pane:literal => $action:path ),* $(,)? ) => {

        #[skyline::hook(offset = 0x230a590, inline)]
        fn __custom_ui_inline_hook(ctx: &mut skyline::hooks::InlineCtx) {
            unsafe {
                let btn_ptr = ctx.registers[19].x() as *mut $crate::ui_framework::ButtonState;

                let btn = &mut *btn_ptr;
                let context_ptr = ctx.registers[20].x();

                if btn.click_state == 2 && btn.state_flags == 4 {
                    if let Some((bflyt, pane, controller)) = $crate::ui_framework::get_button_identity(btn, context_ptr) {
                        println!("[Scanner] ctrl: {controller:#2X}, \"{bflyt}\", \"{pane}\"");

                        let vtable_index = ctx.registers[8].x();
                        let vtable = *(btn_ptr as *const u64);
                        let target_func = *((vtable + (vtable_index * 8)) as *const u64);
                        println!("[Scanner] About to call Native Function at: {target_func:#X}");

                        $(
                            if bflyt == $bflyt && pane == $pane {
                                btn.click_state = 0;
                                $action(btn, $crate::eui::EuiController::new(controller));
                            }
                        )*
                    }
                }
            }
        }

        skyline::install_hooks!(__custom_ui_inline_hook, $crate::ui_framework::__bflyt_register_controller_hook);
    };
}

#[skyline::hook(offset = 0x6db710)]
fn __bflyt_register_controller_hook(
    controller: u64,
    loader_buffer: u64,
    screen: u64,
    manager: u64,
    param_5: u64,
    param_6: u32,
    param_7: u8,
    param_8: u16,
    param_9: u16,
    param_10: u16,
) {
    call_original!(
        controller,
        loader_buffer,
        screen,
        manager,
        param_5,
        param_6,
        param_7,
        param_8,
        param_9,
        param_10
    );

    unsafe {
        let vtable = *(controller as *const u64);
        let get_name_fn_ptr = *((vtable + 0xa8) as *const u64);
        let get_name_fn: extern "C" fn(u64) -> *const c_char = std::mem::transmute(get_name_fn_ptr);
        let name_ptr = get_name_fn(controller);

        if !name_ptr.is_null() {
            let name = CStr::from_ptr(name_ptr).to_string_lossy().to_string();

            let lr: u64;
            std::arch::asm!("mov {}, x30", out(reg) lr);

            println!("[Framework] {name} loaded! LR: {lr:#X}",);

            println!("[Framework] Registered {controller:#X} as {name}, p2: {loader_buffer:#X}, p3: {screen:#X}, p4: {manager:#X}, p5: {param_5:#X}, param_6: {param_6:#X}, param_7: {param_7:#X}, param_8: {param_8}, param_9: {param_9:#X}, {param_10:#X}");
        }
    }
}
