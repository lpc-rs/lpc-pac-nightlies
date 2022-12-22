#[doc = "Register `MAINCLKSEL` reader"]
pub struct R(crate::R<MAINCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSEL` writer"]
pub struct W(crate::W<MAINCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSEL_SPEC>;
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
impl From<crate::W<MAINCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Clock source for main clock"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Clock source for main clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC oscillator"]
    IRC_OSCILLATOR = 0,
    #[doc = "1: Input clock to system PLL"]
    INPUT_CLOCK_TO_SYSTE = 1,
    #[doc = "2: WDT oscillator"]
    WDT_OSCILLATOR = 2,
    #[doc = "3: System PLL clock out"]
    SYSTEM_PLL_CLOCK_OUT = 3,
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
            1 => SEL_A::INPUT_CLOCK_TO_SYSTE,
            2 => SEL_A::WDT_OSCILLATOR,
            3 => SEL_A::SYSTEM_PLL_CLOCK_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SEL_A::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `INPUT_CLOCK_TO_SYSTE`"]
    #[inline(always)]
    pub fn is_input_clock_to_syste(&self) -> bool {
        *self == SEL_A::INPUT_CLOCK_TO_SYSTE
    }
    #[doc = "Checks if the value of the field is `WDT_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_wdt_oscillator(&self) -> bool {
        *self == SEL_A::WDT_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_CLOCK_OUT`"]
    #[inline(always)]
    pub fn is_system_pll_clock_out(&self) -> bool {
        *self == SEL_A::SYSTEM_PLL_CLOCK_OUT
    }
}
#[doc = "Field `SEL` writer - Clock source for main clock"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MAINCLKSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "IRC oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::IRC_OSCILLATOR)
    }
    #[doc = "Input clock to system PLL"]
    #[inline(always)]
    pub fn input_clock_to_syste(self) -> &'a mut W {
        self.variant(SEL_A::INPUT_CLOCK_TO_SYSTE)
    }
    #[doc = "WDT oscillator"]
    #[inline(always)]
    pub fn wdt_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WDT_OSCILLATOR)
    }
    #[doc = "System PLL clock out"]
    #[inline(always)]
    pub fn system_pll_clock_out(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_CLOCK_OUT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock"]
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
#[doc = "Main clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksel](index.html) module"]
pub struct MAINCLKSEL_SPEC;
impl crate::RegisterSpec for MAINCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclksel::R](R) reader structure"]
impl crate::Readable for MAINCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclksel::W](W) writer structure"]
impl crate::Writable for MAINCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAINCLKSEL to value 0"]
impl crate::Resettable for MAINCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
