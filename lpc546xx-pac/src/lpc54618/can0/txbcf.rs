#[doc = "Register `TXBCF` reader"]
pub struct R(crate::R<TXBCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBCF` writer"]
pub struct W(crate::W<TXBCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCF_SPEC>;
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
impl From<crate::W<TXBCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - Cancellation finished."]
pub type TO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TO` writer - Cancellation finished."]
pub type TO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBCF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Cancellation finished."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation finished."]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W<0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Cancellation Finished\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcf](index.html) module"]
pub struct TXBCF_SPEC;
impl crate::RegisterSpec for TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcf::R](R) reader structure"]
impl crate::Readable for TXBCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbcf::W](W) writer structure"]
impl crate::Writable for TXBCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TXBCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}