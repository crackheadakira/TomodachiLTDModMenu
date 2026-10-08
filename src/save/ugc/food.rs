use std::ffi::c_void;

use crate::save::{
    ugc::ugc::{UGCCommon, UGCManager},
    SaveFlag,
};

#[repr(C)]
pub struct FoodManager {
    pub category_id: u32,
    pub unk_flags: u32,
    pub entries: SaveFlag<[FoodEntry; 101]>,
    pub vtable: *const c_void,
    pub parent: *const UGCManager,
    pub unk_0x212a8: *const c_void,
    pub unk_0x212b0: *const c_void,
}

const _: () = assert!(core::mem::size_of::<FoodManager>() == 0x212b8);

#[repr(C)]
pub struct FoodEntry {
    pub base: UGCCommon,
    pub current_idx: i32,
    pub attr_taste: AttrTaste,
    pub attr_type: AttrType,
    pub temperature: Temperature,
    pub base_scale: f32,
}

#[repr(u32)]
pub enum AttrTaste {
    None,
    Sweet,
    Spicy,
    Salty,
    Sour,
    Oily,
    Awful,
}

#[repr(u32)]
pub enum AttrType {
    None,
    Vegetable,
    Meat,
    Fish,
    Rice,
    Noodle,
    Bread,
    Drink,
    Confectionery,
    Fruit,
    Egg,
}

#[repr(u32)]
pub enum Temperature {
    Normal,
    Hot,
    Cool,
}
