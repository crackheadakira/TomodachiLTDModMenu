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
pub struct AnimTransform {
    pub vtable: *const c_void,
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

const _: () = assert!(core::mem::size_of::<TextureInfo>() == 0x10);
const _: () = assert!(core::mem::size_of::<AnimTransform>() == 0x28);
const _: () = assert!(core::mem::size_of::<AnimTransformBasic>() == 0x40);
const _: () = assert!(core::mem::size_of::<Animator>() == 0x68);
