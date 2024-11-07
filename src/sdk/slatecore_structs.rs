use super::Basic::FName;
use crate::sdk::core_uobject_classes::UObject;

#[derive(Debug, Clone, Copy)]
pub enum EFontLoadingPolicy {
    LazyLoad = 0,
    Stream = 1,
    Inline = 2,
    EFontLoadingPolicy_MAX = 3,
}

#[derive(Debug, Clone, Copy)]
pub enum EFontHinting {
    Default = 0,
    Auto = 1,
    AutoLight = 2,
    Monochrome = 3,
    None = 4,
    EFontHinting_MAX = 5,
}

pub struct FFontData {
    pub font_filename: FName,
    pub hinting: EFontHinting,
    pub loading_policy: EFontLoadingPolicy,
    pub sub_face_index: i32,
    pub font_face_asset: Option<UObject>,
}

pub struct FTypefaceEntry {
    pub name: FName,
    pub font: FFontData,
}

pub struct FTypeface {
    pub fonts: Vec<FTypefaceEntry>,
}

struct FCompositeFallbackFont {
    pub typeface: FTypeface, // 0x0 (0x10)
    pub scaling_factor: f32, // 0x10 (0x4)
    pad_2101: [u8; 4],
}

struct FCompositeSubFont {
    pub fallback_font: FCompositeFallbackFont,
    pub character_ranges: Vec<std::ops::Range<i32>>,
    pub cultures: FName,
}

pub struct FCompositeFont {
    pub default_typeface: FTypeface,
    pub fallback_typeface: FCompositeFallbackFont,
    pub sub_typefaces: Vec<FCompositeSubFont>,
}

impl FCompositeFont {
    pub fn new(
        default_typeface: FTypeface,
        fallback_typeface: FCompositeFallbackFont,
        sub_typefaces: Vec<FCompositeSubFont>,
    ) -> Self {
        FCompositeFont {
            default_typeface,
            fallback_typeface,
            sub_typefaces,
        }
    }
}
