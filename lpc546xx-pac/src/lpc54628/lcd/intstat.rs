#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FUFMIS` reader - FIFO underflow masked interrupt status."]
pub type FUFMIS_R = crate::BitReader<bool>;
#[doc = "Field `LNBUMIS` reader - LCD next address base update masked interrupt status."]
pub type LNBUMIS_R = crate::BitReader<bool>;
#[doc = "Field `VCOMPMIS` reader - Vertical compare masked interrupt status."]
pub type VCOMPMIS_R = crate::BitReader<bool>;
#[doc = "Field `BERMIS` reader - AHB master bus error masked interrupt status."]
pub type BERMIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - FIFO underflow masked interrupt status."]
    #[inline(always)]
    pub fn fufmis(&self) -> FUFMIS_R {
        FUFMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update masked interrupt status."]
    #[inline(always)]
    pub fn lnbumis(&self) -> LNBUMIS_R {
        LNBUMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical compare masked interrupt status."]
    #[inline(always)]
    pub fn vcompmis(&self) -> VCOMPMIS_R {
        VCOMPMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error masked interrupt status."]
    #[inline(always)]
    pub fn bermis(&self) -> BERMIS_R {
        BERMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
