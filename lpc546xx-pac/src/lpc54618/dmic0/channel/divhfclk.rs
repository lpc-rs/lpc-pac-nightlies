#[doc = "Register `DIVHFCLK` reader"]
pub struct R(crate::R<DIVHFCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVHFCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVHFCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVHFCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVHFCLK` writer"]
pub struct W(crate::W<DIVHFCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVHFCLK_SPEC>;
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
impl From<crate::W<DIVHFCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVHFCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDMDIV` reader - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved."]
pub type PDMDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDMDIV` writer - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved."]
pub type PDMDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVHFCLK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved."]
    #[inline(always)]
    pub fn pdmdiv(&self) -> PDMDIV_R {
        PDMDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved."]
    #[inline(always)]
    pub fn pdmdiv(&mut self) -> PDMDIV_W<0> {
        PDMDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMIC Clock Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divhfclk](index.html) module"]
pub struct DIVHFCLK_SPEC;
impl crate::RegisterSpec for DIVHFCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divhfclk::R](R) reader structure"]
impl crate::Readable for DIVHFCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divhfclk::W](W) writer structure"]
impl crate::Writable for DIVHFCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVHFCLK to value 0"]
impl crate::Resettable for DIVHFCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
