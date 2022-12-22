#[doc = "Register `HWVADST10` reader"]
pub struct R(crate::R<HWVADST10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADST10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADST10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADST10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVADST10` writer"]
pub struct W(crate::W<HWVADST10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADST10_SPEC>;
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
impl From<crate::W<HWVADST10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADST10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST10` reader - Stage 0"]
pub type ST10_R = crate::BitReader<ST10_A>;
#[doc = "Stage 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST10_A {
    #[doc = "0: Normal operation, waiting for HWVAD trigger event (stage 0)."]
    NORMAL = 0,
    #[doc = "1: Reset internal interrupt flag by writing a '1' pulse."]
    RESET = 1,
}
impl From<ST10_A> for bool {
    #[inline(always)]
    fn from(variant: ST10_A) -> Self {
        variant as u8 != 0
    }
}
impl ST10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST10_A {
        match self.bits {
            false => ST10_A::NORMAL,
            true => ST10_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ST10_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ST10_A::RESET
    }
}
#[doc = "Field `ST10` writer - Stage 0"]
pub type ST10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWVADST10_SPEC, ST10_A, O>;
impl<'a, const O: u8> ST10_W<'a, O> {
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ST10_A::NORMAL)
    }
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ST10_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Stage 0"]
    #[inline(always)]
    pub fn st10(&self) -> ST10_R {
        ST10_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stage 0"]
    #[inline(always)]
    pub fn st10(&mut self) -> ST10_W<0> {
        ST10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HWVAD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadst10](index.html) module"]
pub struct HWVADST10_SPEC;
impl crate::RegisterSpec for HWVADST10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadst10::R](R) reader structure"]
impl crate::Readable for HWVADST10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwvadst10::W](W) writer structure"]
impl crate::Writable for HWVADST10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWVADST10 to value 0"]
impl crate::Resettable for HWVADST10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
