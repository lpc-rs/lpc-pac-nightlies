#[doc = "Register `SCTCLKSEL` reader"]
pub struct R(crate::R<SCTCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTCLKSEL` writer"]
pub struct W(crate::W<SCTCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTCLKSEL_SPEC>;
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
impl From<crate::W<SCTCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - SCT clock source select."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "SCT clock source select.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock (main_clk)"]
    MAIN_CLOCK = 0,
    #[doc = "1: System PLL output (pll_clk)"]
    SYSTEM_PLL_OUTPUT = 1,
    #[doc = "2: FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF = 2,
    #[doc = "3: Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT = 3,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    NONE = 7,
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
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::MAIN_CLOCK),
            1 => Some(SEL_A::SYSTEM_PLL_OUTPUT),
            2 => Some(SEL_A::FRO_HF),
            3 => Some(SEL_A::AUDIO_PLL_OUTPUT),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_system_pll_output(&self) -> bool {
        *self == SEL_A::SYSTEM_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        *self == SEL_A::FRO_HF
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_audio_pll_output(&self) -> bool {
        *self == SEL_A::AUDIO_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - SCT clock source select."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main clock (main_clk)"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = "System PLL output (pll_clk)"]
    #[inline(always)]
    pub fn system_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_OUTPUT)
    }
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
    }
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    #[inline(always)]
    pub fn audio_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL_OUTPUT)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - SCT clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCT clock source select."]
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
#[doc = "SCTimer/PWM clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctclksel](index.html) module"]
pub struct SCTCLKSEL_SPEC;
impl crate::RegisterSpec for SCTCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctclksel::R](R) reader structure"]
impl crate::Readable for SCTCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctclksel::W](W) writer structure"]
impl crate::Writable for SCTCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCTCLKSEL to value 0x07"]
impl crate::Resettable for SCTCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
