#[doc = "Register `TIMH` reader"]
pub struct R(crate::R<TIMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMH` writer"]
pub struct W(crate::W<TIMH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMH_SPEC>;
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
impl From<crate::W<TIMH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPL` reader - Pixels-per-line."]
pub type PPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPL` writer - Pixels-per-line."]
pub type PPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMH_SPEC, u8, u8, 6, O>;
#[doc = "Field `HSW` reader - Horizontal synchronization pulse width."]
pub type HSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSW` writer - Horizontal synchronization pulse width."]
pub type HSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMH_SPEC, u8, u8, 8, O>;
#[doc = "Field `HFP` reader - Horizontal front porch."]
pub type HFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFP` writer - Horizontal front porch."]
pub type HFP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMH_SPEC, u8, u8, 8, O>;
#[doc = "Field `HBP` reader - Horizontal back porch."]
pub type HBP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBP` writer - Horizontal back porch."]
pub type HBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&self) -> PPL_R {
        PPL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&self) -> HFP_R {
        HFP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&mut self) -> PPL_W<2> {
        PPL_W::new(self)
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W<8> {
        HSW_W::new(self)
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&mut self) -> HFP_W<16> {
        HFP_W::new(self)
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&mut self) -> HBP_W<24> {
        HBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal Timing Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timh](index.html) module"]
pub struct TIMH_SPEC;
impl crate::RegisterSpec for TIMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timh::R](R) reader structure"]
impl crate::Readable for TIMH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timh::W](W) writer structure"]
impl crate::Writable for TIMH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMH to value 0"]
impl crate::Resettable for TIMH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
