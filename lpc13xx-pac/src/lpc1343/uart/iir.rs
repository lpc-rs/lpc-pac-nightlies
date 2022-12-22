#[doc = "Register `IIR` reader"]
pub struct R(crate::R<IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSTATUS` reader - Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
pub type INTSTATUS_R = crate::BitReader<INTSTATUS_A>;
#[doc = "Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUS_A {
    #[doc = "0: At least one interrupt is pending."]
    INT = 0,
    #[doc = "1: No interrupt is pending."]
    NOINT = 1,
}
impl From<INTSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl INTSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSTATUS_A {
        match self.bits {
            false => INTSTATUS_A::INT,
            true => INTSTATUS_A::NOINT,
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == INTSTATUS_A::INT
    }
    #[doc = "Checks if the value of the field is `NOINT`"]
    #[inline(always)]
    pub fn is_noint(&self) -> bool {
        *self == INTSTATUS_A::NOINT
    }
}
#[doc = "Field `INTID` reader - Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART Rx FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111)."]
pub type INTID_R = crate::FieldReader<u8, INTID_A>;
#[doc = "Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART Rx FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTID_A {
    #[doc = "3: 1 - Receive Line Status (RLS)."]
    RECEIVE = 3,
    #[doc = "2: 2a - Receive Data Available (RDA)."]
    RDA = 2,
    #[doc = "6: 2b - Character Time-out Indicator (CTI)."]
    CTIMEOUT = 6,
    #[doc = "1: 3 - THRE Interrupt."]
    THRE = 1,
    #[doc = "0: 4 - Modem interrupt."]
    MODEM = 0,
}
impl From<INTID_A> for u8 {
    #[inline(always)]
    fn from(variant: INTID_A) -> Self {
        variant as _
    }
}
impl INTID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTID_A> {
        match self.bits {
            3 => Some(INTID_A::RECEIVE),
            2 => Some(INTID_A::RDA),
            6 => Some(INTID_A::CTIMEOUT),
            1 => Some(INTID_A::THRE),
            0 => Some(INTID_A::MODEM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == INTID_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `RDA`"]
    #[inline(always)]
    pub fn is_rda(&self) -> bool {
        *self == INTID_A::RDA
    }
    #[doc = "Checks if the value of the field is `CTIMEOUT`"]
    #[inline(always)]
    pub fn is_ctimeout(&self) -> bool {
        *self == INTID_A::CTIMEOUT
    }
    #[doc = "Checks if the value of the field is `THRE`"]
    #[inline(always)]
    pub fn is_thre(&self) -> bool {
        *self == INTID_A::THRE
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == INTID_A::MODEM
    }
}
#[doc = "Field `FIFOEN` reader - These bits are equivalent to FCR\\[0\\]."]
pub type FIFOEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABEOINT` reader - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
pub type ABEOINT_R = crate::BitReader<bool>;
#[doc = "Field `ABTOINT` reader - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
pub type ABTOINT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\]
identifies an interrupt corresponding to the UART Rx FIFO. All other combinations of IER\\[3:1\\]
not listed below are reserved (100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - These bits are equivalent to FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> ABEOINT_R {
        ABEOINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> ABTOINT_R {
        ABTOINT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](index.html) module"]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir::R](R) reader structure"]
impl crate::Readable for IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
