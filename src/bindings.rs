#![allow(warnings)]

use std::mem::transmute;

use crate::{Pos, StringFragment};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl VTermColor {
    #[must_use]
    #[inline(always)]
    pub const fn unbind(self) -> crate::Color {
        // SAFETY: crate::Color has the same layout and bit representation.
        unsafe { transmute(self) }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn as_unbound(&self) -> &crate::Color {
        unsafe { transmute(self) }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn as_unbound_mut(&mut self) -> &mut crate::Color {
        unsafe { transmute(self) }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn bind(color: crate::Color) -> Self {
        // SAFETY: crate::Color has the same layout and bit representation.
        unsafe { transmute(color) }
    }
}

impl VTermPos {
    #[must_use]
    #[inline(always)]
    pub const fn from_pos(pos: Pos) -> Self {
        Self {
            row: pos.row as i32,
            col: pos.col as i32,
        }
    }
}

impl VTermStringFragment {
    #[must_use]
    #[inline(always)]
    pub fn unbind(&self) -> StringFragment<'_> {
        StringFragment::from_vterm(self)
    }
}