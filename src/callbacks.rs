// use ::core::ffi::c_void;
// use crate::{Cell, Pos, Rect};

use std::{ffi::{c_char, c_void}, sync::Mutex};

use crate::Context;

// type UserPtr = *mut c_void;
// // TODO
// type Prop = ();
// // TODO
// type Value = ();

// unsafe extern "C" {
//     fn damage(rect: Rect, user: UserPtr) -> i32;
//     fn moverect(dest: Rect, src: Rect, user: UserPtr) -> i32;
//     fn movecursor(new_pos: Pos, old_pos: Pos, visible: i32, user: UserPtr) -> i32;
//     fn settermprop(prop: Prop, value: Value, user: UserPtr) -> i32;
// }

// #[repr(C)]
// #[derive(Debug, Clone)]
// struct VTermScreenCallbacks {
//     pub damage: callback!(fn(Rect, UserPtr) -> i32),
//     pub moverect: callback!(fn(Rect, Rect, UserPtr) -> i32),
//     pub movecursor: callback!(fn(Pos, Pos, i32, UserPtr) -> i32),
//     pub settermprop: callback!(fn(Prop, Value, UserPtr) -> i32),
//     pub bell: callback!(fn(UserPtr) -> i32),
//     pub resize: callback!(fn(i32, i32, UserPtr) -> i32),
//     pub sb_pushline: callback!(fn(i32, *const Cell, UserPtr) -> i32),
//     pub sb_popline: callback!(fn(i32, *const Cell, UserPtr) -> i32),
//     pub sb_clear: callback!(fn(UserPtr) -> i32),
// }

// pub trait ScreenHandler {
//     fn damage(&mut self, rect: Rect) -> bool;
//     fn move_rect(&mut self, dest: Rect, src: Rect) -> bool;
//     fn move_cursor(&mut self, new_pos: Pos, old_pos: Pos, visible: bool) -> bool;
//     fn set_term_property(&mut self, prop: Prop, value: Value) -> bool;
//     fn bell(&mut self) -> bool;
//     fn resize(&mut self, rows: u32, cols: u32) -> bool;
//     fn sb_push_line(&mut self, cells: &[Cell]) -> bool;
//     fn sb_pop_line(&mut self, cells: &[Cell]) -> bool;
//     fn sb_clear(&mut self) -> bool;
// }

unsafe extern "C" fn output_callback<User: Sized + 'static>(s: *const c_char, len: usize, user: &mut User) {
    
}