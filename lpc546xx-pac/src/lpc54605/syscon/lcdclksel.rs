#[doc = "Register `LCDCLKSEL` reader"]
pub struct R(crate::R<LCDCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCLKSEL` writer"]
pub struct W(crate::W<LCDCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCLKSEL_SPEC>;
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
impl From<crate::W<LCDCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - LCD clock source select."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "LCD clock source select.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock (main_clk)"]
    MAIN_CLOCK = 0,
    #[doc = "1: LCDCLKIN (LCDCLK_EXT)"]
    LCDCLKIN = 1,
    #[doc = "2: FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF = 2,
    #[doc = "3: None, this may be selected in order to reduce power when no output is needed."]
    NONE = 3,
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
            0 => SEL_A::MAIN_CLOCK,
            1 => SEL_A::LCDCLKIN,
            2 => SEL_A::FRO_HF,
            3 => SEL_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `LCDCLKIN`"]
    #[inline(always)]
    pub fn is_lcdclkin(&self) -> bool {
        *self == SEL_A::LCDCLKIN
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        *self == SEL_A::FRO_HF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - LCD clock source select."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LCDCLKSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main clock (main_clk)"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = "LCDCLKIN (LCDCLK_EXT)"]
    #[inline(always)]
    pub fn lcdclkin(self) -> &'a mut W {
        self.variant(SEL_A::LCDCLKIN)
    }
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:1 - LCD clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD clock source select."]
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
#[doc = "LCD clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdclksel](index.html) module"]
pub struct LCDCLKSEL_SPEC;
impl crate::RegisterSpec for LCDCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdclksel::R](R) reader structure"]
impl crate::Readable for LCDCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdclksel::W](W) writer structure"]
impl crate::Writable for LCDCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCLKSEL to value 0x03"]
impl crate::Resettable for LCDCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
