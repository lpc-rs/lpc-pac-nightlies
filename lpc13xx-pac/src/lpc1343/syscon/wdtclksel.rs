#[doc = "Register `WDTCLKSEL` reader"]
pub struct R(crate::R<WDTCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCLKSEL` writer"]
pub struct W(crate::W<WDTCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCLKSEL_SPEC>;
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
impl From<crate::W<WDTCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - WDT clock source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "WDT clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC_OSCILLATOR = 0,
    #[doc = "1: Main clock"]
    MAIN_CLOCK = 1,
    #[doc = "2: Watchdog oscillator"]
    WATCHDOG_OSCILLATOR = 2,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::IRC_OSCILLATOR,
            1 => SEL_A::MAIN_CLOCK,
            2 => SEL_A::WATCHDOG_OSCILLATOR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SEL_A::WATCHDOG_OSCILLATOR
    }
}
#[doc = "Field `SEL` writer - WDT clock source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCLKSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSCILLATOR)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG_OSCILLATOR)
    }
}
impl R {
    #[doc = "Bits 0:1 - WDT clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - WDT clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclksel](index.html) module"]
pub struct WDTCLKSEL_SPEC;
impl crate::RegisterSpec for WDTCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtclksel::R](R) reader structure"]
impl crate::Readable for WDTCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtclksel::W](W) writer structure"]
impl crate::Writable for WDTCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCLKSEL to value 0"]
impl crate::Resettable for WDTCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
