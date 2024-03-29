#[doc = "Register `TRIGGER` reader"]
pub struct R(crate::R<TRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGGER` writer"]
pub struct W(crate::W<TRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGGER_SPEC>;
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
impl From<crate::W<TRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TriggerEvent` reader - Trigger event"]
pub type TRIGGER_EVENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TriggerEvent` writer - Trigger event"]
pub type TRIGGER_EVENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIGGER_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - Trigger event"]
    #[inline(always)]
    pub fn trigger_event(&self) -> TRIGGER_EVENT_R {
        TRIGGER_EVENT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Trigger event"]
    #[inline(always)]
    pub fn trigger_event(&mut self) -> TRIGGER_EVENT_W<0> {
        TRIGGER_EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](index.html) module"]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigger::R](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigger::W](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
