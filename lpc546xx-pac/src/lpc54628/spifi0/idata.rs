#[doc = "Register `IDATA` reader"]
pub struct R(crate::R<IDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDATA` writer"]
pub struct W(crate::W<IDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDATA_SPEC>;
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
impl From<crate::W<IDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDATA` reader - Value of intermediate bytes."]
pub type IDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDATA` writer - Value of intermediate bytes."]
pub type IDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Value of intermediate bytes."]
    #[inline(always)]
    pub fn idata(&self) -> IDATA_R {
        IDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of intermediate bytes."]
    #[inline(always)]
    pub fn idata(&mut self) -> IDATA_W<0> {
        IDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIFI intermediate data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata](index.html) module"]
pub struct IDATA_SPEC;
impl crate::RegisterSpec for IDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idata::R](R) reader structure"]
impl crate::Readable for IDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idata::W](W) writer structure"]
impl crate::Writable for IDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDATA to value 0"]
impl crate::Resettable for IDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
