#[doc = "Register `FBWST` reader"]
pub struct R(crate::R<FBWST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBWST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBWST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBWST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBWST` writer"]
pub struct W(crate::W<FBWST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBWST_SPEC>;
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
impl From<crate::W<FBWST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBWST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITSTATES` reader - Wait states for signature generation."]
pub type WAITSTATES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAITSTATES` writer - Wait states for signature generation."]
pub type WAITSTATES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBWST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Wait states for signature generation."]
    #[inline(always)]
    pub fn waitstates(&self) -> WAITSTATES_R {
        WAITSTATES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states for signature generation."]
    #[inline(always)]
    pub fn waitstates(&mut self) -> WAITSTATES_W<0> {
        WAITSTATES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbwst](index.html) module"]
pub struct FBWST_SPEC;
impl crate::RegisterSpec for FBWST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbwst::R](R) reader structure"]
impl crate::Readable for FBWST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbwst::W](W) writer structure"]
impl crate::Writable for FBWST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBWST to value 0xc005"]
impl crate::Resettable for FBWST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc005
    }
}
