use crate::Pos;


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub rows: u16,
    pub cols: u16,
}

impl Size {
    #[must_use]
    #[inline(always)]
    pub const fn new(rows: u16, cols: u16) -> Self {
        Self { rows, cols, }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn contains(self, pos: Pos) -> bool {
        pos.col < self.cols && pos.row < self.rows
    }
}