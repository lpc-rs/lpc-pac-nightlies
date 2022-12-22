#[doc = "Register `CRSR_IMG[%s]` reader"]
pub struct R(crate::R<CRSR_IMG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_IMG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_IMG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_IMG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRSR_IMG[%s]` writer"]
pub struct W(crate::W<CRSR_IMG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_IMG_SPEC>;
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
impl From<crate::W<CRSR_IMG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_IMG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSR_IMG` reader - Cursor Image data."]
pub type CRSR_IMG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRSR_IMG` writer - Cursor Image data."]
pub type CRSR_IMG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRSR_IMG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Cursor Image data."]
    #[inline(always)]
    pub fn crsr_img(&self) -> CRSR_IMG_R {
        CRSR_IMG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cursor Image data."]
    #[inline(always)]
    pub fn crsr_img(&mut self) -> CRSR_IMG_W<0> {
        CRSR_IMG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursor Image registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_img](index.html) module"]
pub struct CRSR_IMG_SPEC;
impl crate::RegisterSpec for CRSR_IMG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_img::R](R) reader structure"]
impl crate::Readable for CRSR_IMG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crsr_img::W](W) writer structure"]
impl crate::Writable for CRSR_IMG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRSR_IMG[%s]
to value 0"]
impl crate::Resettable for CRSR_IMG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
