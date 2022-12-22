#[doc = "Register `CRSR_CLIP` reader"]
pub struct R(crate::R<CRSR_CLIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_CLIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_CLIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_CLIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRSR_CLIP` writer"]
pub struct W(crate::W<CRSR_CLIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_CLIP_SPEC>;
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
impl From<crate::W<CRSR_CLIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_CLIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSRCLIPX` reader - Cursor clip position for X direction."]
pub type CRSRCLIPX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRSRCLIPX` writer - Cursor clip position for X direction."]
pub type CRSRCLIPX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRSR_CLIP_SPEC, u8, u8, 6, O>;
#[doc = "Field `CRSRCLIPY` reader - Cursor clip position for Y direction."]
pub type CRSRCLIPY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRSRCLIPY` writer - Cursor clip position for Y direction."]
pub type CRSRCLIPY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRSR_CLIP_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Cursor clip position for X direction."]
    #[inline(always)]
    pub fn crsrclipx(&self) -> CRSRCLIPX_R {
        CRSRCLIPX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Cursor clip position for Y direction."]
    #[inline(always)]
    pub fn crsrclipy(&self) -> CRSRCLIPY_R {
        CRSRCLIPY_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Cursor clip position for X direction."]
    #[inline(always)]
    pub fn crsrclipx(&mut self) -> CRSRCLIPX_W<0> {
        CRSRCLIPX_W::new(self)
    }
    #[doc = "Bits 8:13 - Cursor clip position for Y direction."]
    #[inline(always)]
    pub fn crsrclipy(&mut self) -> CRSRCLIPY_W<8> {
        CRSRCLIPY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursor Clip Position register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_clip](index.html) module"]
pub struct CRSR_CLIP_SPEC;
impl crate::RegisterSpec for CRSR_CLIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_clip::R](R) reader structure"]
impl crate::Readable for CRSR_CLIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crsr_clip::W](W) writer structure"]
impl crate::Writable for CRSR_CLIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRSR_CLIP to value 0"]
impl crate::Resettable for CRSR_CLIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
