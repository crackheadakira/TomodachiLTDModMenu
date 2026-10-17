use std::ffi::{c_char, c_void};

use native_vtable::{vtable, vtable_impl};
use skyline::nn::ui2d::Pane;

use crate::{
    aal::ResourceArchive,
    eui,
    eui::{BoxCursorNode, ButtonBase, VftScreen},
    sead::{
        container::{FixedObjArray, TreeMapNode},
        heap::Heap,
        math::vector::VectorF,
        thread::CriticalSection,
    },
};

#[vtable(path = game::screen::viewer::VftViewer)]
pub unsafe trait VftViewer: VftScreen {
    unsafe fn is_enable_box_cursor_move(&self, node: *const BoxCursorNode) -> bool;
    unsafe fn is_use_common_back_button(&self) -> bool;
    unsafe fn is_use_common_menu_button(&self) -> bool;
    unsafe fn get_common_button_message_id(&self) -> *const u32;
    unsafe fn is_common_button_enabled(&self) -> bool;
    unsafe fn is_common_button_active(&self) -> bool;
    unsafe fn is_button_input_ready(&self) -> bool;
    unsafe fn is_common_button_decided(&self) -> bool;
    unsafe fn is_hide_common_button(&self) -> bool;
    unsafe fn unk_0x2e8(&self) -> u8;
    unsafe fn unk_0x2f0(&self) -> u8;
    unsafe fn get_fade_control(&self) -> *mut c_void;
    unsafe fn unk_0x300(&mut self, flags: u32);
    unsafe fn handle_input(&mut self);
    unsafe fn on_state_change(&mut self);
    unsafe fn get_camera_move_pane_name(&self) -> *const c_char;
    unsafe fn app_finish_open(&mut self, flags: u32);
    unsafe fn get_cpu_boost_delay_frames(&self) -> i32;

    unsafe fn is_disallow_hit_lower_screen(&self) -> bool;
    unsafe fn is_scale_enabled(&self) -> bool;
    unsafe fn is_disable_3d_touch_input(&self) -> bool;
    unsafe fn is_enable_box_cursor(&self) -> bool;
    unsafe fn is_allow_box_cursor_move_while_button_busy(&self, button: *const ButtonBase) -> bool;
    unsafe fn is_box_cursor_input_blocked(&self) -> bool;

    // +0x360: default returns zero, callable signature is unresolved

    #[vtable(offset = 0x368)]
    unsafe fn app_setup_draw_info(&mut self) -> bool;
    unsafe fn is_suppress_touch_button_input(&self) -> bool;
    unsafe fn get_camera_move_half_offset_x(&self) -> f32;
    unsafe fn get_camera_move_offset_x(&self) -> f32;
    unsafe fn get_camera_move_half_offset_y(&self) -> f32;
    unsafe fn get_camera_move_offset_y(&self) -> f32;
    unsafe fn app_do_initialize(&mut self, heap: *mut Heap);
    unsafe fn app_open_start(&mut self);
    unsafe fn app_open_end(&mut self);
    unsafe fn app_close_start(&mut self);
    unsafe fn app_close_end(&mut self);
    unsafe fn app_do_update(&mut self);
    unsafe fn is_deselect_box_cursor_on_close(&self) -> bool;
    unsafe fn on_button_input_enabled(&mut self);
    unsafe fn on_button_input_disabled(&mut self);
    unsafe fn associate_effect_sound_handle(&mut self, effect_event: *mut c_void);
    unsafe fn app_button_on_start(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_on_end(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_off_start(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_off_end(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_down_start(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_down_end(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_cancel_start(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_cancel_end(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_focus_on(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_focus_off(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_activated_by_box_cursor(&mut self, button: *mut ButtonBase);
    unsafe fn app_button_inactivated_by_box_cursor(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_focus_on(&mut self, button: *mut ButtonBase);
    unsafe fn do_button_focus_off(&mut self, button: *mut ButtonBase);

    unsafe fn is_allow_box_cursor_move_to(
        &self,
        current_node: *const BoxCursorNode,
        candidate_node: *const BoxCursorNode,
        direction: u32,
    ) -> bool;

    unsafe fn get_state_machine(&self) -> *mut c_void;
    unsafe fn register_states(&mut self, heap: *mut Heap);
    unsafe fn change_state(&mut self, state_id: u32);
    unsafe fn trigger_scene_transition(&mut self);
    unsafe fn play_screen_open_audio(&mut self);
}

const _: () = {
    assert!(core::mem::size_of::<VftViewerLayout>() == 0x488);
    assert!(core::mem::offset_of!(VftViewerLayout, base) == 0);
    assert!(VftViewerLayout::OFFSET_IS_ENABLE_BOX_CURSOR_MOVE == 0x2a0);
    assert!(VftViewerLayout::OFFSET_GET_COMMON_BUTTON_MESSAGE_ID == 0x2b8);
    assert!(VftViewerLayout::OFFSET_APP_SETUP_DRAW_INFO == 0x368);
    assert!(VftViewerLayout::OFFSET_APP_DO_INITIALIZE == 0x398);
    assert!(VftViewerLayout::OFFSET_APP_DO_UPDATE == 0x3c0);
    assert!(VftViewerLayout::OFFSET_GET_STATE_MACHINE == 0x460);
    assert!(VftViewerLayout::OFFSET_PLAY_SCREEN_OPEN_AUDIO == 0x480);
};

#[derive(PartialEq, PartialOrd)]
#[repr(u32)]
pub enum CommonButtonStyle {
    Back = 0,
    GetOut = 1,
    BackIcon = 2,
    Reduce = 3,
    FinishCheck = 4,
}

#[repr(C)]
pub struct Viewer {
    pub base: eui::Screen,
    pub elink_event_callback_base: *const c_void,
    pub animator_bindings: AnimatorBindingRegistry,
    pub button_down_end_ids: FixedObjArray<i32, 8>,
    pub pending_button_down_end_ids: FixedObjArray<i32, 8>,
    pub scale_enabled: bool,
    pub base_scale: f32,
    pub screen_lock: CriticalSection,
    pub sound_resource_archive: ResourceArchive,
    pub skip_update: bool,
    pub state_boleans: [u8; 4],
    pub camera_move_pane: *const Pane,
    pub camera_move: CameraMove,
    pub unk_330: u32,
    pub common_button_style: CommonButtonStyle,
    pub calc_fsm_in_do_update: bool,
    pub supress_next_focus_sound: bool,
    pub effect_resource: *const c_void,
}

#[repr(C)]
pub struct AnimatorBindingRegistry {
    pub vtable: *const c_void,
    pub binding_count: i32,
    pub bindings: AnimatorBindingMap,
}

#[repr(C)]
pub struct AnimatorBindingMap {
    pub vtable: *const c_void,
    pub root: *const TreeMapNode<i32, i32>,
    pub count: i32,
}

#[repr(C)]
pub struct CameraMove {
    pub vtable: *const c_void,
    pub offset: VectorF<2>,
    pub offset_scale: f32,
    pub skip_next_calc: bool,
}

const _: () = assert!(core::mem::size_of::<Viewer>() == 0x348);
const _: () = assert!(core::mem::size_of::<AnimatorBindingRegistry>() == 0x28);
const _: () = assert!(core::mem::size_of::<AnimatorBindingMap>() == 0x18);
const _: () = assert!(core::mem::size_of::<CameraMove>() == 0x18);

#[vtable_impl]
unsafe impl VftViewer for Viewer {}
