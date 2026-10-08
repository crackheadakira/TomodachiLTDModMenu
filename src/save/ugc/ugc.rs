use std::ffi::c_void;

use crate::{
    save::{
        ugc::{food::FoodManager, goods::GoodsManager},
        RegionLanguageID,
    },
    sead::{
        heap::IDisposer,
        prim::{FixedSafeString64, WFixedSafeString32, WFixedSafeString64},
    },
};

#[repr(C)]
pub struct UGCManager {
    pub static_disposer: IDisposer,
    pub is_initialized: bool,
    pub unk_0x24: u32,
    pub food: FoodManager,
    pub goods: GoodsManager,
}

#[repr(C, packed(4))]
pub struct UGCCommon {
    pub vtable: *const c_void,
    pub unk_header_data: [u8; 40],
    pub name: WFixedSafeString64,
    pub how_to_call_name: WFixedSafeString64,
    pub region_language_id: RegionLanguageID,
    pub word_attr_gramatically: WordAttrGrammaticality,
    pub word_attr_count: WordAttrCount,
    pub hash: u32,
    pub price: i32,
    pub emission_pattern: i32,
    pub emission_intensity: f32,
    pub is_emission_night_only: bool,
    pub entry_time: u64,

    pub unk_0x178: u32,
    pub unk_0x17c: u32,

    pub network_uuid: FixedSafeString64,
    pub network_deliver_count: u32,
    pub save_texture_file_size: u32,
    pub player_name: WFixedSafeString32,
    pub player_name_region_language_id: RegionLanguageID,
    pub unk_0x22c: u32,
    pub island_name: WFixedSafeString32,
    pub island_name_region_language_id: RegionLanguageID,
    pub unk_0x284: u32,
    pub unk_0x288: u32,
}

const _: () = assert!(core::mem::size_of::<UGCCommon>() == 0x28c);

#[repr(u32)]
pub enum WordAttrCount {
    Singular,
    Plural,
    SingularPossessive,
}

#[repr(u32)]
pub enum WordAttrGrammaticality {
    None,
    Masculine,
    Feminine,
    Neuter,
}
