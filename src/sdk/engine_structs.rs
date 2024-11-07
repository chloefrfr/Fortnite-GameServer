use super::core_uobject_structs::FLinearColor;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETextureCompressionSettings {
    Default = 0,
    Normalmap = 1,
    Masks = 2,
    Grayscale = 3,
    Displacementmap = 4,
    VectorDisplacementmap = 5,
    HDR = 6,
    EditorIcon = 7,
    Alpha = 8,
    DistanceFieldFont = 9,
    HDRCompressed = 10,
    BC7 = 11,
    Max = 12,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETextureFilter {
    Nearest = 0,
    Bilinear = 1,
    Trilinear = 2,
    Default = 3,
    Max = 4,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETextureGroup {
    World = 0,
    WorldNormalMap = 1,
    WorldSpecular = 2,
    Character = 3,
    CharacterNormalMap = 4,
    CharacterSpecular = 5,
    Weapon = 6,
    WeaponNormalMap = 7,
    WeaponSpecular = 8,
    Vehicle = 9,
    VehicleNormalMap = 10,
    VehicleSpecular = 11,
    Cinematic = 12,
    Effects = 13,
    EffectsNotFiltered = 14,
    Skybox = 15,
    UI = 16,
    Lightmap = 17,
    RenderTarget = 18,
    MobileFlattened = 19,
    ProcBuildingFace = 20,
    ProcBuildingLightMap = 21,
    Shadowmap = 22,
    ColorLookupTable = 23,
    TerrainHeightmap = 24,
    TerrainWeightmap = 25,
    Bokeh = 26,
    IESLightProfile = 27,
    Pixels2D = 28,
    HierarchicalLOD = 29,
    Impostor = 30,
    ImpostorNormalDepth = 31,
    Project01 = 32,
    Project02 = 33,
    Project03 = 34,
    Project04 = 35,
    Project05 = 36,
    Project06 = 37,
    Project07 = 38,
    Project08 = 39,
    Project09 = 40,
    Project10 = 41,
    Max = 42,
}

#[derive(Debug, Clone, Copy)]
pub enum EFontImportCharacterSet {
    FontICS_Default = 0,
    FontICS_Ansi = 1,
    FontICS_Symbol = 2,
    FontICS_MAX = 3,
}

impl EFontImportCharacterSet {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(EFontImportCharacterSet::FontICS_Default),
            1 => Some(EFontImportCharacterSet::FontICS_Ansi),
            2 => Some(EFontImportCharacterSet::FontICS_Symbol),
            3 => Some(EFontImportCharacterSet::FontICS_MAX),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FFontImportOptionsData {
    pub font_name: String,
    pub height: f32,
    pub b_enable_antialiasing: bool,
    pub b_enable_bold: bool,
    pub b_enable_italic: bool,
    pub b_enable_underline: bool,
    pub b_alpha_only: bool,
    pub character_set: EFontImportCharacterSet,
    pub chars: String,
    pub unicode_range: String,
    pub chars_file_path: String,
    pub chars_file_wildcard: String,
    pub b_create_printable_only: bool,
    pub b_include_ascii_range: bool,
    pub foreground_color: FLinearColor,
    pub b_enable_drop_shadow: bool,
    pub texture_page_width: i32,
    pub texture_page_max_height: i32,
    pub x_padding: i32,
    pub y_padding: i32,
    pub extend_box_top: i32,
    pub extend_box_bottom: i32,
    pub extend_box_right: i32,
    pub extend_box_left: i32,
    pub b_enable_legacy_mode: bool,
    pub kerning: i32,
    pub b_use_distance_field_alpha: bool,
    pub distance_field_scale_factor: i32,
    pub distance_field_scan_radius_scale: f32,
}

impl FFontImportOptionsData {
    pub fn new(
        font_name: String,
        height: f32,
        b_enable_antialiasing: bool,
        b_enable_bold: bool,
        b_enable_italic: bool,
        b_enable_underline: bool,
        b_alpha_only: bool,
        character_set: EFontImportCharacterSet,
        chars: String,
        unicode_range: String,
        chars_file_path: String,
        chars_file_wildcard: String,
        b_create_printable_only: bool,
        b_include_ascii_range: bool,
        foreground_color: FLinearColor,
        b_enable_drop_shadow: bool,
        texture_page_width: i32,
        texture_page_max_height: i32,
        x_padding: i32,
        y_padding: i32,
        extend_box_top: i32,
        extend_box_bottom: i32,
        extend_box_right: i32,
        extend_box_left: i32,
        b_enable_legacy_mode: bool,
        kerning: i32,
        b_use_distance_field_alpha: bool,
        distance_field_scale_factor: i32,
        distance_field_scan_radius_scale: f32,
    ) -> Self {
        FFontImportOptionsData {
            font_name,
            height,
            b_enable_antialiasing,
            b_enable_bold,
            b_enable_italic,
            b_enable_underline,
            b_alpha_only,
            character_set,
            chars,
            unicode_range,
            chars_file_path,
            chars_file_wildcard,
            b_create_printable_only,
            b_include_ascii_range,
            foreground_color,
            b_enable_drop_shadow,
            texture_page_width,
            texture_page_max_height,
            x_padding,
            y_padding,
            extend_box_top,
            extend_box_bottom,
            extend_box_right,
            extend_box_left,
            b_enable_legacy_mode,
            kerning,
            b_use_distance_field_alpha,
            distance_field_scale_factor,
            distance_field_scan_radius_scale,
        }
    }
}
