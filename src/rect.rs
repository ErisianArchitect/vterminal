use crate::{Size, debug_assert_le_i32_max, pos::Pos};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub start_row: u32,
    pub end_row: u32,
    pub start_col: u32,
    pub end_col: u32,
}

impl Rect {
    #[must_use]
    #[inline(always)]
    pub const fn new(
        start_row: u32,
        end_row: u32,
        start_col: u32,
        end_col: u32,
    ) -> Self {
        debug_assert_le_i32_max!(start_row);
        debug_assert_le_i32_max!(start_col);
        debug_assert_le_i32_max!(end_row);
        debug_assert_le_i32_max!(end_col);
        Self {
            start_row,
            end_row,
            start_col,
            end_col,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn from_min_max(min: Pos, max: Pos) -> Self {
        Self {
            start_row: min.row,
            start_col: min.col,
            end_row: max.row,
            end_col: max.col,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn from_min_size(min: Pos, size: Size) -> Self {
        Self {
            start_row: min.row,
            start_col: min.col,
            end_row: min.row + size.rows,
            end_col: min.col + size.cols,
        }
    }
    
    /// Equivalent of `vterm_rect_contains`.
    #[must_use]
    #[inline]
    pub const fn contains(self, pos: Pos) -> bool {
        pos.row >= self.start_row && pos.row < self.end_row &&
        pos.col >= self.start_col && pos.col < self.end_col
    }
    
    /// Equivalent of `vterm_rect_move`.
    #[inline]
    pub const fn translate(&mut self, delta: Pos) {
        self.start_row += delta.row; self.end_row += delta.row;
        self.start_col += delta.col; self.end_col += delta.col;
    }
}