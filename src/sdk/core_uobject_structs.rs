use super::Basic::{FName, FString};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FGuid {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

impl FGuid {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        FGuid { a, b, c, d }
    }

    pub fn equals(&self, other: &FGuid) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FIntPoint {
    pub x: i32,
    pub y: i32,
}

impl FIntPoint {
    pub fn new(x: i32, y: i32) -> Self {
        FIntPoint { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FLinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl FLinearColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        FLinearColor { r, g, b, a }
    }
}

pub struct FSoftObjectPath {
    pub asset_path_name: FName,
    pub sub_path_string: FString,
}
