use std::ffi::c_void;

use crate::{
    save::SaveFlag,
    sead::{
        container::FixedTreeMap,
        heap::IDisposer,
        prim::{WFixedSafeString32, WFixedSafeString64},
    },
};

#[repr(C)]
pub struct PlayerManager {
    pub disposer: IDisposer,
    pub birthday: Birthday,
    pub last_player_birthday_time: SaveFlag<i64>,
    pub name: SaveFlag<WFixedSafeString32>,
    pub how_to_call_name: SaveFlag<WFixedSafeString64>,
    pub name_region_language_id: SaveFlag<RegionLanguageID>,
    pub island_name: SaveFlag<WFixedSafeString32>,
    pub how_to_call_island_name: SaveFlag<WFixedSafeString64>,
    pub island_name_region_language_id: SaveFlag<RegionLanguageID>,
    pub clock_snapshot: SaveFlag<ClockSnapshot>,
    pub last_region_language_id: SaveFlag<RegionLanguageID>,
    pub last_penalty_time: SaveFlag<i64>,
    pub last_tips_index: SaveFlag<i32>,
    pub region_code: SaveFlag<RegionCode>,
    pub region: SaveFlag<Region>,
    pub hemisphere: SaveFlag<Hemisphere>,
    pub currency: SaveFlag<Currency>,
    pub skin_color_index: SaveFlag<u32>,
    pub encyclopedia_opened_sector: SaveFlag<u32>,
    pub money: SaveFlag<u32>,
    pub emotion_ball: SaveFlag<u32>,
    pub bond_info: BondInfo,
    pub food_info: SaveFlag<FixedTreeMap<u32, ItemTimelineInfo, 512>>,
    pub building_info: SaveFlag<FixedTreeMap<u32, TimedCatalogInfo, 512>>,
    pub floor_info: SaveFlag<FixedTreeMap<u32, DirectCatalogInfo, 50>>,
    pub cloth_info: SaveFlag<[ItemTimelineInfo; 19200]>,
    pub coordinate_info: SaveFlag<[ItemTimelineInfo; 6400]>,
    pub goods_info: SaveFlag<FixedTreeMap<u32, ItemSaveInfo, 300>>,
    pub habit_info: SaveFlag<FixedTreeMap<u32, ItemSaveInfo, 128>>,
    pub interior_room_style_info: SaveFlag<FixedTreeMap<u32, TimedCatalogInfo, 512>>,
    pub trouble_info: SaveFlag<FixedTreeMap<u32, TroubleInfo, 130>>,
    pub common_daily_item: CommonDailyItem,
    pub food_shop_daily_item: FoodShopDailyItem,
    pub cloth_shop_daily_item: ClothShopDailyItem,
    pub room_style_weekly_item: RoomStyleWeeklyItem,
    pub weekly_interior_shop_furniture_idx: SaveFlag<i32>,
    pub building_shop_item: ShopItem,
    pub goods_shop_item: ShopItem,
    pub last_bazaar_purchased_time: SaveFlag<i64>,
    pub item_shop_display_ugc_goods_hash: SaveFlag<[u32; 8]>,
    pub last_donation_game_time: SaveFlag<i64>,
    pub shop_paid_money_total: SaveFlag<[i32; 6]>,
    pub play_time: SaveFlag<u64>,
    pub no_input_play_time: SaveFlag<u64>,
    pub save_data_version: SaveFlag<i32>,
    pub save_data_unique_hash: SaveFlag<u64>,
    pub save_data_create_from_product: SaveFlag<bool>,
    pub is_camera_reverse_rotate_y: SaveFlag<bool>,
    pub is_local_network_explained: SaveFlag<bool>,
    pub is_local_network_first_received_mii: SaveFlag<bool>,
    pub is_local_network_first_received_ugc: SaveFlag<bool>,
    pub is_local_network_first_received_item: SaveFlag<bool>,
    pub is_explained_island_gauge: SaveFlag<bool>,
    pub proposal_failure_count: SaveFlag<i32>,
    pub last_island_edit_time: SaveFlag<i64>,
    pub next_generate_trouble_island_edit_time: SaveFlag<i64>,
    pub special_sale_start_time: SaveFlag<i64>,
    pub special_sale_end_time: SaveFlag<i64>,
    pub first_boot_time: SaveFlag<i64>,
    pub unlock_map_level: SaveFlag<i32>,
    pub save_event_flag: SaveFlag<u32>,
    pub breaking_news_flag: SaveFlag<u64>,
    pub last_news_watched_time: SaveFlag<i64>,
    pub next_update_news_random_seed: SaveFlag<u32>,
    pub mii_birthday_news: MiiBirthdayNews,
    pub photo_studio_unlock_flag: SaveFlag<u64>,
    pub photo_studio_unlock_new_flag: SaveFlag<u64>,
    pub bgm_volume: SaveFlag<u32>,
    pub se_value: SaveFlag<u32>,
    pub voice_volume: SaveFlag<u32>,
    pub market_update_info: MarketUpdateInfo,
    pub trial: Trial,
    pub unk: u64,
}

const _: () = assert!(core::mem::size_of::<PlayerManager>() == 0x243e38);

