use core::{ffi::c_char, mem::transmute};
use crate::lowlevel::*;

use crate::bindings;

#[repr(u8)]
pub enum FragmentType {
    Middle = 0,
    Initial = 1,
    Final = 2,
    Full = 3,
}

#[repr(C)]
struct VTermFragmentHelper {
    string: RawStr,
    frag_type: FragmentType,
}

impl VTermFragmentHelper {
    #[must_use]
    #[inline(always)]
    fn from_vterm(vterm_fragment: &bindings::VTermStringFragment) -> &Self {
        unsafe { transmute(vterm_fragment) }
    }
    
    #[must_use]
    #[inline]
    fn as_str(&self) -> &str {
        unsafe {
            self.string.get()
        }
    }
    
    #[must_use]
    #[inline]
    fn to_safe(&self) -> StringFragment<'_> {
        match self.frag_type {
            FragmentType::Middle => StringFragment::Middle(self.as_str()),
            FragmentType::Initial => StringFragment::Initial(self.as_str()),
            FragmentType::Final => StringFragment::Final(self.as_str()),
            FragmentType::Full => StringFragment::Full(self.as_str()),
        }
    }
}

#[repr(C, u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StringFragment<'a> {
    Middle(&'a str) = 0,
    Initial(&'a str) = 1,
    Final(&'a str) = 2,
    Full(&'a str) = 3,
}

impl<'a> StringFragment<'a> {
    #[must_use]
    #[inline]
    pub(crate) fn from_vterm(vterm_fragment: &'a bindings::VTermStringFragment) -> Self {
        VTermFragmentHelper::from_vterm(vterm_fragment).to_safe()
    }
    
    #[must_use]
    #[inline]
    pub const fn fragment_type(&self) -> FragmentType {
        match self {
            StringFragment::Middle(_) => FragmentType::Middle,
            StringFragment::Initial(_) => FragmentType::Initial,
            StringFragment::Final(_) => FragmentType::Final,
            StringFragment::Full(_) => FragmentType::Full,
        }
    }
}