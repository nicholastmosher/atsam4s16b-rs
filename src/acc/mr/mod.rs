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
#[doc = "Possible values of the field `SELMINUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELMINUSR {
    #[doc = "SelectTS"]
    TS,
    #[doc = "Select ADVREF"]
    ADVREF,
    #[doc = "Select DAC0"]
    DAC0,
    #[doc = "Select DAC1"]
    DAC1,
    #[doc = "Select AD0"]
    AD0,
    #[doc = "Select AD1"]
    AD1,
    #[doc = "Select AD2"]
    AD2,
    #[doc = "Select AD3"]
    AD3,
}
impl SELMINUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELMINUSR::TS => 0,
            SELMINUSR::ADVREF => 1,
            SELMINUSR::DAC0 => 2,
            SELMINUSR::DAC1 => 3,
            SELMINUSR::AD0 => 4,
            SELMINUSR::AD1 => 5,
            SELMINUSR::AD2 => 6,
            SELMINUSR::AD3 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELMINUSR {
        match value {
            0 => SELMINUSR::TS,
            1 => SELMINUSR::ADVREF,
            2 => SELMINUSR::DAC0,
            3 => SELMINUSR::DAC1,
            4 => SELMINUSR::AD0,
            5 => SELMINUSR::AD1,
            6 => SELMINUSR::AD2,
            7 => SELMINUSR::AD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline]
    pub fn is_ts(&self) -> bool {
        *self == SELMINUSR::TS
    }
    #[doc = "Checks if the value of the field is `ADVREF`"]
    #[inline]
    pub fn is_advref(&self) -> bool {
        *self == SELMINUSR::ADVREF
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline]
    pub fn is_dac0(&self) -> bool {
        *self == SELMINUSR::DAC0
    }
    #[doc = "Checks if the value of the field is `DAC1`"]
    #[inline]
    pub fn is_dac1(&self) -> bool {
        *self == SELMINUSR::DAC1
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline]
    pub fn is_ad0(&self) -> bool {
        *self == SELMINUSR::AD0
    }
    #[doc = "Checks if the value of the field is `AD1`"]
    #[inline]
    pub fn is_ad1(&self) -> bool {
        *self == SELMINUSR::AD1
    }
    #[doc = "Checks if the value of the field is `AD2`"]
    #[inline]
    pub fn is_ad2(&self) -> bool {
        *self == SELMINUSR::AD2
    }
    #[doc = "Checks if the value of the field is `AD3`"]
    #[inline]
    pub fn is_ad3(&self) -> bool {
        *self == SELMINUSR::AD3
    }
}
#[doc = "Possible values of the field `SELPLUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELPLUSR {
    #[doc = "Select AD0"]
    AD0,
    #[doc = "Select AD1"]
    AD1,
    #[doc = "Select AD2"]
    AD2,
    #[doc = "Select AD3"]
    AD3,
    #[doc = "Select AD4"]
    AD4,
    #[doc = "Select AD5"]
    AD5,
    #[doc = "Select AD6"]
    AD6,
    #[doc = "Select AD7"]
    AD7,
}
impl SELPLUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELPLUSR::AD0 => 0,
            SELPLUSR::AD1 => 1,
            SELPLUSR::AD2 => 2,
            SELPLUSR::AD3 => 3,
            SELPLUSR::AD4 => 4,
            SELPLUSR::AD5 => 5,
            SELPLUSR::AD6 => 6,
            SELPLUSR::AD7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELPLUSR {
        match value {
            0 => SELPLUSR::AD0,
            1 => SELPLUSR::AD1,
            2 => SELPLUSR::AD2,
            3 => SELPLUSR::AD3,
            4 => SELPLUSR::AD4,
            5 => SELPLUSR::AD5,
            6 => SELPLUSR::AD6,
            7 => SELPLUSR::AD7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AD0`"]
    #[inline]
    pub fn is_ad0(&self) -> bool {
        *self == SELPLUSR::AD0
    }
    #[doc = "Checks if the value of the field is `AD1`"]
    #[inline]
    pub fn is_ad1(&self) -> bool {
        *self == SELPLUSR::AD1
    }
    #[doc = "Checks if the value of the field is `AD2`"]
    #[inline]
    pub fn is_ad2(&self) -> bool {
        *self == SELPLUSR::AD2
    }
    #[doc = "Checks if the value of the field is `AD3`"]
    #[inline]
    pub fn is_ad3(&self) -> bool {
        *self == SELPLUSR::AD3
    }
    #[doc = "Checks if the value of the field is `AD4`"]
    #[inline]
    pub fn is_ad4(&self) -> bool {
        *self == SELPLUSR::AD4
    }
    #[doc = "Checks if the value of the field is `AD5`"]
    #[inline]
    pub fn is_ad5(&self) -> bool {
        *self == SELPLUSR::AD5
    }
    #[doc = "Checks if the value of the field is `AD6`"]
    #[inline]
    pub fn is_ad6(&self) -> bool {
        *self == SELPLUSR::AD6
    }
    #[doc = "Checks if the value of the field is `AD7`"]
    #[inline]
    pub fn is_ad7(&self) -> bool {
        *self == SELPLUSR::AD7
    }
}
#[doc = "Possible values of the field `ACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACENR {
    #[doc = "Analog Comparator Disabled."]
    DIS,
    #[doc = "Analog Comparator Enabled."]
    EN,
}
impl ACENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACENR::DIS => false,
            ACENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACENR {
        match value {
            false => ACENR::DIS,
            true => ACENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ACENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ACENR::EN
    }
}
#[doc = "Possible values of the field `EDGETYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGETYPR {
    #[doc = "only rising edge of comparator output"]
    RISING,
    #[doc = "falling edge of comparator output"]
    FALLING,
    #[doc = "any edge of comparator output"]
    ANY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EDGETYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGETYPR::RISING => 0,
            EDGETYPR::FALLING => 1,
            EDGETYPR::ANY => 2,
            EDGETYPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGETYPR {
        match value {
            0 => EDGETYPR::RISING,
            1 => EDGETYPR::FALLING,
            2 => EDGETYPR::ANY,
            i => EDGETYPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == EDGETYPR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == EDGETYPR::FALLING
    }
    #[doc = "Checks if the value of the field is `ANY`"]
    #[inline]
    pub fn is_any(&self) -> bool {
        *self == EDGETYPR::ANY
    }
}
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Analog Comparator output is directly processed."]
    DIS,
    #[doc = "Analog Comparator output is inverted prior to being processed."]
    EN,
}
impl INVR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INVR::DIS => false,
            INVR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::DIS,
            true => INVR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == INVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == INVR::EN
    }
}
#[doc = "Possible values of the field `SELFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELFSR {
    #[doc = "the CF flag is used to drive the FAULT output."]
    CF,
    #[doc = "the output of the Analog Comparator flag is used to drive the FAULT output."]
    OUTPUT,
}
impl SELFSR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SELFSR::CF => false,
            SELFSR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELFSR {
        match value {
            false => SELFSR::CF,
            true => SELFSR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CF`"]
    #[inline]
    pub fn is_cf(&self) -> bool {
        *self == SELFSR::CF
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == SELFSR::OUTPUT
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "the FAULT output is tied to 0."]
    DIS,
    #[doc = "the FAULT output is driven by the signal defined by SELFS."]
    EN,
}
impl FER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FER::DIS => false,
            FER::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER {
        match value {
            false => FER::DIS,
            true => FER::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FER::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FER::EN
    }
}
#[doc = "Values that can be written to the field `SELMINUS`"]
pub enum SELMINUSW {
    #[doc = "SelectTS"]
    TS,
    #[doc = "Select ADVREF"]
    ADVREF,
    #[doc = "Select DAC0"]
    DAC0,
    #[doc = "Select DAC1"]
    DAC1,
    #[doc = "Select AD0"]
    AD0,
    #[doc = "Select AD1"]
    AD1,
    #[doc = "Select AD2"]
    AD2,
    #[doc = "Select AD3"]
    AD3,
}
impl SELMINUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELMINUSW::TS => 0,
            SELMINUSW::ADVREF => 1,
            SELMINUSW::DAC0 => 2,
            SELMINUSW::DAC1 => 3,
            SELMINUSW::AD0 => 4,
            SELMINUSW::AD1 => 5,
            SELMINUSW::AD2 => 6,
            SELMINUSW::AD3 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELMINUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELMINUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELMINUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SelectTS"]
    #[inline]
    pub fn ts(self) -> &'a mut W {
        self.variant(SELMINUSW::TS)
    }
    #[doc = "Select ADVREF"]
    #[inline]
    pub fn advref(self) -> &'a mut W {
        self.variant(SELMINUSW::ADVREF)
    }
    #[doc = "Select DAC0"]
    #[inline]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SELMINUSW::DAC0)
    }
    #[doc = "Select DAC1"]
    #[inline]
    pub fn dac1(self) -> &'a mut W {
        self.variant(SELMINUSW::DAC1)
    }
    #[doc = "Select AD0"]
    #[inline]
    pub fn ad0(self) -> &'a mut W {
        self.variant(SELMINUSW::AD0)
    }
    #[doc = "Select AD1"]
    #[inline]
    pub fn ad1(self) -> &'a mut W {
        self.variant(SELMINUSW::AD1)
    }
    #[doc = "Select AD2"]
    #[inline]
    pub fn ad2(self) -> &'a mut W {
        self.variant(SELMINUSW::AD2)
    }
    #[doc = "Select AD3"]
    #[inline]
    pub fn ad3(self) -> &'a mut W {
        self.variant(SELMINUSW::AD3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELPLUS`"]
pub enum SELPLUSW {
    #[doc = "Select AD0"]
    AD0,
    #[doc = "Select AD1"]
    AD1,
    #[doc = "Select AD2"]
    AD2,
    #[doc = "Select AD3"]
    AD3,
    #[doc = "Select AD4"]
    AD4,
    #[doc = "Select AD5"]
    AD5,
    #[doc = "Select AD6"]
    AD6,
    #[doc = "Select AD7"]
    AD7,
}
impl SELPLUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELPLUSW::AD0 => 0,
            SELPLUSW::AD1 => 1,
            SELPLUSW::AD2 => 2,
            SELPLUSW::AD3 => 3,
            SELPLUSW::AD4 => 4,
            SELPLUSW::AD5 => 5,
            SELPLUSW::AD6 => 6,
            SELPLUSW::AD7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELPLUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELPLUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELPLUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select AD0"]
    #[inline]
    pub fn ad0(self) -> &'a mut W {
        self.variant(SELPLUSW::AD0)
    }
    #[doc = "Select AD1"]
    #[inline]
    pub fn ad1(self) -> &'a mut W {
        self.variant(SELPLUSW::AD1)
    }
    #[doc = "Select AD2"]
    #[inline]
    pub fn ad2(self) -> &'a mut W {
        self.variant(SELPLUSW::AD2)
    }
    #[doc = "Select AD3"]
    #[inline]
    pub fn ad3(self) -> &'a mut W {
        self.variant(SELPLUSW::AD3)
    }
    #[doc = "Select AD4"]
    #[inline]
    pub fn ad4(self) -> &'a mut W {
        self.variant(SELPLUSW::AD4)
    }
    #[doc = "Select AD5"]
    #[inline]
    pub fn ad5(self) -> &'a mut W {
        self.variant(SELPLUSW::AD5)
    }
    #[doc = "Select AD6"]
    #[inline]
    pub fn ad6(self) -> &'a mut W {
        self.variant(SELPLUSW::AD6)
    }
    #[doc = "Select AD7"]
    #[inline]
    pub fn ad7(self) -> &'a mut W {
        self.variant(SELPLUSW::AD7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACEN`"]
pub enum ACENW {
    #[doc = "Analog Comparator Disabled."]
    DIS,
    #[doc = "Analog Comparator Enabled."]
    EN,
}
impl ACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACENW::DIS => false,
            ACENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog Comparator Disabled."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACENW::DIS)
    }
    #[doc = "Analog Comparator Enabled."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ACENW::EN)
    }
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGETYP`"]
pub enum EDGETYPW {
    #[doc = "only rising edge of comparator output"]
    RISING,
    #[doc = "falling edge of comparator output"]
    FALLING,
    #[doc = "any edge of comparator output"]
    ANY,
}
impl EDGETYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGETYPW::RISING => 0,
            EDGETYPW::FALLING => 1,
            EDGETYPW::ANY => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGETYPW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGETYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGETYPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "only rising edge of comparator output"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGETYPW::RISING)
    }
    #[doc = "falling edge of comparator output"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGETYPW::FALLING)
    }
    #[doc = "any edge of comparator output"]
    #[inline]
    pub fn any(self) -> &'a mut W {
        self.variant(EDGETYPW::ANY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Analog Comparator output is directly processed."]
    DIS,
    #[doc = "Analog Comparator output is inverted prior to being processed."]
    EN,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::DIS => false,
            INVW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog Comparator output is directly processed."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(INVW::DIS)
    }
    #[doc = "Analog Comparator output is inverted prior to being processed."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(INVW::EN)
    }
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELFS`"]
pub enum SELFSW {
    #[doc = "the CF flag is used to drive the FAULT output."]
    CF,
    #[doc = "the output of the Analog Comparator flag is used to drive the FAULT output."]
    OUTPUT,
}
impl SELFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELFSW::CF => false,
            SELFSW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELFSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the CF flag is used to drive the FAULT output."]
    #[inline]
    pub fn cf(self) -> &'a mut W {
        self.variant(SELFSW::CF)
    }
    #[doc = "the output of the Analog Comparator flag is used to drive the FAULT output."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(SELFSW::OUTPUT)
    }
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE`"]
pub enum FEW {
    #[doc = "the FAULT output is tied to 0."]
    DIS,
    #[doc = "the FAULT output is driven by the signal defined by SELFS."]
    EN,
}
impl FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEW::DIS => false,
            FEW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the FAULT output is tied to 0."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FEW::DIS)
    }
    #[doc = "the FAULT output is driven by the signal defined by SELFS."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FEW::EN)
    }
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline]
    pub fn selminus(&self) -> SELMINUSR {
        SELMINUSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline]
    pub fn selplus(&self) -> SELPLUSR {
        SELPLUSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline]
    pub fn acen(&self) -> ACENR {
        ACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline]
    pub fn edgetyp(&self) -> EDGETYPR {
        EDGETYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline]
    pub fn selfs(&self) -> SELFSR {
        SELFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline]
    pub fn fe(&self) -> FER {
        FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline]
    pub fn selminus(&mut self) -> _SELMINUSW {
        _SELMINUSW { w: self }
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline]
    pub fn selplus(&mut self) -> _SELPLUSW {
        _SELPLUSW { w: self }
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline]
    pub fn acen(&mut self) -> _ACENW {
        _ACENW { w: self }
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline]
    pub fn edgetyp(&mut self) -> _EDGETYPW {
        _EDGETYPW { w: self }
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline]
    pub fn selfs(&mut self) -> _SELFSW {
        _SELFSW { w: self }
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline]
    pub fn fe(&mut self) -> _FEW {
        _FEW { w: self }
    }
}
