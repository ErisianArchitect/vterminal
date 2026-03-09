use paste::paste;
use crate::bindings::VTERM_MAX_CHARS_PER_CELL;

use crate::Color;
pub const MAX_CHARS_PER_CELL: usize = VTERM_MAX_CHARS_PER_CELL as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AttrField {
    mask: u32,
    offset: u32,
    delete_mask: u32,
}

impl AttrField {
    #[must_use]
    pub const fn new(width: u32, previous: Option<&AttrField>) -> Self {
        let mask = !(u32::MAX << width);
        let offset = match previous {
            Some(previous) => previous.offset + previous.mask_width(),
            None => 0,
        };
        Self {
            mask,
            offset,
            delete_mask: !(mask << offset),
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn mask_width(self) -> u32 {
        self.mask.trailing_ones()
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn extract_from(self, bits: u32) -> u32 {
        bits >> self.offset & self.mask
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn inject_into(self, bits: u32, value: u32) -> u32 {
        (value & self.delete_mask) | (bits << self.offset)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attrs(u32);

macro_rules! bitfields {
    (@impl:
        $previous:expr
        =>
        $width:literal,
        $lower:ident,
        $upper:ident,
        $abstract_type:ty,
        $into:expr,
        $from:expr
        $(,)?
    ) => {
        paste!{
            const _: () = {
                // this is strictly for tooling.
                #[allow(warnings)]
                const $upper: AttrField = AttrField::new(0, None);
            };
            impl Attrs {
                const [<$upper _FIELD>]: AttrField = AttrField::new($width, $previous);
                
                pub const fn $lower(self) -> $abstract_type {
                    let $lower = self.0;
                    $into
                }
                
                pub const fn [<set_ $lower>](&mut self, $lower: $abstract_type) {
                    let as_u32: u32 = $from;
                    let field = const { Self::[<$upper _FIELD>] };
                    self.0 = (self.0 & field.delete_mask) | (as_u32 << field.offset);
                }
            }
        }
    };
    (@continue: $_previous_upper:ident) => {};
    (@continue:
        $previous_upper:ident,
        [
            $next_width            : literal,
            $next_lower            : ident,
            $next_upper            : ident,
            $next_abstract_type    : ty,
            $next_into             : expr,
            $next_from             : expr
            $(,)?
        ]
        $(,[
            $width                  : literal,
            $lower                  : ident,
            $upper                  : ident,
            $abstract_type          : ty,
            $into                   : expr,
            $from                   : expr
            $(,)?
        ])*
        $(,)?
    ) => {
        paste!{
            bitfields!(@impl: Some(&Self::[<$previous_upper _FIELD>]) => $next_width, $next_lower, $next_upper, $next_abstract_type, $next_into, $next_from);
            bitfields!(@continue: $next_upper $(,[
                $width,
                $lower,
                $upper,
                $abstract_type,
                $into,
                $from
            ])*);
        }
    };
    (
        [
            $first_width            : literal,
            $first_lower            : ident,
            $first_upper            : ident,
            $first_abstract_type    : ty,
            $first_into             : expr,
            $first_from             : expr
            $(,)?
        ]
        $(,[
            $width                  : literal,  // Bitfield width.
            $lower                  : ident,    // Lowercase name for function identifiers.
            $upper                  : ident,    // Uppercase name for const identifiers.
            $abstract_type          : ty,       // Mapped type. Used to abstract away the bitfield's u32 type to constrain values.
            $into                   : expr,     // Expression to map field into return value (u32 -> type). Uses lowercase name as binding.
            $from                   : expr      // Expression to map input value into field (type -> u32). Uses lowercase name as binding.
            $(,)?
        ])*
        $(,)?
    ) => {
        bitfields!(@impl: None => $first_width, $first_lower, $first_upper, $first_abstract_type, $first_into, $first_from);
        bitfields!(@continue: $first_upper $(,[
            $width,
            $lower,
            $upper,
            $abstract_type,
            $into,
            $from
        ])*);
    };
}

#[repr(u8)]
pub enum UnderlineAttr {
    Off = 0,
    Single = 1,
    Double = 2,
    Curly = 3,
}

impl UnderlineAttr {
    #[must_use]
    #[inline(always)]
    const fn from_u32(value: u32) -> Self {
        match value & 0b11 {
            0 => Self::Off,
            1 => Self::Single,
            2 => Self::Double,
            3 => Self::Curly,
            // SAFETY: unsafe function, value is expected to be 4.
            _ => unsafe { ::core::hint::unreachable_unchecked() },
        }
    }
    
    #[must_use]
    #[inline(always)]
    const fn to_u32(self) -> u32 {
        self as u32
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontAttr {
    Font0,
    Font1,
    Font2,
    Font3,
    Font4,
    Font5,
    Font6,
    Font7,
    Font8,
    Font9,
}

impl FontAttr {
    #[must_use]
    #[inline]
    pub const fn from_u32(font_id: u32) -> Self {
        match font_id {
            1 => Self::Font1,
            2 => Self::Font2,
            3 => Self::Font3,
            4 => Self::Font4,
            5 => Self::Font5,
            6 => Self::Font6,
            7 => Self::Font7,
            8 => Self::Font8,
            9 => Self::Font9,
            _ => Self::Font0,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn to_u32(self) -> u32 {
        self as u32
    }
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DhlAttr {
    Off,
    Top = 1,
    Bottom = 2,
}

impl DhlAttr {
    #[inline]
    const fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::Top,
            2 => Self::Bottom,
            _ => Self::Off,
        }
    }
    
    pub const fn to_u32(self) -> u32 {
        self as u32
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Baseline {
    Normal = 0,
    Raise = 1,
    Lower = 2,
}

impl Baseline {
    #[inline]
    const fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::Raise,
            2 => Self::Lower,
            _ => Self::Normal,
        }
    }
    
    #[must_use]
    const fn to_u32(self) -> u32 {
        self as u32
    }
}

#[must_use]
#[inline(always)]
const fn u32_to_bool(value: u32) -> bool {
    value != 0
}

#[must_use]
#[inline(always)]
const fn bool_to_u32(value: bool) -> u32 {
    value as u32
}

// TODO
impl Attrs {
    // [bold, BOLD]: 1 => <u32>;
    // [font, FONT]: 4 => <Font> -> font_from_u32;
    // attrs_consts!(
    //     BOLD[1],
    //     UNDERLINE[2],
    //     ITALIC[1],
    //     BLINK[1],
    //     REVERSE[1],
    //     CONCEAL[1],
    //     STRIKE[1],
    //     FONT[4],
    //     DWL[2],
    //     SMALL[1],
    //     BASELINE[2],
    // );
    
    pub const NONE: Self = Self(0);
    
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self::NONE
    }
}

bitfields!(
/*      Bit width
        |   Lowercase name for function identifiers.
        |   |           Uppercase name for const identifiers.
        |   |           |           Abstract type. Used to abstract away the bitfield's u32 type to constrain values.
        |   |           |           |               Into expression to map field into return value (u32 -> type). Uses lowercase name as binding.
        |   |           |           |               |                                   From expression to map input value into field (type -> u32). Uses lowercase name as binding.
*/  [   1,  bold,       BOLD,       bool,           u32_to_bool(bold),                  bool_to_u32(bold)       ],
    [   2,  underline,  UNDERLINE,  UnderlineAttr,  UnderlineAttr::from_u32(underline), underline.to_u32()      ],
    [   1,  italic,     ITALIC,     bool,           u32_to_bool(italic),                bool_to_u32(italic)     ],
    [   1,  blink,      BLINK,      bool,           u32_to_bool(blink),                 bool_to_u32(blink)      ],
    [   1,  reverse,    REVERSE,    bool,           u32_to_bool(reverse),               bool_to_u32(reverse)    ],
    [   1,  conceal,    CONCEAL,    bool,           u32_to_bool(conceal),               bool_to_u32(conceal)    ],
    [   1,  strike,     STRIKE,     bool,           u32_to_bool(strike),                bool_to_u32(strike)     ],
    [   4,  font,       FONT,       FontAttr,       FontAttr::from_u32(font),           font.to_u32()           ],
    [   1,  dwl,        DWL,        bool,           u32_to_bool(dwl),                   bool_to_u32(dwl)        ],
    [   2,  dhl,        DHL,        DhlAttr,        DhlAttr::from_u32(dhl),             dhl.to_u32()            ],
    [   1,  small,      SMALL,      bool,           u32_to_bool(small),                 bool_to_u32(small)      ],
    [   2,  baseline,   BASELINE,   Baseline,       Baseline::from_u32(baseline),       baseline.to_u32()       ],
);

#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Width {
    Continuation = 0,
    Single = 1,
    Double = 2,
}

impl Width {
    #[must_use]
    #[inline(always)]
    pub const fn from_i8(width: i8) -> Self {
        match width {
            1 => Self::Single,
            2 => Self::Double,
            _ => Self::Continuation,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn to_i8(self) -> i8 {
        self as i8
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    len: u32,
    utf8_bytes: [u8; MAX_CHARS_PER_CELL * 4],
    width: Width,
}

const _: () = {
    ["size_of::<Symbol>() == 32"][(
        size_of::<Symbol>() - 32usize
    ) as usize];
    ["align_of::<Symbol> == 4"][(
        align_of::<Symbol>() - 4usize
    ) as usize];
};

impl Symbol {
    #[must_use]
    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        unsafe {
            ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(self.utf8_bytes.as_ptr(), self.len as usize))
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len as usize
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> Width {
        self.width
    }
    
    #[inline(always)]
    fn push(&mut self, ch: char) -> bool {
        let self_len = self.len();
        let utf8_len = ch.len_utf8();
        let len_plus = self_len + utf8_len;
        if len_plus > self.utf8_bytes.len() {
            return false;
        }
        ch.encode_utf8(&mut self.utf8_bytes[self.len as usize..]);
        self.len += utf8_len as u32;
        true
    }
    
    pub(crate) fn from_vterm(chars: &[u32; MAX_CHARS_PER_CELL], width: i8) -> Self {
        let mut builder = Self { len: 0, utf8_bytes: [0; _], width: Width::from_i8(width) };
        let mut i = 0usize;
        while i < chars.len() && chars[i] != 0 {
            let chr = unsafe { char::from_u32_unchecked(chars[i]) };
            builder.push(chr);
            i += 1;
        }
        builder
    }
}

impl ::core::ops::Deref for Symbol {
    type Target = str;
    
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for Symbol {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ::core::borrow::Borrow<str> for Symbol {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl ::core::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl ::core::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[repr(C)]
pub struct Cell {
    chars: [u32; MAX_CHARS_PER_CELL],
    width: i8,
    attrs: Attrs,
    fg: Color,
    bg: Color,
}

impl Cell {
    #[inline]
    const fn utf8_len_and_char_count(&self) -> (usize, usize) {
        let mut i = 0usize;
        let mut len = 0usize;
        while i < MAX_CHARS_PER_CELL && self.chars[i] != 0 {
            let chr = unsafe { char::from_u32_unchecked(self.chars[i]) };
            len += chr.len_utf8();
            i += 1;
        }
        (len, i)
    }
    
    #[must_use]
    pub const fn char_count(&self) -> usize {
        self.utf8_len_and_char_count().1
    }
    
    #[must_use]
    pub const fn symbol_utf8_len(&self) -> usize {
        self.utf8_len_and_char_count().0
    }
    
    #[must_use]
    pub fn symbol(&self) -> Symbol {
        Symbol::from_vterm(&self.chars, self.width)
    }
}