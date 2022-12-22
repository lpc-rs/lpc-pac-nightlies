#[doc = "Register `FMSSTOP` reader"]
pub struct R(crate::R<FMSSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSSTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSSTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMSSTOP` writer"]
pub struct W(crate::W<FMSSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSSTOP_SPEC>;
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
impl From<crate::W<FMSSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - Stop address for signature generation (the word specified by STOP is included in the address range)."]
pub type STOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STOP` writer - Stop address for signature generation (the word specified by STOP is included in the address range)."]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMSSTOP_SPEC, u32, u32, 17, O>;
#[doc = "Field `SIG_START` reader - When this bit is written to 1, signature generation starts."]
pub type SIG_START_R = crate::BitReader<bool>;
#[doc = "Field `SIG_START` writer - When this bit is written to 1, signature generation starts."]
pub type SIG_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMSSTOP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range)."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - When this bit is written to 1, signature generation starts."]
    #[inline(always)]
    pub fn sig_start(&self) -> SIG_START_R {
        SIG_START_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range)."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<0> {
        STOP_W::new(self)
    }
    #[doc = "Bit 17 - When this bit is written to 1, signature generation starts."]
    #[inline(always)]
    pub fn sig_start(&mut self) -> SIG_START_W<17> {
        SIG_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature stop-address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstop](index.html) module"]
pub struct FMSSTOP_SPEC;
impl crate::RegisterSpec for FMSSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsstop::R](R) reader structure"]
impl crate::Readable for FMSSTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmsstop::W](W) writer structure"]
impl crate::Writable for FMSSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMSSTOP to value 0"]
impl crate::Resettable for FMSSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
