#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Aperture` reader - Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
pub type APERTURE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Minor_Rev` reader - Minor revision of module implementation, starting at 0."]
pub type MINOR_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Major_Rev` reader - Major revision of module implementation, starting at 0."]
pub type MAJOR_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ID` reader - Unique module identifier for this IP block."]
pub type ID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub fn aperture(&self) -> APERTURE_R {
        APERTURE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision of module implementation, starting at 0."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision of module implementation, starting at 0."]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Unique module identifier for this IP block."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "I2S Module identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID to value 0xe090_0000"]
impl crate::Resettable for ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe090_0000
    }
}
