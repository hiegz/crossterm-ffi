use crate::attributes::crossterm_attributes;
use crate::attributes::from_ffi_attributes_to_rust_attributes;
use crate::color::crossterm_color;
use crate::color::from_ffi_color_to_rust_color;

#[repr(C)]
pub struct crossterm_style {
    pub foreground_color: crossterm_color,
    pub background_color: crossterm_color,
    pub underline_color: crossterm_color,
    pub attributes: crossterm_attributes,

    pub has_foreground_color: bool,
    pub has_background_color: bool,
    pub has_underline_color: bool,
}

pub fn from_ffi_style_to_rust_style(ffi_style: &crossterm_style) -> crossterm::style::ContentStyle {
    crossterm::style::ContentStyle {
        foreground_color: if ffi_style.has_foreground_color {
            Some(from_ffi_color_to_rust_color(&ffi_style.foreground_color))
        } else {
            None
        },
        background_color: if ffi_style.has_background_color {
            Some(from_ffi_color_to_rust_color(&ffi_style.background_color))
        } else {
            None
        },
        underline_color: if ffi_style.has_underline_color {
            Some(from_ffi_color_to_rust_color(&ffi_style.underline_color))
        } else {
            None
        },
        attributes: from_ffi_attributes_to_rust_attributes(ffi_style.attributes),
    }
}
