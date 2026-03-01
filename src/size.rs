
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub rows: u32,
    pub cols: u32,
}

impl Size {
    #[must_use]
    #[inline(always)]
    pub const fn new(rows: u32, cols: u32) -> Self {
        debug_assert!(rows <= i32::MAX as u32, "rows > i32::MAX");
        debug_assert!(cols <= i32::MAX as u32, "cols > i32::MAX");
        Self { rows, cols, }
    }
}