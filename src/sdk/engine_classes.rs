use std::sync::{Arc, Once};

use super::{
    core_uobject_classes::UClass,
    core_uobject_structs::{FGuid, FIntPoint, FSoftObjectPath},
    engine_structs::{
        ETextureCompressionSettings, ETextureFilter, ETextureGroup, FFontImportOptionsData,
    },
    slatecore_structs::FCompositeFont,
    Basic::{FName, FString},
};

pub struct UAssetUserData;

impl UAssetUserData {
    pub fn static_class() -> &'static str {
        static ONCE: Once = Once::new();
        static mut CLSS: Option<&'static str> = None;

        ONCE.call_once(|| unsafe {
            CLSS = Some("AssetUserData");
        });

        unsafe { CLSS.unwrap() }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ETextureAddress {
    Wrap = 0,
    Clamp = 1,
    Mirror = 2,
    Max = 3,
}

#[derive(Debug)]
pub struct UTexture2D {
    pub streaming_index: i32,
    pub level_index: i32,
    pub first_resource_mem_mip: i32,
    pub temporarily_disable_streaming: bool,
    pub is_streamable: bool,
    pub has_streaming_update_pending: bool,
    pub force_miplevels_to_be_resident: bool,
    pub ignore_streaming_mip_bias: bool,
    pub global_force_mip_levels_to_be_resident: bool,
    pub address_x: ETextureAddress,
    pub address_y: ETextureAddress,
    pub imported_size: FIntPoint,
    pub force_mip_levels_to_be_resident_timestamp: f64,
}

pub struct UTexture {
    _pad_5a3: [u8; 8],
    pub lighting_guid: FGuid,
    pub lod_bias: i32,
    pub num_cinematic_mip_levels: i32,
    pub compression_settings: ETextureCompressionSettings,
    pub filter: ETextureFilter,
    pub lod_group: ETextureGroup,
    pub s_rgb: bool,
    pub never_stream: bool,
    pub no_tiling: bool,
    pub use_cinematic_mip_levels: bool,
    pub async_resource_release_started: bool,
    pub cached_combined_lod_bias: i32,
    pub asset_user_data: Vec<UAssetUserData>,
    _pad_5a4: [u8; 80],
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EFontCacheType {
    Offline = 0,
    Runtime = 1,
    FontCacheTypeMax = 2,
}

#[derive(Debug)]
pub struct FFontCharacter {
    pub start_u: i32,
    pub start_v: i32,
    pub u_size: i32,
    pub v_size: i32,
    pub texture_index: u8,
    _pad_ca0: [u8; 3],
    pub vertical_offset: i32,
}

impl FFontCharacter {
    pub fn new(
        start_u: i32,
        start_v: i32,
        u_size: i32,
        v_size: i32,
        texture_index: u8,
        vertical_offset: i32,
    ) -> Self {
        FFontCharacter {
            start_u,
            start_v,
            u_size,
            v_size,
            texture_index,
            _pad_ca0: [0; 3],
            vertical_offset,
        }
    }
}

pub struct UFont {
    _pad_60c: [u8; 8],
    pub font_cache_type: EFontCacheType,
    _pad_60d: [u8; 7],
    pub characters: Vec<FFontCharacter>,
    pub textures: Vec<UTexture2D>,
    pub is_remapped: i32,
    pub em_scale: f32,
    pub ascent: f32,
    pub descent: f32,
    pub leading: f32,
    pub kerning: i32,
    pub import_options: FFontImportOptionsData,
    pub num_characters: i32,
    _pad_60e: [u8; 4],
    pub max_char_height: Vec<i32>,
    pub scaling_factor: f32,
    pub legacy_font_size: i32,
    pub legacy_font_name: FName,
    pub composite_font: FCompositeFont,
    _pad_60f: [u8; 80],
}

// TODO: Finish this
#[derive(Debug, Clone, Copy)]
pub struct ULocalPlayer {
    _pad_311: [u8; 0x28],
    // viewport_client: Option<*mut UGameViewportClient>,
    _pad_312: [u8; 0x1C],
    // aspect_ratio_axis_constraint: EAspectRatioAxisConstraint,
    _pad_313: [u8; 0x3],
    // pending_level_player_controller_class: Option<*mut APlayerController>,
    b_sent_split_join: bool,
    _bit_pad_2c: u8,
    _pad_314: [u8; 0x67],
    controller_id: i32,
    _pad_315: [u8; 0xBC],
}

#[derive(Debug, Clone)]
pub struct UConsole {
    pad_481: [u8; 16],
    pub console_target_player: Option<Arc<ULocalPlayer>>,
    pub default_texture_black: Option<Arc<UTexture2D>>,
    pub default_texture_white: Option<Arc<UTexture2D>>,
    pub history_buffer: Vec<FString>,
    pad_483: [u8; 184],
}

pub struct UEngine {
    tiny_font: Option<UFont>,
    tiny_font_name: FSoftObjectPath,
    small_font: Option<UFont>,
    small_font_name: FSoftObjectPath,
    medium_font: Option<UFont>,
    medium_font_name: FSoftObjectPath,
    large_font: Option<UFont>,
    large_font_name: FSoftObjectPath,
    sub_title_font: Option<UFont>,
    sub_title_font_name: FSoftObjectPath,
    additional_fonts: Vec<UFont>,
    additional_font_names: Vec<FString>,
}

#[derive(Default)]
pub struct UProperty {}

#[derive(Default)]
pub struct UChannel {}

#[derive(Default)]
pub struct UReplicationDriver {}

#[derive(Default)]
pub struct UPackage {}

#[derive(Default)]
pub struct UNetConnection {}

#[derive(Default)]
pub struct UNetDriver {
    pub net_connection_class_name: FString,
    pub replication_driver_class_name: FString,
    pub max_download_size: i32,
    pub b_clamp_listen_server_tick_rate: bool,
    pub bit_pad_33: u8,
    pub net_server_max_tick_rate: i32,
    pub max_internet_client_rate: i32,
    pub max_client_rate: i32,
    pub server_travel_pause: f32,
    pub spawn_priority_seconds: f32,
    pub relevant_timeout: f32,
    pub keep_alive_time: f32,
    pub initial_connect_timeout: f32,
    pub connection_timeout: f32,
    pub timeout_multiplier_for_unoptimized_builds: f32,
    pub b_no_timeouts: bool,
    pub server_connection: Option<Box<UNetConnection>>,
    pub client_connections: Vec<Box<UNetConnection>>,
    pub world: Option<Box<UWorld>>,
    pub world_package: Option<Box<UPackage>>,
    pub net_connection_class: Option<Box<UClass>>,
    pub replication_driver_class: Option<Box<UClass>>,
    pub role_property: Option<Box<UProperty>>,
    pub remote_role_property: Option<Box<UProperty>>,
    pub net_driver_name: FString,
    pub actor_channel_pool: Vec<Box<UChannel>>,
    pub time: f32,
    pub replication_driver: Option<Box<UReplicationDriver>>,
}

impl UNetDriver {
    pub fn new() -> Self {
        UNetDriver {
            net_connection_class_name: FString::default(),
            replication_driver_class_name: FString::default(),
            max_download_size: 0,
            b_clamp_listen_server_tick_rate: false,
            bit_pad_33: 0,
            net_server_max_tick_rate: 0,
            max_internet_client_rate: 0,
            max_client_rate: 0,
            server_travel_pause: 0.0,
            spawn_priority_seconds: 0.0,
            relevant_timeout: 0.0,
            keep_alive_time: 0.0,
            initial_connect_timeout: 0.0,
            connection_timeout: 0.0,
            timeout_multiplier_for_unoptimized_builds: 0.0,
            b_no_timeouts: false,
            server_connection: None,
            client_connections: Vec::new(),
            world: None,
            world_package: None,
            net_connection_class: None,
            replication_driver_class: None,
            role_property: None,
            remote_role_property: None,
            net_driver_name: FString::default(),
            actor_channel_pool: Vec::new(),
            time: 0.0,
            replication_driver: None,
        }
    }
}

pub struct UWorld {}
