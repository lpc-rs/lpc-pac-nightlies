#[doc = "Register `ETSCV` reader"]
pub struct R(crate::R<ETSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETSCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETSCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETSCV` writer"]
pub struct W(crate::W<ETSCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETSCV_SPEC>;
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
impl From<crate::W<ETSCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETSCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETSC` reader - External timestamp counter."]
pub type ETSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ETSC` writer - External timestamp counter."]
pub type ETSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETSCV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&self) -> ETSC_R {
        ETSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&mut self) -> ETSC_W<0> {
        ETSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Timestamp Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etscv](index.html) module"]
pub struct ETSCV_SPEC;
impl crate::RegisterSpec for ETSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etscv::R](R) reader structure"]
impl crate::Readable for ETSCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etscv::W](W) writer structure"]
impl crate::Writable for ETSCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETSCV to value 0"]
impl crate::Resettable for ETSCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
