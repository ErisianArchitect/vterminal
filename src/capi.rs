use crate::{Cell, Key, Modifier, Pos};

pub enum VTermScreenT {}
pub enum VTermT {}
// pub type VPtr = *mut VTermT;
// pub type VScreenPtr = *mut VTermScreenT;
pub type VPtr = ::core::ptr::NonNull<VTermT>;
pub type VScreenPtr = ::core::ptr::NonNull<VTermScreenT>;

unsafe extern "C" {
    pub fn vterm_new(rows: libc::c_int, cols: libc::c_int) -> Option<VPtr>;
    pub fn vterm_free(vt: VPtr);
    
    // We don't need vterm_get_utf8 because this wrapper always uses UTF-8 mode.
    // fn vterm_get_utf8(vt: VPtr) -> i32;
    pub fn vterm_set_utf8(vt: VPtr, is_utf8: i32);
    
    pub fn vterm_get_size(vt: VPtr, rows: &mut u32, cols: &mut u32);
    pub fn vterm_set_size(vt: VPtr, rows: u32, cols: u32);
    
    pub fn vterm_input_write(vt: VPtr, byte_ptr: *const u8, len: usize) -> usize;
    
    pub fn vterm_keyboard_unichar(vt: VPtr, c: u32, modifier: Modifier);
    pub fn vterm_keyboard_key(vt: VPtr, key: Key, modifier: Modifier);
    
    pub fn vterm_keyboard_start_paste(vt: VPtr);
    pub fn vterm_keyboard_end_paste(vt: VPtr);
    
    pub fn vterm_mouse_move(vt: VPtr, row: u32, col: u32, modifier: Modifier);
    pub fn vterm_mouse_button(vt: VPtr, button: i32, pressed: bool, modifier: Modifier);
    
    pub fn vterm_obtain_screen(vt: VPtr) -> VScreenPtr;
    pub fn vterm_screen_reset(screen: VScreenPtr, hard: i32);
    pub fn vterm_screen_get_cell(screen: VScreenPtr, pos: Pos, cell: *mut Cell) -> i32;
}