#[doc = "Register `WDTCLKDIV` reader"]
pub struct R(crate::R<WDTCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCLKDIV` writer"]
pub struct W(crate::W<WDTCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCLKDIV_SPEC>;
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
impl From<crate::W<WDTCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - WDT clock divider values. 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - WDT clock divider values. 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCLKDIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - WDT clock divider values. 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - WDT clock divider values. 0: Disable WDCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclkdiv](index.html) module"]
pub struct WDTCLKDIV_SPEC;
impl crate::RegisterSpec for WDTCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtclkdiv::R](R) reader structure"]
impl crate::Readable for WDTCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtclkdiv::W](W) writer structure"]
impl crate::Writable for WDTCLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCLKDIV to value 0"]
impl crate::Resettable for WDTCLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
