#[doc = "Register `INDATA` reader"]
pub struct R(crate::R<INDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDATA` writer"]
pub struct W(crate::W<INDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDATA_SPEC>;
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
impl From<crate::W<INDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - In this field the next word is written in little-endian format."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - In this field the next word is written in little-endian format."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - In this field the next word is written in little-endian format."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In this field the next word is written in little-endian format."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indata](index.html) module"]
pub struct INDATA_SPEC;
impl crate::RegisterSpec for INDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indata::R](R) reader structure"]
impl crate::Readable for INDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indata::W](W) writer structure"]
impl crate::Writable for INDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDATA to value 0"]
impl crate::Resettable for INDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
