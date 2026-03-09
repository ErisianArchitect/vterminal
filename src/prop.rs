use crate::bindings;


#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Prop {
    CursorVisible(bool) = 1,
    CursorBlink(bool),
    AltScreen(bool),
    Title(Box<str>),
    IconName(Box<str>),
    Reverse(bool),
    CursorShape(i32),
    Mouse(i32),
    FocusReport(bool),
}

impl Prop {
    pub(crate) unsafe fn from_vterm(prop: bindings::VTermProp, value: bindings::VTermValue) -> Self {
        
        unimplemented!()
    }
}