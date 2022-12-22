#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DELTACTS` reader - Set upon state change of input CTS. Cleared on a MSR read."]
pub type DELTACTS_R = crate::BitReader<DELTACTS_A>;
#[doc = "Set upon state change of input CTS. Cleared on a MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DELTACTS_A {
    #[doc = "0: No change detected on modem input CTS."]
    NO_STATE_CHANGE = 0,
    #[doc = "1: State change detected on modem input CTS."]
    STATE_CHANGE_DETECTE = 1,
}
impl From<DELTACTS_A> for bool {
    #[inline(always)]
    fn from(variant: DELTACTS_A) -> Self {
        variant as u8 != 0
    }
}
impl DELTACTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELTACTS_A {
        match self.bits {
            false => DELTACTS_A::NO_STATE_CHANGE,
            true => DELTACTS_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STATE_CHANGE`"]
    #[inline(always)]
    pub fn is_no_state_change(&self) -> bool {
        *self == DELTACTS_A::NO_STATE_CHANGE
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DELTACTS_A::STATE_CHANGE_DETECTE
    }
}
#[doc = "Field `DELTADSR` reader - Set upon state change of input DSR. Cleared on a MSR read."]
pub type DELTADSR_R = crate::BitReader<DELTADSR_A>;
#[doc = "Set upon state change of input DSR. Cleared on a MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DELTADSR_A {
    #[doc = "0: No change detected on modem input DSR."]
    NO_STATE_CHANGE = 0,
    #[doc = "1: State change detected on modem input DSR."]
    STATE_CHANGE_DETECTE = 1,
}
impl From<DELTADSR_A> for bool {
    #[inline(always)]
    fn from(variant: DELTADSR_A) -> Self {
        variant as u8 != 0
    }
}
impl DELTADSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELTADSR_A {
        match self.bits {
            false => DELTADSR_A::NO_STATE_CHANGE,
            true => DELTADSR_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STATE_CHANGE`"]
    #[inline(always)]
    pub fn is_no_state_change(&self) -> bool {
        *self == DELTADSR_A::NO_STATE_CHANGE
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DELTADSR_A::STATE_CHANGE_DETECTE
    }
}
#[doc = "Field `TERI` reader - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on a MSR read."]
pub type TERI_R = crate::BitReader<TERI_A>;
#[doc = "Trailing Edge RI. Set upon low to high transition of input RI. Cleared on a MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERI_A {
    #[doc = "0: No change detected on modem input, RI."]
    NO_STATE_CHANGE = 0,
    #[doc = "1: Low-to-high transition detected on RI."]
    LOW_TO_HIGH_TRANSITI = 1,
}
impl From<TERI_A> for bool {
    #[inline(always)]
    fn from(variant: TERI_A) -> Self {
        variant as u8 != 0
    }
}
impl TERI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERI_A {
        match self.bits {
            false => TERI_A::NO_STATE_CHANGE,
            true => TERI_A::LOW_TO_HIGH_TRANSITI,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STATE_CHANGE`"]
    #[inline(always)]
    pub fn is_no_state_change(&self) -> bool {
        *self == TERI_A::NO_STATE_CHANGE
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH_TRANSITI`"]
    #[inline(always)]
    pub fn is_low_to_high_transiti(&self) -> bool {
        *self == TERI_A::LOW_TO_HIGH_TRANSITI
    }
}
#[doc = "Field `DELTADCD` reader - Set upon state change of input DCD. Cleared on a MSR read."]
pub type DELTADCD_R = crate::BitReader<DELTADCD_A>;
#[doc = "Set upon state change of input DCD. Cleared on a MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DELTADCD_A {
    #[doc = "0: No change detected on modem input DCD."]
    NO_STATE_CHANGE = 0,
    #[doc = "1: State change detected on modem input DCD."]
    STATE_CHANGE_DETECTE = 1,
}
impl From<DELTADCD_A> for bool {
    #[inline(always)]
    fn from(variant: DELTADCD_A) -> Self {
        variant as u8 != 0
    }
}
impl DELTADCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELTADCD_A {
        match self.bits {
            false => DELTADCD_A::NO_STATE_CHANGE,
            true => DELTADCD_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STATE_CHANGE`"]
    #[inline(always)]
    pub fn is_no_state_change(&self) -> bool {
        *self == DELTADCD_A::NO_STATE_CHANGE
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DELTADCD_A::STATE_CHANGE_DETECTE
    }
}
#[doc = "Field `CTS` reader - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\]
in modem loopback mode."]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `DSR` reader - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\]
in modem loopback mode."]
pub type DSR_R = crate::BitReader<bool>;
#[doc = "Field `RI` reader - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\]
in modem loopback mode."]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `DCD` reader - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\]
in modem loopback mode."]
pub type DCD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set upon state change of input CTS. Cleared on a MSR read."]
    #[inline(always)]
    pub fn deltacts(&self) -> DELTACTS_R {
        DELTACTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set upon state change of input DSR. Cleared on a MSR read."]
    #[inline(always)]
    pub fn deltadsr(&self) -> DELTADSR_R {
        DELTADSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on a MSR read."]
    #[inline(always)]
    pub fn teri(&self) -> TERI_R {
        TERI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set upon state change of input DCD. Cleared on a MSR read."]
    #[inline(always)]
    pub fn deltadcd(&self) -> DELTADCD_R {
        DELTADCD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Modem status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module\n\nThe register is **modified** in some way after a read operation."]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
