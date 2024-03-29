#[doc = "Register `EVFLAG` reader"]
pub struct R(crate::R<EVFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVFLAG` writer"]
pub struct W(crate::W<EVFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVFLAG_SPEC>;
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
impl From<crate::W<EVFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG` reader - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type FLAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLAG` writer - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type FLAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVFLAG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W<0> {
        FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT event flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflag](index.html) module"]
pub struct EVFLAG_SPEC;
impl crate::RegisterSpec for EVFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evflag::R](R) reader structure"]
impl crate::Readable for EVFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evflag::W](W) writer structure"]
impl crate::Writable for EVFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVFLAG to value 0"]
impl crate::Resettable for EVFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