#[derive(Debug)]
#[repr(u32)]
pub enum RegionLanguageID {
    JPja,
    USen,
    USes,
    USfr,
    USpt,
    EUen,
    EUes,
    EUfr,
    EUde,
    EUit,
    EUpt,
    EUnl,
    EUru,
    KRko,
    CNzh,
    TWzh,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Region {
    Invalid,
    Japan,
    Europe,
    NorthAmerica,
    SouthAmericaN,
    SouthAmericaS,
    Australia,
    Asia,
    OthersN,
    OthersS,
}

#[derive(Debug)]
#[repr(u32)]
pub enum RegionCode {
    Invalid,
    Japan,
    Usa,
    Europe,
    Australia,
    HongKongTaiwanKorea,
    China,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Hemisphere {
    Invalid,
    North,
    South,
}

#[derive(Debug)]
#[repr(u32)]
pub enum Currency {
    Invalid,
    Yen,
    Dollar,
    Euro,
    Pound,
    AsiaDollar,
    Won,
    Yuan,
    Rouble,
    Peso,
    GeneralUse,
}

#[derive(Debug)]
#[repr(C)]
pub struct Birthday {
    pub year: SaveFlag<u64>,
    pub month: SaveFlag<u64>,
    pub day: SaveFlag<u64>,
}

#[derive(Debug)]
#[repr(C)]
pub struct BondInfo {
    pub lottery_for_good: SaveFlag<i32>,
    pub lottery_for_bad: SaveFlag<i32>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CommonDailyItem {
    pub next_update_random_seed: SaveFlag<u32>,
    pub last_update_time: SaveFlag<i64>,
}

#[derive(Debug)]
#[repr(C)]
pub struct FoodShopDailyItem {
    pub next_update_random_seed: SaveFlag<[u32; 4]>,
    pub is_need_show_new: SaveFlag<bool>,
    pub is_unlocked_other_region: SaveFlag<bool>,
}

#[derive(Debug)]
#[repr(C)]
pub struct ClothShopDailyItem {
    pub daily_coordinate_info: SaveFlag<[GenericCoordinateInfo; 5]>,
    pub daily_cloth_info: SaveFlag<[DailyClothInfo; 40]>,
    pub is_need_show_new: SaveFlag<bool>,
}

#[derive(Debug)]
#[repr(C)]
pub struct RoomStyleWeeklyItem {
    pub room_style_variation_group_string_id: SaveFlag<u32>,
    pub is_need_show_new: SaveFlag<bool>,
}

#[derive(Debug)]
#[repr(C)]
pub struct ShopItem {
    pub is_need_show_new: SaveFlag<bool>,
}

#[derive(Debug)]
#[repr(C)]
pub struct GenericCoordinateInfo {
    pub coordinate_string_id: u32,
    pub variation: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct DailyClothInfo {
    pub cloth_string_id: u32,
    pub color: i32,
    pub cloth_type: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MiiBirthdayNews {
    pub last_watched_time: SaveFlag<i64>,
    pub watched_count: SaveFlag<i32>,
    pub watched_mii_index: [SaveFlag<i32>; 6],
}

#[derive(Debug)]
#[repr(C)]
pub struct MarketUpdateInfo {
    pub last_update_time: SaveFlag<i64>,
    pub last_enter_time: SaveFlag<i64>,
    pub market_food_string_id: SaveFlag<u32>,
    pub market_coordinate_info: SaveFlag<GenericCoordinateInfo>,
}

#[derive(Debug)]
#[repr(C)]
pub struct Trial {
    pub is_viewed_sequence_import_trial_save_data: SaveFlag<bool>,
    pub is_success_import_trial_save_data: SaveFlag<bool>,
}

#[derive(Debug)]
#[repr(C)]
pub struct ClockSnapshot {
    pub user_context: SystemClockContext,
    pub network_context: SystemClockContext,
    pub user_posix_time: i64,
    pub network_posix_time: i64,
    pub user_calendar_time: CalendarTime,
    pub network_calendar_time: CalendarTime,
    pub user_calendar_additional_info: CalendarAdditionalInfo,
    pub network_calendar_additional_info: CalendarAdditionalInfo,
    pub steady_clock_time_point: SteadyClockTimePoint,
    pub location_name: [u8; 36],
    pub is_automatic_correction_enabled: bool,
    pub _type: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct SteadyClockTimePoint {
    pub value: i64,
    pub source_id: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub struct SystemClockContext {
    pub offset: i64,
    pub steady_time_point: SteadyClockTimePoint,
}

#[derive(Debug)]
#[repr(C)]
pub struct CalendarTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct CalendarAdditionalInfo {
    pub day_of_week: u32,
    pub day_of_year: u32,
    pub timezone_abbr: [u8; 8],
    pub is_dst: u32,
    pub utc_offset_sec: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemSaveInfo {
    pub vtable: *const c_void,
    pub last_obtained_sec: u64,
    pub own_num: i32,
    pub is_newly_owned: bool,
    pub state: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemTimelineInfo {
    pub base: ItemSaveInfo,
    pub first_obtained_sec: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct TimedCatalogInfo {
    pub base: ItemTimelineInfo,
    pub key_hash: i32,
    pub informed_new_release: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct DirectCatalogInfo {
    pub base: ItemSaveInfo,
    pub key_hash: i32,
    pub informed_new_release: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct TroubleInfo {
    pub vtable: *const c_void,
    pub key_hash: u32,
    pub generate_count: i32,
    pub resolve_success_count: i32,
    pub last_generate_time: u64,
}
