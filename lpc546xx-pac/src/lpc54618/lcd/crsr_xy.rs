#[doc = "Register `CRSR_XY` reader"]
pub struct R(crate::R<CRSR_XY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_XY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_XY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_XY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRSR_XY` writer"]
pub struct W(crate::W<CRSR_XY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_XY_SPEC>;
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
impl From<crate::W<CRSR_XY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_XY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRSRX` reader - X ordinate of the cursor origin measured in pixels."]
pub type CRSRX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRSRX` writer - X ordinate of the cursor origin measured in pixels."]
pub type CRSRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRSR_XY_SPEC, u16, u16, 10, O>;
#[doc = "Field `CRSRY` reader - Y ordinate of the cursor origin measured in pixels."]
pub type CRSRY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRSRY` writer - Y ordinate of the cursor origin measured in pixels."]
pub type CRSRY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRSR_XY_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - X ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsrx(&self) -> CRSRX_R {
        CRSRX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Y ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsry(&self) -> CRSRY_R {
        CRSRY_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - X ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsrx(&mut self) -> CRSRX_W<0> {
        CRSRX_W::new(self)
    }
    #[doc = "Bits 16:25 - Y ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsry(&mut self) -> CRSRY_W<16> {
        CRSRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursor XY Position register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_xy](index.html) module"]
pub struct CRSR_XY_SPEC;
impl crate::RegisterSpec for CRSR_XY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_xy::R](R) reader structure"]
impl crate::Readable for CRSR_XY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crsr_xy::W](W) writer structure"]
impl crate::Writable for CRSR_XY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRSR_XY to value 0"]
impl crate::Resettable for CRSR_XY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
