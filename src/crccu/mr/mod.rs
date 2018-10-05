#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct COMPARER {
    bits: bool,
}
impl COMPARER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYPER {
    #[doc = "Polynom 0x04C11DB7"]
    CCITT8023,
    #[doc = "Polynom 0x1EDC6F41"]
    CASTAGNOLI,
    #[doc = "Polynom 0x1021"]
    CCITT16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTYPER::CCITT8023 => 0,
            PTYPER::CASTAGNOLI => 1,
            PTYPER::CCITT16 => 2,
            PTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTYPER {
        match value {
            0 => PTYPER::CCITT8023,
            1 => PTYPER::CASTAGNOLI,
            2 => PTYPER::CCITT16,
            i => PTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CCITT8023`"]
    #[inline]
    pub fn is_ccitt8023(&self) -> bool {
        *self == PTYPER::CCITT8023
    }
    #[doc = "Checks if the value of the field is `CASTAGNOLI`"]
    #[inline]
    pub fn is_castagnoli(&self) -> bool {
        *self == PTYPER::CASTAGNOLI
    }
    #[doc = "Checks if the value of the field is `CCITT16`"]
    #[inline]
    pub fn is_ccitt16(&self) -> bool {
        *self == PTYPER::CCITT16
    }
}
#[doc = r" Value of the field"]
pub struct DIVIDERR {
    bits: u8,
}
impl DIVIDERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTYPE`"]
pub enum PTYPEW {
    #[doc = "Polynom 0x04C11DB7"]
    CCITT8023,
    #[doc = "Polynom 0x1EDC6F41"]
    CASTAGNOLI,
    #[doc = "Polynom 0x1021"]
    CCITT16,
}
impl PTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTYPEW::CCITT8023 => 0,
            PTYPEW::CASTAGNOLI => 1,
            PTYPEW::CCITT16 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _PTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Polynom 0x04C11DB7"]
    #[inline]
    pub fn ccitt8023(self) -> &'a mut W {
        self.variant(PTYPEW::CCITT8023)
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline]
    pub fn castagnoli(self) -> &'a mut W {
        self.variant(PTYPEW::CASTAGNOLI)
    }
    #[doc = "Polynom 0x1021"]
    #[inline]
    pub fn ccitt16(self) -> &'a mut W {
        self.variant(PTYPEW::CCITT16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIVIDERW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVIDERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CRC Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline]
    pub fn compare(&self) -> COMPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMPARER { bits }
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline]
    pub fn ptype(&self) -> PTYPER {
        PTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline]
    pub fn divider(&self) -> DIVIDERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVIDERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - CRC Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline]
    pub fn compare(&mut self) -> _COMPAREW {
        _COMPAREW { w: self }
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline]
    pub fn ptype(&mut self) -> _PTYPEW {
        _PTYPEW { w: self }
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline]
    pub fn divider(&mut self) -> _DIVIDERW {
        _DIVIDERW { w: self }
    }
}
