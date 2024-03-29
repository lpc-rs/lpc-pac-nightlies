#[doc = "Register `HCFMREMAINING` reader"]
pub struct R(crate::R<HCFMREMAINING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFMREMAINING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCFMREMAINING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCFMREMAINING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCFMREMAINING` writer"]
pub struct W(crate::W<HCFMREMAINING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCFMREMAINING_SPEC>;
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
impl From<crate::W<HCFMREMAINING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCFMREMAINING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FR` reader - FrameRemaining This counter is decremented at each bit time."]
pub type FR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FR` writer - FrameRemaining This counter is decremented at each bit time."]
pub type FR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCFMREMAINING_SPEC, u16, u16, 14, O>;
#[doc = "Field `FRT` reader - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
pub type FRT_R = crate::BitReader<bool>;
#[doc = "Field `FRT` writer - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
pub type FRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFMREMAINING_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub fn frt(&self) -> FRT_R {
        FRT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W<0> {
        FR_W::new(self)
    }
    #[doc = "Bit 31 - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub fn frt(&mut self) -> FRT_W<31> {
        FRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A 14-bit counter showing the bit time remaining in the current frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfmremaining](index.html) module"]
pub struct HCFMREMAINING_SPEC;
impl crate::RegisterSpec for HCFMREMAINING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfmremaining::R](R) reader structure"]
impl crate::Readable for HCFMREMAINING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcfmremaining::W](W) writer structure"]
impl crate::Writable for HCFMREMAINING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCFMREMAINING to value 0"]
impl crate::Resettable for HCFMREMAINING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
