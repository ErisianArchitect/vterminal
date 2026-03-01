
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
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbType {
    pub color_type: ColorType,
    pub rgb: Rgb,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexedType {
    pub color_type: ColorType,
    pub index: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union ColorUnion {
    pub color_type: ColorType,
    pub rgb: RgbType,
    pub indexed: IndexedType,
}

impl PartialEq<Self> for ColorUnion {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            if self.color_type != other.color_type {
                return false;
            }
            match self.color_type {
                ColorType::Rgb => self.rgb.rgb == other.rgb.rgb,
                ColorType::Indexed => self.indexed.index == other.indexed.index,
                ColorType::DefaultFgRgb => self.rgb.rgb == other.rgb.rgb,
                ColorType::DefaultFgIndexed => self.indexed.index == other.indexed.index,
                ColorType::DefaultBgRgb => self.rgb.rgb == other.rgb.rgb,
                ColorType::DefaultBgIndexed => self.indexed.index == other.indexed.index,
            }
        }
    }
}

impl Eq for ColorUnion {}

impl ::core::fmt::Debug for ColorUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.color_type {
                ColorType::Rgb => write!(f, "ColorUnion::Rgb({:?})", self.rgb),
                ColorType::Indexed => write!(f, "ColorUnion::Indexed({:?})", self.indexed),
                ColorType::DefaultFgRgb => write!(f, "ColorUnion::DefaultFgRgb({:?})", self.rgb),
                ColorType::DefaultFgIndexed => write!(f, "ColorUnion::DefaultFgIndexed({:?})", self.indexed),
                ColorType::DefaultBgRgb => write!(f, "ColorUnion::DefaultBgRgb({:?})", self.rgb),
                ColorType::DefaultBgIndexed => write!(f, "ColorUnion::DefaultGbIndexed({:?})", self.indexed),
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    color_union: ColorUnion,
}

impl Color {
    #[must_use]
    #[inline(always)]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            color_union: ColorUnion {
                rgb: RgbType {
                    color_type: ColorType::Rgb,
                    rgb: Rgb {
                        red,
                        green,
                        blue
                    }
                }
            }
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn indexed(index: u8) -> Self {
        Self {
            color_union: ColorUnion {
                indexed: IndexedType {
                    color_type: ColorType::Indexed,
                    index,
                }
            }
        }
    }
}