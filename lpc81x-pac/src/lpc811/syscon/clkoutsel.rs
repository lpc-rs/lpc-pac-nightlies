#[doc = "Register `CLKOUTSEL` reader"]
pub struct R(crate::R<CLKOUTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTSEL` writer"]
pub struct W(crate::W<CLKOUTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTSEL_SPEC>;
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
impl From<crate::W<CLKOUTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - CLKOUT clock source."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "CLKOUT clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC = 0,
    #[doc = "1: Crystal oscillator (SYSOSC)"]
    SYSOSC = 1,
    #[doc = "2: Watchdog oscillator"]
    WATCHDOG = 2,
    #[doc = "3: Main clock"]
    MAIN_CLK = 3,
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
            0 => SEL_A::IRC,
            1 => SEL_A::SYSOSC,
            2 => SEL_A::WATCHDOG,
            3 => SEL_A::MAIN_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == SEL_A::IRC
    }
    #[doc = "Checks if the value of the field is `SYSOSC`"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == SEL_A::SYSOSC
    }
    #[doc = "Checks if the value of the field is `WATCHDOG`"]
    #[inline(always)]
    pub fn is_watchdog(&self) -> bool {
        *self == SEL_A::WATCHDOG
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == SEL_A::MAIN_CLK
    }
}
#[doc = "Field `SEL` writer - CLKOUT clock source."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLKOUTSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(SEL_A::IRC)
    }
    #[doc = "Crystal oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut W {
        self.variant(SEL_A::SYSOSC)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK)
    }
}
impl R {
    #[doc = "Bits 0:1 - CLKOUT clock source."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT clock source."]
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
#[doc = "CLKOUT clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutsel](index.html) module"]
pub struct CLKOUTSEL_SPEC;
impl crate::RegisterSpec for CLKOUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutsel::R](R) reader structure"]
impl crate::Readable for CLKOUTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutsel::W](W) writer structure"]
impl crate::Writable for CLKOUTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKOUTSEL to value 0"]
impl crate::Resettable for CLKOUTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
