#[doc = "Register `IIR` reader"]
pub struct R(crate::R<IIR_FCR_IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_FCR_IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_FCR_IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_FCR_IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSTATUS` reader - Interrupt status."]
pub type INTSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `INTID` reader - Interrupt identification."]
pub type INTID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOENABLE` reader - Copies of SCInFCR\\[0\\]."]
pub type FIFOENABLE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Interrupt status."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Copies of SCInFCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Interrupt ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir_fcr_iir](index.html) module"]
pub struct IIR_FCR_IIR_SPEC;
impl crate::RegisterSpec for IIR_FCR_IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir_fcr_iir::R](R) reader structure"]
impl crate::Readable for IIR_FCR_IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_FCR_IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
