use std::mem::transmute;

use crate::bindings;


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorType {
    Rgb = 0x00,
    Indexed = 0x01,
    DefaultFgRgb = 0x02,
    DefaultFgIndexed = 0x03,
    DefaultBgRgb = 0x04,
    DefaultBgIndexed = 0x05,
}

impl ColorType {
    #[must_use]
    #[inline]
    pub const fn is_indexed(self) -> bool {
        matches!(self, Self::Indexed | Self::DefaultFgIndexed | Self::DefaultBgIndexed)
    }
    
    #[must_use]
    #[inline]
    pub const fn is_rgb(self) -> bool {
        matches!(self, Self::Rgb | Self::DefaultFgRgb | Self::DefaultBgRgb)
    }
    
    #[must_use]
    #[inline]
    pub const fn is_default_fg(self) -> bool {
        matches!(self, Self::DefaultFgRgb | Self::DefaultFgIndexed)
    }
    
    pub const fn is_default_bg(self) -> bool {
        matches!(self, Self::DefaultBgRgb | Self::DefaultBgIndexed)
    }
}

// impl ColorType {
//     const VTERM_RGB: Self = Self(0x00);
//     const VTERM_INDEXED: Self = Self(0x01);
//     const VTERM_TYPE_MASK: Self = Self(0x01);
//     const VTERM_DEFAULT_FG: Self = Self(0x02);
//     const VTERM_DEFAULT_BG: Self = Self(0x04);
//     const VTERM_DEFAULT_MASK: Self = Self(0x06);
    
    
// }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[must_use]
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b  }
    }
}

#[repr(C, i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Rgb(Rgb) = 0,
    Indexed(u8) = 1,
    DefaultFgRgb = 2,
    DefaultFgIndexed = 3,
    DefaultBgRgb = 4,
    DefaultBgIndexed = 5,
}

#[repr(C, i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorValue {
    Rgb(Rgb) = 0,
    Indexed(u8) = 1,
}

impl Color {
    #[must_use]
    #[inline(always)]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(Rgb::new(r, g, b))
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn rgb_gray(gray: u8) -> Self {
        Self::rgb(gray, gray, gray)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn indexed(index: u8) -> Self {
        Self::Indexed(index)
    }
    
    #[must_use]
    #[inline(always)]
    pub(crate) const fn from_bindings(vterm_color: bindings::VTermColor) -> Self {
        unsafe { transmute(vterm_color) }
    }
    
    #[must_use]
    #[inline(always)]
    pub(crate) const fn to_bindings(self) -> bindings::VTermColor {
        unsafe { transmute(self) }
    }
    
    #[must_use]
    #[inline(always)]
    pub(crate) const fn as_bound(&self) -> &bindings::VTermColor {
        unsafe { transmute(self) }
    }
    
    #[must_use]
    #[inline(always)]
    pub(crate) const fn as_bound_mut(&mut self) -> &mut bindings::VTermColor {
        unsafe { transmute(self) }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_rgb(self) -> bool {
        matches!(self, Self::Rgb(_) | Self::DefaultFgRgb | Self::DefaultBgRgb)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_indexed(self) -> bool {
        matches!(self, Self::Indexed(_) | Self::DefaultFgIndexed | Self::DefaultBgIndexed)
    }
    
    // #[must_use]
    // #[inline]
    // pub const fn value(self) -> ColorValue {
    //     match self {
    //         Color::Rgb(rgb) => ColorValue::Rgb(rgb),
    //         Color::Indexed(index) => ColorValue::Indexed(index),
    //         Color::DefaultFgRgb(rgb) => ColorValue::Rgb(rgb),
    //         Color::DefaultFgIndexed(index) => ColorValue::Indexed(index),
    //         Color::DefaultBgRgb(rgb) => ColorValue::Rgb(rgb),
    //         Color::DefaultBgIndexed(index) => ColorValue::Indexed(index),
    //     }
    // }
}