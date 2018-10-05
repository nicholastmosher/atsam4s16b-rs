#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSR {
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
        let mut w = W { bits: 0 };
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Value of the field"]
pub struct TXCOMPR {
    bits: bool,
}
impl TXCOMPR {
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
pub struct RX_DATA_BK0R {
    bits: bool,
}
impl RX_DATA_BK0R {
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
pub struct RXSETUPR {
    bits: bool,
}
impl RXSETUPR {
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
pub struct STALLSENTR {
    bits: bool,
}
impl STALLSENTR {
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
pub struct TXPKTRDYR {
    bits: bool,
}
impl TXPKTRDYR {
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
pub struct FORCESTALLR {
    bits: bool,
}
impl FORCESTALLR {
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
pub struct RX_DATA_BK1R {
    bits: bool,
}
impl RX_DATA_BK1R {
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
pub struct DIRR {
    bits: bool,
}
impl DIRR {
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
#[doc = "Possible values of the field `EPTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPTYPER {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous OUT"]
    ISO_OUT,
    #[doc = "Bulk OUT"]
    BULK_OUT,
    #[doc = "Interrupt OUT"]
    INT_OUT,
    #[doc = "Isochronous IN"]
    ISO_IN,
    #[doc = "Bulk IN"]
    BULK_IN,
    #[doc = "Interrupt IN"]
    INT_IN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EPTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPTYPER::CTRL => 0,
            EPTYPER::ISO_OUT => 1,
            EPTYPER::BULK_OUT => 2,
            EPTYPER::INT_OUT => 3,
            EPTYPER::ISO_IN => 5,
            EPTYPER::BULK_IN => 6,
            EPTYPER::INT_IN => 7,
            EPTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPTYPER {
        match value {
            0 => EPTYPER::CTRL,
            1 => EPTYPER::ISO_OUT,
            2 => EPTYPER::BULK_OUT,
            3 => EPTYPER::INT_OUT,
            5 => EPTYPER::ISO_IN,
            6 => EPTYPER::BULK_IN,
            7 => EPTYPER::INT_IN,
            i => EPTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline]
    pub fn is_ctrl(&self) -> bool {
        *self == EPTYPER::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO_OUT`"]
    #[inline]
    pub fn is_iso_out(&self) -> bool {
        *self == EPTYPER::ISO_OUT
    }
    #[doc = "Checks if the value of the field is `BULK_OUT`"]
    #[inline]
    pub fn is_bulk_out(&self) -> bool {
        *self == EPTYPER::BULK_OUT
    }
    #[doc = "Checks if the value of the field is `INT_OUT`"]
    #[inline]
    pub fn is_int_out(&self) -> bool {
        *self == EPTYPER::INT_OUT
    }
    #[doc = "Checks if the value of the field is `ISO_IN`"]
    #[inline]
    pub fn is_iso_in(&self) -> bool {
        *self == EPTYPER::ISO_IN
    }
    #[doc = "Checks if the value of the field is `BULK_IN`"]
    #[inline]
    pub fn is_bulk_in(&self) -> bool {
        *self == EPTYPER::BULK_IN
    }
    #[doc = "Checks if the value of the field is `INT_IN`"]
    #[inline]
    pub fn is_int_in(&self) -> bool {
        *self == EPTYPER::INT_IN
    }
}
#[doc = r" Value of the field"]
pub struct DTGLER {
    bits: bool,
}
impl DTGLER {
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
pub struct EPEDSR {
    bits: bool,
}
impl EPEDSR {
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
pub struct RXBYTECNTR {
    bits: u16,
}
impl RXBYTECNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TXCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCOMPW<'a> {
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
pub struct _RX_DATA_BK0W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DATA_BK0W<'a> {
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
#[doc = r" Proxy"]
pub struct _RXSETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSETUPW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STALLSENTW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLSENTW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXPKTRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPKTRDYW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FORCESTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCESTALLW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_DATA_BK1W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DATA_BK1W<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPTYPE`"]
pub enum EPTYPEW {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous OUT"]
    ISO_OUT,
    #[doc = "Bulk OUT"]
    BULK_OUT,
    #[doc = "Interrupt OUT"]
    INT_OUT,
    #[doc = "Isochronous IN"]
    ISO_IN,
    #[doc = "Bulk IN"]
    BULK_IN,
    #[doc = "Interrupt IN"]
    INT_IN,
}
impl EPTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPTYPEW::CTRL => 0,
            EPTYPEW::ISO_OUT => 1,
            EPTYPEW::BULK_OUT => 2,
            EPTYPEW::INT_OUT => 3,
            EPTYPEW::ISO_IN => 5,
            EPTYPEW::BULK_IN => 6,
            EPTYPEW::INT_IN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPTYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Control"]
    #[inline]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(EPTYPEW::CTRL)
    }
    #[doc = "Isochronous OUT"]
    #[inline]
    pub fn iso_out(self) -> &'a mut W {
        self.variant(EPTYPEW::ISO_OUT)
    }
    #[doc = "Bulk OUT"]
    #[inline]
    pub fn bulk_out(self) -> &'a mut W {
        self.variant(EPTYPEW::BULK_OUT)
    }
    #[doc = "Interrupt OUT"]
    #[inline]
    pub fn int_out(self) -> &'a mut W {
        self.variant(EPTYPEW::INT_OUT)
    }
    #[doc = "Isochronous IN"]
    #[inline]
    pub fn iso_in(self) -> &'a mut W {
        self.variant(EPTYPEW::ISO_IN)
    }
    #[doc = "Bulk IN"]
    #[inline]
    pub fn bulk_in(self) -> &'a mut W {
        self.variant(EPTYPEW::BULK_IN)
    }
    #[doc = "Interrupt IN"]
    #[inline]
    pub fn int_in(self) -> &'a mut W {
        self.variant(EPTYPEW::INT_IN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTGLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DTGLEW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPEDSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPEDSW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXBYTECNTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBYTECNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline]
    pub fn txcomp(&self) -> TXCOMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXCOMPR { bits }
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline]
    pub fn rx_data_bk0(&self) -> RX_DATA_BK0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_DATA_BK0R { bits }
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline]
    pub fn rxsetup(&self) -> RXSETUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXSETUPR { bits }
    }
    #[doc = "Bit 3 - Stall Sent"]
    #[inline]
    pub fn stallsent(&self) -> STALLSENTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STALLSENTR { bits }
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline]
    pub fn txpktrdy(&self) -> TXPKTRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXPKTRDYR { bits }
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline]
    pub fn forcestall(&self) -> FORCESTALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FORCESTALLR { bits }
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline]
    pub fn rx_data_bk1(&self) -> RX_DATA_BK1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_DATA_BK1R { bits }
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints)"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIRR { bits }
    }
    #[doc = "Bits 8:10 - Endpoint Type"]
    #[inline]
    pub fn eptype(&self) -> EPTYPER {
        EPTYPER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Data Toggle"]
    #[inline]
    pub fn dtgle(&self) -> DTGLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTGLER { bits }
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline]
    pub fn epeds(&self) -> EPEDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPEDSR { bits }
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO"]
    #[inline]
    pub fn rxbytecnt(&self) -> RXBYTECNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RXBYTECNTR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline]
    pub fn txcomp(&mut self) -> _TXCOMPW {
        _TXCOMPW { w: self }
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline]
    pub fn rx_data_bk0(&mut self) -> _RX_DATA_BK0W {
        _RX_DATA_BK0W { w: self }
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline]
    pub fn rxsetup(&mut self) -> _RXSETUPW {
        _RXSETUPW { w: self }
    }
    #[doc = "Bit 3 - Stall Sent"]
    #[inline]
    pub fn stallsent(&mut self) -> _STALLSENTW {
        _STALLSENTW { w: self }
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline]
    pub fn txpktrdy(&mut self) -> _TXPKTRDYW {
        _TXPKTRDYW { w: self }
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline]
    pub fn forcestall(&mut self) -> _FORCESTALLW {
        _FORCESTALLW { w: self }
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline]
    pub fn rx_data_bk1(&mut self) -> _RX_DATA_BK1W {
        _RX_DATA_BK1W { w: self }
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints)"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bits 8:10 - Endpoint Type"]
    #[inline]
    pub fn eptype(&mut self) -> _EPTYPEW {
        _EPTYPEW { w: self }
    }
    #[doc = "Bit 11 - Data Toggle"]
    #[inline]
    pub fn dtgle(&mut self) -> _DTGLEW {
        _DTGLEW { w: self }
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline]
    pub fn epeds(&mut self) -> _EPEDSW {
        _EPEDSW { w: self }
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO"]
    #[inline]
    pub fn rxbytecnt(&mut self) -> _RXBYTECNTW {
        _RXBYTECNTW { w: self }
    }
}
