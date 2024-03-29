#[doc = "Register `DYNAMICXSR` reader"]
pub struct R(crate::R<DYNAMICXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICXSR` writer"]
pub struct W(crate::W<DYNAMICXSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICXSR_SPEC>;
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
impl From<crate::W<DYNAMICXSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICXSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSR` reader - Exit self-refresh to active command time."]
pub type TXSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSR` writer - Exit self-refresh to active command time."]
pub type TXSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICXSR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Exit self-refresh to active command time."]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Exit self-refresh to active command time."]
    #[inline(always)]
    pub fn txsr(&mut self) -> TXSR_W<0> {
        TXSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time for exit self-refresh to active command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicxsr](index.html) module"]
pub struct DYNAMICXSR_SPEC;
impl crate::RegisterSpec for DYNAMICXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicxsr::R](R) reader structure"]
impl crate::Readable for DYNAMICXSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicxsr::W](W) writer structure"]
impl crate::Writable for DYNAMICXSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICXSR to value 0x1f"]
impl crate::Resettable for DYNAMICXSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
