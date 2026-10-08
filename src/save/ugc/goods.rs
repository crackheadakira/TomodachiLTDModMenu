use std::ffi::c_void;

use crate::{
    save::{
        ugc::ugc::{UGCCommon, UGCManager},
        RegionLanguageID, SaveFlag,
    },
    sead::{
        math::vector::VectorF,
        prim::{WFixedSafeString32, WFixedSafeString64},
    },
};

#[repr(C)]
pub struct GoodsManager {
    pub category_id: u32,
    pub unk_flags: u32,
    pub entries: SaveFlag<[GoodsEntry; 101]>,
    pub vtable: *const c_void,
    pub parent: *const UGCManager,
    pub unk_0x212a8: *const c_void,
    pub unk_0x212b0: *const c_void,
}

const _: () = assert!(core::mem::size_of::<GoodsManager>() == 0x2efa8);

#[repr(C)]
pub struct GoodsEntry {
    pub base: UGCCommon,
    pub current_idx: i32,

    pub goods_type: GoodsType,
    pub goods_feature: GoodsFeature,
    pub book_type: BookType,
    pub dvd_sound: SoundType,
    pub game_sound: SoundType,
    pub cd_sound: SoundType,
    pub pet_behavior: PetBehavior,
    pub pet_direction: PetDirection,
    pub pet_voice_type: PetVoiceType,
    pub pet_voice_effect_type: PetVoiceEffectType,
    pub other_effect: OtherEffect,
    pub base_color: VectorF<3>,
    pub goods_text: WFixedSafeString32,
    pub how_to_call_goods_text: WFixedSafeString64,

    pub unk_0x3a8: u64,

    pub goods_text_region_language_id: RegionLanguageID,
}

#[repr(u32)]
pub enum GoodsType {
    Invalid,
    Book,
    CD,
    DVD,
    Game,
    Pet,
    Other,
}

#[repr(u32)]
pub enum GoodsFeature {
    None,
    Funny,
    Impression,
    Horror,
    Discovery,
    Difficult,
    Ashamed,
}

#[repr(u32)]
pub enum BookType {
    Both,
    SideL,
    SideR,
}

#[repr(u32)]
pub enum SoundType {
    Sound00,
    Sound01,
    Sound02,
    Sound03,
    Sound04,
    Sound05,
    Sound06,
    Sound07,
    Sound08,
    Sound09,
}

#[repr(u32)]
pub enum PetBehavior {
    Skipping,
    Hopping,
    Flying,
    Slowly,
    Swimming,
}

#[repr(u32)]
pub enum PetDirection {
    LeftForward,
    RightForward,
}

#[repr(u32)]
pub enum PetVoiceEffectType {
    None,
    PetWater,
    PetRobot,
    PetShiny,
    PetGloomy,
    PetInsect,
}

#[repr(u32)]
pub enum PetVoiceType {
    None,
    Pet00,
    Pet01,
    Pet02,
    Pet03,
    Pet04,
    Pet05,
    Pet06,
    Pet07,
    Pet08,
    Pet09,
}

#[repr(u32)]
pub enum OtherEffect {
    Effect00,
    Effect01,
    Effect02,
    Effect03,
    Effect04,
    Effect05,
    Effect06,
    Effect07,
}
