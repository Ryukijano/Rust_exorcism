use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
#[derive(Debug, IntEnum, IntoEnumIterator, Copy, Clone, PartialEq)]
#[repr(usize)]
pub enum Color {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_code(color: &Color) -> usize {
    color as usize
}
pub fn color_to_value_string(value: usize) -> String {
    match ResistorColor::from_usize(value) {
        Some(color) => color.to_string(),
        None => "".to_string(),
    }
}
pub fn color_to_value_string_with_band(value: usize) -> String {
    match ResistorColor::from_usize(value) {
        Some(color) => format!("{} {}", color.to_string(), color.band()),
        None => "".to_string(),
    }
}

pub fn list_colors() -> String {
    let mut colors = String::new();
    for color in ResistorColor::iter() {
        colors.push_str(&format!("{} ", color.to_string()));
    }
    colors
}

