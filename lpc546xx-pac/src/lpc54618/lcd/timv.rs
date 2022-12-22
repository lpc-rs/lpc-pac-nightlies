#[doc = "Register `TIMV` reader"]
pub struct R(crate::R<TIMV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMV` writer"]
pub struct W(crate::W<TIMV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMV_SPEC>;
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
impl From<crate::W<TIMV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPP` reader - Lines per panel."]
pub type LPP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LPP` writer - Lines per panel."]
pub type LPP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMV_SPEC, u16, u16, 10, O>;
#[doc = "Field `VSW` reader - Vertical synchronization pulse width."]
pub type VSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSW` writer - Vertical synchronization pulse width."]
pub type VSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMV_SPEC, u8, u8, 6, O>;
#[doc = "Field `VFP` reader - Vertical front porch."]
pub type VFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VFP` writer - Vertical front porch."]
pub type VFP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMV_SPEC, u8, u8, 8, O>;
#[doc = "Field `VBP` reader - Vertical back porch."]
pub type VBP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBP` writer - Vertical back porch."]
pub type VBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&self) -> LPP_R {
        LPP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&self) -> VSW_R {
        VSW_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&mut self) -> LPP_W<0> {
        LPP_W::new(self)
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&mut self) -> VSW_W<10> {
        VSW_W::new(self)
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W<16> {
        VFP_W::new(self)
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W<24> {
        VBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical Timing Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timv](index.html) module"]
pub struct TIMV_SPEC;
impl crate::RegisterSpec for TIMV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timv::R](R) reader structure"]
impl crate::Readable for TIMV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timv::W](W) writer structure"]
impl crate::Writable for TIMV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMV to value 0"]
impl crate::Resettable for TIMV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
