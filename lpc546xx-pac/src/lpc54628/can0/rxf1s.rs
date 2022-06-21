#[doc = "Register `RXF1S` reader"]
pub struct R(crate::R<RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1FL` reader - Rx FIFO 1 fill level."]
pub type F1FL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1GI` reader - Rx FIFO 1 get index."]
pub type F1GI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1PI` reader - Rx FIFO 1 put index."]
pub type F1PI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1F` reader - Rx FIFO 1 full."]
pub type F1F_R = crate::BitReader<bool>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost."]
pub type RF1L_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 fill level."]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 get index."]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 put index."]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Rx FIFO 1 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1s](index.html) module"]
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf1s::R](R) reader structure"]
impl crate::Readable for RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for RXF1S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
