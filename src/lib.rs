mod capi;
mod cell;
mod color;
mod key;
mod pos;
mod rect;
mod size;

use std::{marker::PhantomData, mem::{ManuallyDrop, MaybeUninit}, ptr::NonNull};

use capi::*;
pub use cell::*;
pub use color::*;
pub use key::*;
pub use pos::*;
pub use rect::*;
pub use size::*;

#[macro_export]
macro_rules! debug_assert_le_i32_max {
    ($value:expr) => {
        debug_assert!($value <= i32::MAX as u32, concat!(stringify!($value), "Was not less-or-equal to i32::MAX."));
    };
    ($value:expr, $message:literal) => {
        debug_assert!($value <= i32::MAX as u32, $message);
    };
}

const C_BOOLS: [i32; 2] = [0, 1];

#[must_use]
#[inline(always)]
pub const fn cbool(value: bool) -> i32 {
    C_BOOLS[value as usize]
}

#[must_use]
#[inline(always)]
pub const fn from_cbool(value: i32) -> bool {
    value != 0
}

pub struct VTerm {
    vt: VPtr,
    screen: VScreenPtr,
}

pub struct Screen<'a> {
    ptr: VScreenPtr,
    term: VPtr,
    _phantom: PhantomData<&'a mut VTerm>,
}

impl VTerm {
    fn with_init(mut self) -> Self {
        // Set to UTF-8 mode for compatibility with Rust.
        unsafe { vterm_set_utf8(self.vt, 1); }
        self.reset_screen(true);
        self
    }
    
    #[must_use]
    #[inline]
    pub fn new(rows: u32, cols: u32) -> Self {
        debug_assert!(rows <= i32::MAX as u32 && cols <= i32::MAX as u32);
        let ptr = unsafe { vterm_new(rows as i32, cols as i32) };
        let Some(ptr) = ptr else {
            panic!("Failed to initialize vterm.");
        };
        Self {
            vt: ptr,
            screen: unsafe { vterm_obtain_screen(ptr) },
        }.with_init()
    }
    
    #[must_use]
    #[inline]
    pub fn get_size(&self) -> Size {
        let (mut rows, mut cols) = (0u32, 0u32);
        unsafe { vterm_get_size(self.vt, &mut rows, &mut cols); }
        Size { rows, cols }
    }
    
    #[inline]
    pub fn set_size(&mut self, size: Size) {
        debug_assert_le_i32_max!(size.rows);
        debug_assert_le_i32_max!(size.cols);
        unsafe { vterm_set_size(self.vt, size.rows, size.cols); }
    }
    
    #[inline]
    pub fn contains(&self, pos: Pos) -> bool {
        let size = self.get_size();
        pos.row < size.rows &&
        pos.col < size.cols
    }
    
    // not sure if this is expected to be used.
    // #[must_use]
    #[inline]
    pub fn write_input(&mut self, bytes: &[u8]) -> usize {
        let bytes_ptr = bytes.as_ptr();
        unsafe { vterm_input_write(self.vt, bytes_ptr, bytes.len()) }
    }
    
    #[inline]
    pub fn put_char(&mut self, chr: char, modifier: Modifier) {
        unsafe { vterm_keyboard_unichar(self.vt, chr as u32, modifier); }
    }
    
    #[inline]
    pub fn put_key(&mut self, key: Key, modifier: Modifier) {
        unsafe { vterm_keyboard_key(self.vt, key, modifier); }
    }
    
    pub fn paste(&mut self, content: &str) {
        unsafe {
            vterm_keyboard_start_paste(self.vt);
            for chr in content.chars() {
                self.put_char(chr, Modifier::NONE);
            }
            vterm_keyboard_end_paste(self.vt);
        }
    }
    
    #[inline]
    pub fn mouse_move(&mut self, pos: Pos, modifier: Modifier) {
        unsafe { vterm_mouse_move(self.vt, pos.row, pos.col, modifier); }
    }
    
    #[inline]
    pub fn mouse_button(&mut self, button: i32, pressed: bool, modifier: Modifier) {
        unsafe { vterm_mouse_button(self.vt, button, pressed, modifier); }
    }
    
    #[inline]
    pub fn reset_screen(&mut self, hard: bool) {
        unsafe { vterm_screen_reset(self.screen, cbool(hard)); }
    }
    
    #[inline(always)]
    unsafe fn unsafe_read_cell_into(&self, pos: Pos, cell: *mut Cell) -> bool {
        from_cbool(unsafe { vterm_screen_get_cell(self.screen, pos, cell) })
    }
    
    #[inline]
    pub fn read_cell_into(&self, pos: Pos, cell: &mut Cell) -> bool {
        unsafe { self.unsafe_read_cell_into(pos, cell) }
    }
    
    #[inline]
    pub fn get_cell(&self, pos: Pos) -> Option<Cell> {
        let mut cell = MaybeUninit::uninit();
        if unsafe { self.unsafe_read_cell_into(pos, cell.as_mut_ptr()) } {
            Some(unsafe { cell.assume_init() })
        } else {
            None
        }
    }
}

impl Drop for VTerm {
    #[inline]
    fn drop(&mut self) {
        unsafe { vterm_free(self.vt); }
    }
}