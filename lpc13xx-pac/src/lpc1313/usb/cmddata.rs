#[doc = "Register `CMDDATA` reader"]
pub struct R(crate::R<CMDDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMD_RDATA` reader - Command Read Data."]
pub type CMD_RDATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Command Read Data."]
    #[inline(always)]
    pub fn cmd_rdata(&self) -> CMD_RDATA_R {
        CMD_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "USB Command Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmddata](index.html) module"]
pub struct CMDDATA_SPEC;
impl crate::RegisterSpec for CMDDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmddata::R](R) reader structure"]
impl crate::Readable for CMDDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMDDATA to value 0"]
impl crate::Resettable for CMDDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
