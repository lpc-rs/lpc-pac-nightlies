#[doc = "Register `USER0` reader"]
pub struct R(crate::R<USER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USER0` reader - User application specific option."]
pub type USER0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - User application specific option."]
    #[inline(always)]
    pub fn user0(&self) -> USER0_R {
        USER0_R::new(self.bits)
    }
}
#[doc = "User application specific options.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user0](index.html) module"]
pub struct USER0_SPEC;
impl crate::RegisterSpec for USER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user0::R](R) reader structure"]
impl crate::Readable for USER0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USER0 to value 0"]
impl crate::Resettable for USER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
