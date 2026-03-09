use std::ops::*;


#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifier(u32);

impl Modifier {
    pub const  NONE: Self = Self(0x00);
    pub const SHIFT: Self = Self(0x01);
    pub const   ALT: Self = Self(0x02);
    pub const  CTRL: Self = Self(0x04);
    pub const   ALL: Self = Self(0x07);
    
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self::NONE
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn shift(mut self) -> Self {
        self.or_assign(Self::SHIFT);
        self
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn alt(mut self) -> Self {
        self.or_assign(Self::ALT);
        self
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn ctrl(mut self) -> Self {
        self.or_assign(Self::CTRL);
        self
    }
    
    #[inline(always)]
    pub const fn or_assign(&mut self, modifier: Self) {
        self.0 |= modifier.0;
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn or(mut self, modifier: Self) -> Self {
        self.or_assign(modifier);
        self
    }
    
    #[inline(always)]
    pub const fn and_assign(&mut self, modifier: Self) {
        self.0 &= modifier.0;
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn and(mut self, modifier: Self) -> Self {
        self.and_assign(modifier);
        self
    }
    
    #[inline(always)]
    pub const fn xor_assign(&mut self, modifier: Self) {
        self.0 ^= modifier.0;
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn xor(mut self, modifier: Self) -> Self {
        self.xor_assign(modifier);
        self
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn union(modifiers: &[Self]) -> Self {
        let mut builder = Self::NONE;
        let mut index = 0;
        while index < modifiers.len() {
            builder.or_assign(modifiers[index]);
            index += 1;
        }
        builder
    }
}

impl BitOrAssign<Self> for Modifier {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.or_assign(rhs);
    }
}

impl BitOr<Self> for Modifier {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitAndAssign<Self> for Modifier {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.and_assign(rhs);
    }
}

impl BitAnd<Self> for Modifier {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitXorAssign<Self> for Modifier {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.xor_assign(rhs);
    }
}

impl BitXor<Self> for Modifier {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

#[repr(i32)]
pub enum Key {
    None,
    Enter,
    Tab,
    Backspace,
    Esc,
    Up,
    Down,
    Left,
    Right,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Function0 = 256,
    FunctionMax = Key::Function0 as i32 + 255,
    KeyPad0,
    KeyPad1,
    KeyPad2,
    KeyPad3,
    KeyPad4,
    KeyPad5,
    KeyPad6,
    KeyPad7,
    KeyPad8,
    KeyPad9,
    KeyPadMult,
    KeyPadPlus,
    KeyPadComma,
    KeyPadMinus,
    KeyPadPeriod,
    KeyPadDivide,
    KeyPadEnter,
    KeyPadEqual,
    Max,
}