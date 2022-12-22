#[doc = "Register `DYNAMICRFC` reader"]
pub struct R(crate::R<DYNAMICRFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICRFC` writer"]
pub struct W(crate::W<DYNAMICRFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRFC_SPEC>;
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
impl From<crate::W<DYNAMICRFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRFC` reader - Auto-refresh period and auto-refresh to active command period."]
pub type TRFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRFC` writer - Auto-refresh period and auto-refresh to active command period."]
pub type TRFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICRFC_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Auto-refresh period and auto-refresh to active command period."]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Auto-refresh period and auto-refresh to active command period."]
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W<0> {
        TRFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the auto-refresh period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrfc](index.html) module"]
pub struct DYNAMICRFC_SPEC;
impl crate::RegisterSpec for DYNAMICRFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicrfc::R](R) reader structure"]
impl crate::Readable for DYNAMICRFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicrfc::W](W) writer structure"]
impl crate::Writable for DYNAMICRFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICRFC to value 0x1f"]
impl crate::Resettable for DYNAMICRFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
