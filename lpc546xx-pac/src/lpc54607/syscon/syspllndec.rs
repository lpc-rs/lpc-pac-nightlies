#[doc = "Register `SYSPLLNDEC` reader"]
pub struct R(crate::R<SYSPLLNDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLNDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLNDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLNDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLNDEC` writer"]
pub struct W(crate::W<SYSPLLNDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLNDEC_SPEC>;
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
impl From<crate::W<SYSPLLNDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLNDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDEC` reader - Decoded N-divider coefficient value."]
pub type NDEC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDEC` writer - Decoded N-divider coefficient value."]
pub type NDEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLNDEC_SPEC, u16, u16, 10, O>;
#[doc = "Field `NREQ` reader - NDEC reload request."]
pub type NREQ_R = crate::BitReader<bool>;
#[doc = "Field `NREQ` writer - NDEC reload request."]
pub type NREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLNDEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - Decoded N-divider coefficient value."]
    #[inline(always)]
    pub fn ndec(&self) -> NDEC_R {
        NDEC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - NDEC reload request."]
    #[inline(always)]
    pub fn nreq(&self) -> NREQ_R {
        NREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Decoded N-divider coefficient value."]
    #[inline(always)]
    pub fn ndec(&mut self) -> NDEC_W<0> {
        NDEC_W::new(self)
    }
    #[doc = "Bit 10 - NDEC reload request."]
    #[inline(always)]
    pub fn nreq(&mut self) -> NREQ_W<10> {
        NREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllndec](index.html) module"]
pub struct SYSPLLNDEC_SPEC;
impl crate::RegisterSpec for SYSPLLNDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllndec::R](R) reader structure"]
impl crate::Readable for SYSPLLNDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllndec::W](W) writer structure"]
impl crate::Writable for SYSPLLNDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPLLNDEC to value 0"]
impl crate::Resettable for SYSPLLNDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
