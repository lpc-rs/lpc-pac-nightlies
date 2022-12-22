#[doc = "Register `_ITMISCIN` reader"]
pub struct R(crate::R<_ITMISCIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_ITMISCIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_ITMISCIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_ITMISCIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTIN` reader - A read of these bits returns the value of the EXTIN\\[1:0\\]
input pins."]
pub type EXTIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COREHALT` reader - A read of this bit returns the value of the COREHALT input pin."]
pub type COREHALT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - A read of these bits returns the value of the EXTIN\\[1:0\\]
input pins."]
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - A read of this bit returns the value of the COREHALT input pin."]
    #[inline(always)]
    pub fn corehalt(&self) -> COREHALT_R {
        COREHALT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Integration Test Miscelaneous Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_itmiscin](index.html) module"]
pub struct _ITMISCIN_SPEC;
impl crate::RegisterSpec for _ITMISCIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_itmiscin::R](R) reader structure"]
impl crate::Readable for _ITMISCIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _ITMISCIN to value 0"]
impl crate::Resettable for _ITMISCIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
