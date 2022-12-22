#[doc = "Register `TSEVR` reader"]
pub struct R(crate::R<TSEVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSEVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSEVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSEVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSEVR` writer"]
pub struct W(crate::W<TSEVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEVR_SPEC>;
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
impl From<crate::W<TSEVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSEVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TimestampEvent` reader - Timestamp event."]
pub type TIMESTAMP_EVENT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TimestampEvent` writer - Timestamp event."]
pub type TIMESTAMP_EVENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSEVR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Timestamp event."]
    #[inline(always)]
    pub fn timestamp_event(&self) -> TIMESTAMP_EVENT_R {
        TIMESTAMP_EVENT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Timestamp event."]
    #[inline(always)]
    pub fn timestamp_event(&mut self) -> TIMESTAMP_EVENT_W<0> {
        TIMESTAMP_EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsevr](index.html) module"]
pub struct TSEVR_SPEC;
impl crate::RegisterSpec for TSEVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsevr::R](R) reader structure"]
impl crate::Readable for TSEVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsevr::W](W) writer structure"]
impl crate::Writable for TSEVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSEVR to value 0"]
impl crate::Resettable for TSEVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
