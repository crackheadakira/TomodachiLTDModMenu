use skyline::nn::ui2d::{ResAnimationBlock, ResAnimationContent};
use std::ffi::c_void;

use crate::sead::container::ListNode;

#[repr(C)]
pub struct BinaryBlockHeader {
    pub kind: u32,
    pub size: u32,
}

#[repr(C)]
pub struct ResAnimationTagBlock {
    pub block_header: BinaryBlockHeader,
    pub tag_order: u16,
    pub group_count: u16,
    pub name_offset: u32,
    pub groups_offset: u32,
    pub user_data_list_offset: u32,
    pub start_frame: i16,
    pub end_frame: i16,
    pub flag: u8,
    pub unk_1d: [u8; 3],
}

#[repr(C)]
pub struct TextureInfo {
    pub vtable: *const c_void,
    pub descriptor_slot: u64,
}

#[repr(C)]
pub struct AnimatorVtable {
    pub get_runtime_type_info: extern "C" fn(),
    pub dtor_anim_transform_basic: extern "C" fn(),
    pub dtor_animator: extern "C" fn(),
    pub update_frame: extern "C" fn(),
    pub set_enabled: extern "C" fn(),
    pub animate: extern "C" fn(),
    pub animate_pane: extern "C" fn(),
    pub animate_material: extern "C" fn(),
    pub set_resource_1: extern "C" fn(),
    pub set_resource_2: extern "C" fn(),
    pub bind_pane: extern "C" fn(),
    pub bind_group: extern "C" fn(),
    pub bind_material: extern "C" fn(),
    pub force_bind_pane: extern "C" fn(),
    pub unbind_pane: extern "C" fn(),
    pub unbind_group: extern "C" fn(),
    pub unbind_material: extern "C" fn(),
    pub unbind_all: extern "C" fn(),
    pub unk_90: extern "C" fn(),
    pub unk_98: extern "C" fn(),
    pub animate_pane_impl: extern "C" fn(),
    pub animate_material_impl: extern "C" fn(),
    pub animate_ext_user_data_impl: extern "C" fn(),
    pub play: extern "C" fn(),
    pub play_auto: extern "C" fn(),
    pub play_from_current: extern "C" fn(),
    pub stop: extern "C" fn(),
    pub stop_current: extern "C" fn(),
    pub stop_at_min: extern "C" fn(),
    pub stop_at_max: extern "C" fn(),
}

#[repr(C)]
pub struct AnimTransform {
    pub vtable: *const AnimatorVtable,
    pub list_node: ListNode,
    pub res: *mut ResAnimationBlock,
    pub frame: f32,
    pub is_enabled: bool,
    pub pad_25: [u8; 3],
}

#[repr(C)]
pub struct BindPair {
    pub target: *const c_void,
    pub anim_cont: *const ResAnimationContent,
}

#[repr(C)]
pub struct AnimTransformBasic {
    pub base: AnimTransform,
    pub textures: *const TextureInfo,
    pub bind_pairs: *const BindPair,
    pub bind_pair_count: u16,
    pub bind_pair_count_max: u16,
    pub unk_3c: i32,
}

#[repr(C)]
pub struct Animator {
    pub base: AnimTransformBasic,
    pub unk_40: ListNode,
    pub target_frame: f32,
    pub flags: u16,
    pub m_loop: u8,
    pub unk_57: u8,
    pub layout: *const c_void, // LayoutEx
    pub tag_block: *const ResAnimationTagBlock,
}

impl Animator {
    pub unsafe fn get_frame_size(&self) -> u16 {
        let res = &*(self.base.base.res);

        res.num_frames
    }
}

const _: () = assert!(core::mem::size_of::<TextureInfo>() == 0x10);
const _: () = assert!(core::mem::size_of::<AnimTransform>() == 0x28);
const _: () = assert!(core::mem::size_of::<AnimTransformBasic>() == 0x40);
const _: () = assert!(core::mem::size_of::<Animator>() == 0x68);
const _: () = assert!(core::mem::size_of::<AnimatorVtable>() == 0xf0);
