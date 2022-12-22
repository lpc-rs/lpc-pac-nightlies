#[doc = "Register `NDAT1` reader"]
pub struct R(crate::R<NDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NDAT1` writer"]
pub struct W(crate::W<NDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDAT1_SPEC>;
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
impl From<crate::W<NDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ND` reader - New Data."]
pub type ND_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ND` writer - New Data."]
pub type ND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NDAT1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - New Data."]
    #[inline(always)]
    pub fn nd(&self) -> ND_R {
        ND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - New Data."]
    #[inline(always)]
    pub fn nd(&mut self) -> ND_W<0> {
        ND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "New Data 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat1](index.html) module"]
pub struct NDAT1_SPEC;
impl crate::RegisterSpec for NDAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndat1::R](R) reader structure"]
impl crate::Readable for NDAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ndat1::W](W) writer structure"]
impl crate::Writable for NDAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NDAT1 to value 0"]
impl crate::Resettable for NDAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
