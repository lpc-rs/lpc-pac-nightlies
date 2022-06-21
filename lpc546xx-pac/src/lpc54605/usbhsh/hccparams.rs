#[doc = "Register `HCCPARAMS` reader"]
pub struct R(crate::R<HCCPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPMC` reader - Link Power Management Capability."]
pub type LPMC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 17 - Link Power Management Capability."]
    #[inline(always)]
    pub fn lpmc(&self) -> LPMC_R {
        LPMC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Host Controller Capability Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccparams](index.html) module"]
pub struct HCCPARAMS_SPEC;
impl crate::RegisterSpec for HCCPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccparams::R](R) reader structure"]
impl crate::Readable for HCCPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCCPARAMS to value 0x0002_0006"]
impl crate::Resettable for HCCPARAMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0006
    }
}
