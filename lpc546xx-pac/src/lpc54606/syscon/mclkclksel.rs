#[doc = "Register `MCLKCLKSEL` reader"]
pub struct R(crate::R<MCLKCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKCLKSEL` writer"]
pub struct W(crate::W<MCLKCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKCLKSEL_SPEC>;
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
impl From<crate::W<MCLKCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - MCLK source select. This may be used by Flexcomms that support I2S, and/or by the digital microphone subsystem."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "MCLK source select. This may be used by Flexcomms that support I2S, and/or by the digital microphone subsystem.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO HF DIV (fro_hf_div)"]
    FRO_HF_DIV = 0,
    #[doc = "1: Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT = 1,
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
            0 => Some(SEL_A::FRO_HF_DIV),
            1 => Some(SEL_A::AUDIO_PLL_OUTPUT),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRO_HF_DIV`"]
    #[inline(always)]
    pub fn is_fro_hf_div(&self) -> bool {
        *self == SEL_A::FRO_HF_DIV
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
#[doc = "Field `SEL` writer - MCLK source select. This may be used by Flexcomms that support I2S, and/or by the digital microphone subsystem."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCLKCLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "FRO HF DIV (fro_hf_div)"]
    #[inline(always)]
    pub fn fro_hf_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF_DIV)
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
    #[doc = "Bits 0:2 - MCLK source select. This may be used by Flexcomms that support I2S, and/or by the digital microphone subsystem."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK source select. This may be used by Flexcomms that support I2S, and/or by the digital microphone subsystem."]
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
#[doc = "MCLK clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkclksel](index.html) module"]
pub struct MCLKCLKSEL_SPEC;
impl crate::RegisterSpec for MCLKCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkclksel::R](R) reader structure"]
impl crate::Readable for MCLKCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkclksel::W](W) writer structure"]
impl crate::Writable for MCLKCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCLKCLKSEL to value 0x07"]
impl crate::Resettable for MCLKCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
