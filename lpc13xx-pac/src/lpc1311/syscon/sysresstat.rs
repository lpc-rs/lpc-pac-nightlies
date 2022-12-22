#[doc = "Register `SYSRESSTAT` reader"]
pub struct R(crate::R<SYSRESSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSRESSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSRESSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSRESSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POR` reader - POR reset status"]
pub type POR_R = crate::BitReader<POR_A>;
#[doc = "POR reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: No POR detected"]
    NO_POR_DETECTED = 0,
    #[doc = "1: POR detected"]
    POR_DETECTED = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
impl POR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::NO_POR_DETECTED,
            true => POR_A::POR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POR_DETECTED`"]
    #[inline(always)]
    pub fn is_no_por_detected(&self) -> bool {
        *self == POR_A::NO_POR_DETECTED
    }
    #[doc = "Checks if the value of the field is `POR_DETECTED`"]
    #[inline(always)]
    pub fn is_por_detected(&self) -> bool {
        *self == POR_A::POR_DETECTED
    }
}
#[doc = "Field `EXTRST` reader - Status of the external RESET pin"]
pub type EXTRST_R = crate::BitReader<EXTRST_A>;
#[doc = "Status of the external RESET pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRST_A {
    #[doc = "0: No RESET event detected"]
    NO_RESET_EVENT_DETEC = 0,
    #[doc = "1: RESET detected"]
    RESET_DETECTED = 1,
}
impl From<EXTRST_A> for bool {
    #[inline(always)]
    fn from(variant: EXTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTRST_A {
        match self.bits {
            false => EXTRST_A::NO_RESET_EVENT_DETEC,
            true => EXTRST_A::RESET_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET_EVENT_DETEC`"]
    #[inline(always)]
    pub fn is_no_reset_event_detec(&self) -> bool {
        *self == EXTRST_A::NO_RESET_EVENT_DETEC
    }
    #[doc = "Checks if the value of the field is `RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_reset_detected(&self) -> bool {
        *self == EXTRST_A::RESET_DETECTED
    }
}
#[doc = "Field `WDT` reader - Status of the Watchdog reset"]
pub type WDT_R = crate::BitReader<WDT_A>;
#[doc = "Status of the Watchdog reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: No WDT reset detected"]
    NO_WDT_RESET_DETECTE = 0,
    #[doc = "1: WDT reset detected"]
    WDT_RESET_DETECTED = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::NO_WDT_RESET_DETECTE,
            true => WDT_A::WDT_RESET_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WDT_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_no_wdt_reset_detecte(&self) -> bool {
        *self == WDT_A::NO_WDT_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `WDT_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_wdt_reset_detected(&self) -> bool {
        *self == WDT_A::WDT_RESET_DETECTED
    }
}
#[doc = "Field `BOD` reader - Status of the Brown-out detect reset"]
pub type BOD_R = crate::BitReader<BOD_A>;
#[doc = "Status of the Brown-out detect reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_A {
    #[doc = "0: No BOD reset detected"]
    NO_BOD_RESET_DETECTE = 0,
    #[doc = "1: BOD reset detected"]
    BOD_RESET_DETECTED = 1,
}
impl From<BOD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_A) -> Self {
        variant as u8 != 0
    }
}
impl BOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_A {
        match self.bits {
            false => BOD_A::NO_BOD_RESET_DETECTE,
            true => BOD_A::BOD_RESET_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BOD_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_no_bod_reset_detecte(&self) -> bool {
        *self == BOD_A::NO_BOD_RESET_DETECTE
    }
    #[doc = "Checks if the value of the field is `BOD_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_bod_reset_detected(&self) -> bool {
        *self == BOD_A::BOD_RESET_DETECTED
    }
}
#[doc = "Field `SYSRST` reader - Status of the software system reset. The ARM software reset has the same effect as the hardware reset using the RESET pin."]
pub type SYSRST_R = crate::BitReader<SYSRST_A>;
#[doc = "Status of the software system reset. The ARM software reset has the same effect as the hardware reset using the RESET pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRST_A {
    #[doc = "0: No System reset detected"]
    NO_SYSTEM_RESET_DETE = 0,
    #[doc = "1: System reset detected"]
    SYSTEM_RESET_DETECTE = 1,
}
impl From<SYSRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRST_A {
        match self.bits {
            false => SYSRST_A::NO_SYSTEM_RESET_DETE,
            true => SYSRST_A::SYSTEM_RESET_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYSTEM_RESET_DETE`"]
    #[inline(always)]
    pub fn is_no_system_reset_dete(&self) -> bool {
        *self == SYSRST_A::NO_SYSTEM_RESET_DETE
    }
    #[doc = "Checks if the value of the field is `SYSTEM_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_system_reset_detecte(&self) -> bool {
        *self == SYSRST_A::SYSTEM_RESET_DETECTE
    }
}
impl R {
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the external RESET pin"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the software system reset. The ARM software reset has the same effect as the hardware reset using the RESET pin."]
    #[inline(always)]
    pub fn sysrst(&self) -> SYSRST_R {
        SYSRST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "System reset status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysresstat](index.html) module"]
pub struct SYSRESSTAT_SPEC;
impl crate::RegisterSpec for SYSRESSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysresstat::R](R) reader structure"]
impl crate::Readable for SYSRESSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSRESSTAT to value 0"]
impl crate::Resettable for SYSRESSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
