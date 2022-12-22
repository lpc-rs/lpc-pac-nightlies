#[doc = "Register `MOD` reader"]
pub struct R(crate::R<MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOD` writer"]
pub struct W(crate::W<MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOD_SPEC>;
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
impl From<crate::W<MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDEN` reader - Watchdog enable bit. This bit is Set Only. Remark: Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to watchdog oscillator before setting this bit to one."]
pub type WDEN_R = crate::BitReader<WDEN_A>;
#[doc = "Watchdog enable bit. This bit is Set Only. Remark: Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to watchdog oscillator before setting this bit to one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDEN_A {
    #[doc = "0: The watchdog timer is stopped."]
    STOP = 0,
    #[doc = "1: The watchdog timer is running."]
    RUN = 1,
}
impl From<WDEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDEN_A {
        match self.bits {
            false => WDEN_A::STOP,
            true => WDEN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == WDEN_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WDEN_A::RUN
    }
}
#[doc = "Field `WDEN` writer - Watchdog enable bit. This bit is Set Only. Remark: Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to watchdog oscillator before setting this bit to one."]
pub type WDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDEN_A, O>;
impl<'a, const O: u8> WDEN_W<'a, O> {
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(WDEN_A::STOP)
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(WDEN_A::RUN)
    }
}
#[doc = "Field `WDRESET` reader - Watchdog reset enable bit. This bit is Set Only."]
pub type WDRESET_R = crate::BitReader<WDRESET_A>;
#[doc = "Watchdog reset enable bit. This bit is Set Only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESET_A {
    #[doc = "0: A watchdog timeout will not cause a chip reset."]
    RESET = 0,
    #[doc = "1: A watchdog timeout will cause a chip reset."]
    INTERRUPT = 1,
}
impl From<WDRESET_A> for bool {
    #[inline(always)]
    fn from(variant: WDRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl WDRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDRESET_A {
        match self.bits {
            false => WDRESET_A::RESET,
            true => WDRESET_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDRESET_A::RESET
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDRESET_A::INTERRUPT
    }
}
#[doc = "Field `WDRESET` writer - Watchdog reset enable bit. This bit is Set Only."]
pub type WDRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDRESET_A, O>;
impl<'a, const O: u8> WDRESET_W<'a, O> {
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDRESET_A::RESET)
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDRESET_A::INTERRUPT)
    }
}
#[doc = "Field `WDTOF` reader - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
pub type WDTOF_R = crate::BitReader<bool>;
#[doc = "Field `WDTOF` writer - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
pub type WDTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, bool, O>;
#[doc = "Field `WDINT` reader - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
pub type WDINT_R = crate::BitReader<bool>;
#[doc = "Field `WDINT` writer - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
pub type WDINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, bool, O>;
#[doc = "Field `WDPROTECT` reader - Watchdog update mode. This bit is Set Only."]
pub type WDPROTECT_R = crate::BitReader<WDPROTECT_A>;
#[doc = "Watchdog update mode. This bit is Set Only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECT_A {
    #[doc = "0: The watchdog reload value (WDTC) can be changed at any time."]
    ANYTIME = 0,
    #[doc = "1: The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1."]
    MATCH = 1,
}
impl From<WDPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: WDPROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDPROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDPROTECT_A {
        match self.bits {
            false => WDPROTECT_A::ANYTIME,
            true => WDPROTECT_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `ANYTIME`"]
    #[inline(always)]
    pub fn is_anytime(&self) -> bool {
        *self == WDPROTECT_A::ANYTIME
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == WDPROTECT_A::MATCH
    }
}
#[doc = "Field `WDPROTECT` writer - Watchdog update mode. This bit is Set Only."]
pub type WDPROTECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MOD_SPEC, WDPROTECT_A, O>;
impl<'a, const O: u8> WDPROTECT_W<'a, O> {
    #[doc = "The watchdog reload value (WDTC) can be changed at any time."]
    #[inline(always)]
    pub fn anytime(self) -> &'a mut W {
        self.variant(WDPROTECT_A::ANYTIME)
    }
    #[doc = "The watchdog reload value (WDTC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW. Note: this mode is intended for use only when WDRESET =1."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(WDPROTECT_A::MATCH)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. Remark: Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to watchdog oscillator before setting this bit to one."]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only."]
    #[inline(always)]
    pub fn wdreset(&self) -> WDRESET_R {
        WDRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&self) -> WDTOF_R {
        WDTOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&self) -> WDINT_R {
        WDINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only."]
    #[inline(always)]
    pub fn wdprotect(&self) -> WDPROTECT_R {
        WDPROTECT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only. Remark: Setting this bit to one also locks the watchdog clock source. Once the watchdog timer is enabled, the watchdog timer clock source cannot be changed. If the watchdog timer is needed in Deep-sleep mode, the watchdog clock source must be changed to watchdog oscillator before setting this bit to one."]
    #[inline(always)]
    pub fn wden(&mut self) -> WDEN_W<0> {
        WDEN_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only."]
    #[inline(always)]
    pub fn wdreset(&mut self) -> WDRESET_W<1> {
        WDRESET_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT, cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&mut self) -> WDTOF_W<2> {
        WDTOF_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&mut self) -> WDINT_W<3> {
        WDINT_W::new(self)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit is Set Only."]
    #[inline(always)]
    pub fn wdprotect(&mut self) -> WDPROTECT_W<4> {
        WDPROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](index.html) module"]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_::R](R) reader structure"]
impl crate::Readable for MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mod_::W](W) writer structure"]
impl crate::Writable for MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
