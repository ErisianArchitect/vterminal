mod bindings;
mod callbacks;
mod cell;
mod color;
mod fragment;
mod key;
mod lowlevel;
mod pos;
mod prop;
mod rect;
mod size;
mod user_context;

use std::{marker::PhantomData, mem::{MaybeUninit, transmute}, ptr::NonNull, sync::Mutex};

use callbacks::*;
pub use cell::*;
pub use color::*;
pub use fragment::*;
pub use key::*;
pub use pos::*;
pub use prop::*;
pub use rect::*;
pub use size::*;
pub use user_context::*;

#[macro_export]
macro_rules! c_callback_type {
    (Option: fn($($arg_name:ident : $type:ty),*$(,)?) $(-> $return:ty)?) => {
        Option<$crate::c_callback_type!(fn($($arg_name : $type),*) $(-> $return)?)>
    };
    (Option: fn($($type:ty),*$(,)?) $(-> $return:ty)?) => {
        Option<$crate::c_callback_type!(fn($($type),*) $(-> $return)?)>
    };
    (fn($($arg_name:ident : $type:ty),*$(,)?) $(-> $return:ty)?) => {
        unsafe extern "C" fn($($arg_name : $type),*) $(-> $return)?
    };
    (fn($($type:ty),*$(,)?) $(-> $return:ty)?) => {
        unsafe extern "C" fn($($type),*) $(-> $return)?
    };
}

#[macro_export]
macro_rules! debug_assert_le_i32_max {
    ($value:expr) => {
        debug_assert!($value <= i32::MAX as u32, concat!(stringify!($value), "Was not less-or-equal to i32::MAX."));
    };
    ($value:expr, $message:literal) => {
        debug_assert!($value <= i32::MAX as u32, $message);
    };
}

#[must_use]
#[inline(always)]
pub const fn cbool(value: bool) -> i32 {
    [0, 1][value as usize]
}

#[must_use]
#[inline(always)]
pub const fn from_cbool(value: i32) -> bool {
    value != 0
}

type KeyTy = bindings::VTermKey;

// VTermHandle disambiguates handle types into Owned, Immut<'a>, and Mut<'a>
struct VTermHandle<Marker> {
    vt: NonNull<bindings::VTerm>,
    screen: NonNull<bindings::VTermScreen>,
    state: NonNull<bindings::VTermState>,
    _phantom: PhantomData<(Marker,)>,
}

pub struct VTerm {
    vt: NonNull<bindings::VTerm>,
    screen: NonNull<bindings::VTermScreen>,
    state: NonNull<bindings::VTermState>,
    // context: Box<Mutex<Context>>,
}

impl VTerm {
    #[must_use]
    #[inline(always)]
    fn vt(&self) -> *mut bindings::VTerm {
        self.vt.as_ptr()
    }
    
    fn with_init(mut self) -> Self {
        // Set to UTF-8 mode for compatibility with Rust.
        unsafe { bindings::vterm_set_utf8(self.vt.as_ptr().cast(), 1); }
        self.reset_screen(true);
        self
    }
    
    #[must_use]
    #[inline]
    pub fn new(rows: u16, cols: u16) -> Self {
        let ptr = unsafe { bindings::vterm_new(rows as i32, cols as i32) };
        let Some(vt) = NonNull::new(ptr) else {
            panic!("Failed to initialize vterm.");
        };
        // obtain_screen creates screen and state, but obtain_state only creates state.
        // Order: obtain_screen -> obtain_state
        // SAFETY: Since vt is non-null, these calls are guaranteed to succeed.
        let screen = unsafe { NonNull::new_unchecked(bindings::vterm_obtain_screen(vt.as_ptr())) };
        let state = unsafe { NonNull::new_unchecked(bindings::vterm_obtain_state(vt.as_ptr())) };
        Self {
            vt,
            screen,
            state,
        }.with_init()
    }
    
    #[must_use]
    #[inline]
    pub fn get_size(&self) -> Size {
        let (mut rows, mut cols) = (0i32, 0i32);
        unsafe { bindings::vterm_get_size(self.vt(), &mut rows, &mut cols); }
        Size { rows: rows as u16, cols: cols as u16 }
    }
    
    #[inline]
    pub fn set_size(&mut self, size: Size) {
        unsafe { bindings::vterm_set_size(self.vt(), size.rows as i32, size.cols as i32); }
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
        unsafe { bindings::vterm_input_write(self.vt(), bytes_ptr.cast(), bytes.len()) }
    }
    
    pub fn paste(&mut self, content: &str) {
        unsafe {
            bindings::vterm_keyboard_start_paste(self.vt());
            for chr in content.chars() {
                self.keyboard_char(chr, Modifier::NONE);
            }
            bindings::vterm_keyboard_end_paste(self.vt());
        }
    }
    
    #[inline]
    pub fn keyboard_char(&mut self, chr: char, modifier: Modifier) {
        unsafe { bindings::vterm_keyboard_unichar(self.vt(), chr as u32, ::core::mem::transmute(modifier)); }
    }
    
    #[inline]
    pub fn keyboard_key(&mut self, key: Key, modifier: Modifier) {
        unsafe {
            bindings::vterm_keyboard_key(
                self.vt.as_ptr().cast(),
                transmute(key),
                transmute(modifier),
            );
        }
    }
    
    #[inline]
    pub fn mouse_move(&mut self, pos: Pos, modifier: Modifier) {
        unsafe { bindings::vterm_mouse_move(self.vt(), pos.row as i32, pos.col as i32, transmute(modifier)); }
    }
    
    #[inline]
    pub fn mouse_button(&mut self, button: i32, pressed: bool, modifier: Modifier) {
        unsafe { bindings::vterm_mouse_button(self.vt(), button, pressed, transmute(modifier)); }
    }
    
    #[inline]
    pub fn reset_screen(&mut self, hard: bool) {
        unsafe { bindings::vterm_screen_reset(self.screen.as_ptr(), cbool(hard)); }
    }
    
    #[inline(always)]
    pub unsafe fn unsafe_read_cell_into(&self, pos: Pos, cell: *mut Cell) -> bool {
        from_cbool(unsafe { bindings::vterm_screen_get_cell(
            self.screen.as_ptr(),
            bindings::VTermPos {
                col: pos.col as i32,
                row: pos.row as i32
            },
            cell.cast(),
        )})
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
        unsafe { bindings::vterm_free(self.vt()); }
    }
}