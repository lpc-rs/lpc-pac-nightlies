#[doc = "Register `SYSRSTSTAT` reader"]
pub struct R(crate::R<SYSRSTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSRSTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSRSTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSRSTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSRSTSTAT` writer"]
pub struct W(crate::W<SYSRSTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSRSTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYSRSTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSRSTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` reader - POR reset status."]
pub type POR_R = crate::BitReader<POR_A>;
#[doc = "POR reset status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: No POR detected."]
    POR_0 = 0,
    #[doc = "1: POR detected. Writing a one clears this reset."]
    POR_1 = 1,
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
            false => POR_A::POR_0,
            true => POR_A::POR_1,
        }
    }
    #[doc = "Checks if the value of the field is `POR_0`"]
    #[inline(always)]
    pub fn is_por_0(&self) -> bool {
        *self == POR_A::POR_0
    }
    #[doc = "Checks if the value of the field is `POR_1`"]
    #[inline(always)]
    pub fn is_por_1(&self) -> bool {
        *self == POR_A::POR_1
    }
}
#[doc = "Field `POR` writer - POR reset status."]
pub type POR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSRSTSTAT_SPEC, POR_A, O>;
impl<'a, const O: u8> POR_W<'a, O> {
    #[doc = "No POR detected."]
    #[inline(always)]
    pub fn por_0(self) -> &'a mut W {
        self.variant(POR_A::POR_0)
    }
    #[doc = "POR detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn por_1(self) -> &'a mut W {
        self.variant(POR_A::POR_1)
    }
}
#[doc = "Field `EXTRST` reader - Status of the external RESET pin. External reset status."]
pub type EXTRST_R = crate::BitReader<EXTRST_A>;
#[doc = "Status of the external RESET pin. External reset status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRST_A {
    #[doc = "0: No reset event detected."]
    EXTRST_0 = 0,
    #[doc = "1: Reset detected. Writing a one clears this reset."]
    EXTRST_1 = 1,
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
            false => EXTRST_A::EXTRST_0,
            true => EXTRST_A::EXTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXTRST_0`"]
    #[inline(always)]
    pub fn is_extrst_0(&self) -> bool {
        *self == EXTRST_A::EXTRST_0
    }
    #[doc = "Checks if the value of the field is `EXTRST_1`"]
    #[inline(always)]
    pub fn is_extrst_1(&self) -> bool {
        *self == EXTRST_A::EXTRST_1
    }
}
#[doc = "Field `EXTRST` writer - Status of the external RESET pin. External reset status."]
pub type EXTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSRSTSTAT_SPEC, EXTRST_A, O>;
impl<'a, const O: u8> EXTRST_W<'a, O> {
    #[doc = "No reset event detected."]
    #[inline(always)]
    pub fn extrst_0(self) -> &'a mut W {
        self.variant(EXTRST_A::EXTRST_0)
    }
    #[doc = "Reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn extrst_1(self) -> &'a mut W {
        self.variant(EXTRST_A::EXTRST_1)
    }
}
#[doc = "Field `WDT` reader - Status of the Watchdog reset."]
pub type WDT_R = crate::BitReader<WDT_A>;
#[doc = "Status of the Watchdog reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: No WDT reset detected."]
    WDT_0 = 0,
    #[doc = "1: WDT reset detected. Writing a one clears this reset."]
    WDT_1 = 1,
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
            false => WDT_A::WDT_0,
            true => WDT_A::WDT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDT_0`"]
    #[inline(always)]
    pub fn is_wdt_0(&self) -> bool {
        *self == WDT_A::WDT_0
    }
    #[doc = "Checks if the value of the field is `WDT_1`"]
    #[inline(always)]
    pub fn is_wdt_1(&self) -> bool {
        *self == WDT_A::WDT_1
    }
}
#[doc = "Field `WDT` writer - Status of the Watchdog reset."]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSRSTSTAT_SPEC, WDT_A, O>;
impl<'a, const O: u8> WDT_W<'a, O> {
    #[doc = "No WDT reset detected."]
    #[inline(always)]
    pub fn wdt_0(self) -> &'a mut W {
        self.variant(WDT_A::WDT_0)
    }
    #[doc = "WDT reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn wdt_1(self) -> &'a mut W {
        self.variant(WDT_A::WDT_1)
    }
}
#[doc = "Field `BOD` reader - Status of the Brown-out detect reset."]
pub type BOD_R = crate::BitReader<BOD_A>;
#[doc = "Status of the Brown-out detect reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_A {
    #[doc = "0: No BOD reset detected."]
    BOD_0 = 0,
    #[doc = "1: BOD reset detected. Writing a one clears this reset."]
    BOD_1 = 1,
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
            false => BOD_A::BOD_0,
            true => BOD_A::BOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOD_0`"]
    #[inline(always)]
    pub fn is_bod_0(&self) -> bool {
        *self == BOD_A::BOD_0
    }
    #[doc = "Checks if the value of the field is `BOD_1`"]
    #[inline(always)]
    pub fn is_bod_1(&self) -> bool {
        *self == BOD_A::BOD_1
    }
}
#[doc = "Field `BOD` writer - Status of the Brown-out detect reset."]
pub type BOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSRSTSTAT_SPEC, BOD_A, O>;
impl<'a, const O: u8> BOD_W<'a, O> {
    #[doc = "No BOD reset detected."]
    #[inline(always)]
    pub fn bod_0(self) -> &'a mut W {
        self.variant(BOD_A::BOD_0)
    }
    #[doc = "BOD reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn bod_1(self) -> &'a mut W {
        self.variant(BOD_A::BOD_1)
    }
}
#[doc = "Field `SYSRST` reader - Status of the software system reset."]
pub type SYSRST_R = crate::BitReader<SYSRST_A>;
#[doc = "Status of the software system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRST_A {
    #[doc = "0: No System reset detected."]
    SYSRST_0 = 0,
    #[doc = "1: System reset detected. Writing a one clears this reset."]
    SYSRST_1 = 1,
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
            false => SYSRST_A::SYSRST_0,
            true => SYSRST_A::SYSRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST_0`"]
    #[inline(always)]
    pub fn is_sysrst_0(&self) -> bool {
        *self == SYSRST_A::SYSRST_0
    }
    #[doc = "Checks if the value of the field is `SYSRST_1`"]
    #[inline(always)]
    pub fn is_sysrst_1(&self) -> bool {
        *self == SYSRST_A::SYSRST_1
    }
}
#[doc = "Field `SYSRST` writer - Status of the software system reset."]
pub type SYSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSRSTSTAT_SPEC, SYSRST_A, O>;
impl<'a, const O: u8> SYSRST_W<'a, O> {
    #[doc = "No System reset detected."]
    #[inline(always)]
    pub fn sysrst_0(self) -> &'a mut W {
        self.variant(SYSRST_A::SYSRST_0)
    }
    #[doc = "System reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn sysrst_1(self) -> &'a mut W {
        self.variant(SYSRST_A::SYSRST_1)
    }
}
impl R {
    #[doc = "Bit 0 - POR reset status."]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the external RESET pin. External reset status."]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset."]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the software system reset."]
    #[inline(always)]
    pub fn sysrst(&self) -> SYSRST_R {
        SYSRST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POR reset status."]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W<0> {
        POR_W::new(self)
    }
    #[doc = "Bit 1 - Status of the external RESET pin. External reset status."]
    #[inline(always)]
    pub fn extrst(&mut self) -> EXTRST_W<1> {
        EXTRST_W::new(self)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<2> {
        WDT_W::new(self)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset."]
    #[inline(always)]
    pub fn bod(&mut self) -> BOD_W<3> {
        BOD_W::new(self)
    }
    #[doc = "Bit 4 - Status of the software system reset."]
    #[inline(always)]
    pub fn sysrst(&mut self) -> SYSRST_W<4> {
        SYSRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System reset status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrststat](index.html) module"]
pub struct SYSRSTSTAT_SPEC;
impl crate::RegisterSpec for SYSRSTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysrststat::R](R) reader structure"]
impl crate::Readable for SYSRSTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysrststat::W](W) writer structure"]
impl crate::Writable for SYSRSTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSRSTSTAT to value 0"]
impl crate::Resettable for SYSRSTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}