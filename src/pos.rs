
/// Equivalent of VTermPos. Uses `u32` instead of `i32`. Values less than or equal to `i32::MAX` are expected.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub row: u16,
    pub col: u16,
}

impl Pos {
    #[must_use]
    #[inline(always)]
    pub const fn new(row: u16, col: u16) -> Self {
        Self {
            row,
            col,
        }
    }
    
    
    #[must_use]
    #[inline(always)]
    pub const fn screen_flow_cmp(lhs: Self, rhs: Self) -> i32 {
        if lhs.row == rhs.row {
            lhs.col as i32 - rhs.col as i32
        } else {
            lhs.row as i32 - rhs.row as i32
        }
    }
}