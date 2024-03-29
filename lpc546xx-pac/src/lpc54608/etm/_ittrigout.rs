#[doc = "Register `_ITTRIGOUT` reader"]
pub struct R(crate::R<_ITTRIGOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_ITTRIGOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_ITTRIGOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_ITTRIGOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_ITTRIGOUT` writer"]
pub struct W(crate::W<_ITTRIGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_ITTRIGOUT_SPEC>;
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
impl From<crate::W<_ITTRIGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_ITTRIGOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` writer - A write to this bit sets the TRIGGER output."]
pub type TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, _ITTRIGOUT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - A write to this bit sets the TRIGGER output."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W<0> {
        TRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration Test Trigger Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_ittrigout](index.html) module"]
pub struct _ITTRIGOUT_SPEC;
impl crate::RegisterSpec for _ITTRIGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_ittrigout::R](R) reader structure"]
impl crate::Readable for _ITTRIGOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_ittrigout::W](W) writer structure"]
impl crate::Writable for _ITTRIGOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _ITTRIGOUT to value 0"]
impl crate::Resettable for _ITTRIGOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
