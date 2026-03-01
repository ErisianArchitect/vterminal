use paste::paste;

use crate::Color;
pub const MAX_CHARS_PER_CELL: usize = 6;


#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attrs(u32);

macro_rules! attrs_consts {
    (@rest $before:ident) => {};
    (@rest $before:ident, $next:ident[$width:literal] $(, $rest:ident[$rest_width:literal])*$(,)?) => {
        paste!{
            const [<$next _MASK>]: u32 = !(u32::MAX << $width);
            const [<$next _OFFSET>]: u32 = Self::[<$before _OFFSET>] + Self::[<$before _MASK>].trailing_ones();
        }
        attrs_consts!(@rest $next $(, $rest[$rest_width])*);
    };
    ($first:ident[$bit_width:literal] $(, $rest:ident[$width:literal])+ $(,)?) => {
        paste!{
            const [<$first _MASK>]: u32 = !(u32::MAX << $bit_width);
            const [<$first _OFFSET>]: u32 = 0;
        }
        attrs_consts!(@rest $first $(, $rest[$width])*);
    };
}

// TODO
impl Attrs {
    attrs_consts!(
        BOLD[1],
        UNDERLINE[2],
        ITALIC[1],
        BLINK[1],
        REVERSE[1],
        CONCEAL[1],
        STRIKE[1],
        FONT[4],
        DWL[2],
        SMALL[1],
        BASELINE[2],
    );
    
    pub const NONE: Self = Self(0);
    
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self::NONE
    }
    
    #[inline]
    pub const fn set_bold_if(&mut self, bold: bool) {
        if bold {
            self.0 |= Self::BOLD_MASK << Self::BOLD_OFFSET;
        }
    }
    
    #[inline]
    pub const fn set_bold(&mut self) {
        self.set_bold_if(true);
    }
    
    #[inline]
    pub const fn bold(mut self) -> Self {
        self.set_bold();
        self
    }
}

#[repr(C)]
pub struct Cell {
    pub chars: [u32; MAX_CHARS_PER_CELL],
    pub width: i8,
    pub attrs: Attrs,
    pub fg: Color,
    pub bg: Color,
}