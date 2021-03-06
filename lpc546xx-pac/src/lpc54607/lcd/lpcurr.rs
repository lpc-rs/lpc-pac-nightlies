#[doc = "Register `LPCURR` reader"]
pub struct R(crate::R<LPCURR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCURR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCURR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCURR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCDLPCURR` reader - LCD Lower Panel Current Address."]
pub type LCDLPCURR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - LCD Lower Panel Current Address."]
    #[inline(always)]
    pub fn lcdlpcurr(&self) -> LCDLPCURR_R {
        LCDLPCURR_R::new(self.bits)
    }
}
#[doc = "Lower Panel Current Address Value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcurr](index.html) module"]
pub struct LPCURR_SPEC;
impl crate::RegisterSpec for LPCURR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpcurr::R](R) reader structure"]
impl crate::Readable for LPCURR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPCURR to value 0"]
impl crate::Resettable for LPCURR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
