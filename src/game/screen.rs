use std::ffi::c_void;

use skyline::nn::ui2d::Pane;

use crate::{
    aal::ResourceArchive,
    eui,
    sead::{
        container::{FixedObjArray, TreeMapNode},
        math::vector::VectorF,
        thread::CriticalSection,
    },
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
