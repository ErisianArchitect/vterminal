
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawStr {
    ptr: *const u8,
    str_size: usize,
}

impl RawStr {
    #[inline(always)]
    pub unsafe fn get(&self) -> &str {
        unsafe { ::core::mem::transmute(*self) }
    }
}