#[doc = "Register `INTMSK` reader"]
pub struct R(crate::R<INTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTMSK` writer"]
pub struct W(crate::W<INTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTMSK_SPEC>;
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
impl From<crate::W<INTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUFIM` reader - FIFO underflow interrupt enable."]
pub type FUFIM_R = crate::BitReader<bool>;
#[doc = "Field `FUFIM` writer - FIFO underflow interrupt enable."]
pub type FUFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMSK_SPEC, bool, O>;
#[doc = "Field `LNBUIM` reader - LCD next base address update interrupt enable."]
pub type LNBUIM_R = crate::BitReader<bool>;
#[doc = "Field `LNBUIM` writer - LCD next base address update interrupt enable."]
pub type LNBUIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMSK_SPEC, bool, O>;
#[doc = "Field `VCOMPIM` reader - Vertical compare interrupt enable."]
pub type VCOMPIM_R = crate::BitReader<bool>;
#[doc = "Field `VCOMPIM` writer - Vertical compare interrupt enable."]
pub type VCOMPIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMSK_SPEC, bool, O>;
#[doc = "Field `BERIM` reader - AHB master error interrupt enable."]
pub type BERIM_R = crate::BitReader<bool>;
#[doc = "Field `BERIM` writer - AHB master error interrupt enable."]
pub type BERIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - FIFO underflow interrupt enable."]
    #[inline(always)]
    pub fn fufim(&self) -> FUFIM_R {
        FUFIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable."]
    #[inline(always)]
    pub fn lnbuim(&self) -> LNBUIM_R {
        LNBUIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable."]
    #[inline(always)]
    pub fn vcompim(&self) -> VCOMPIM_R {
        VCOMPIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable."]
    #[inline(always)]
    pub fn berim(&self) -> BERIM_R {
        BERIM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FIFO underflow interrupt enable."]
    #[inline(always)]
    pub fn fufim(&mut self) -> FUFIM_W<1> {
        FUFIM_W::new(self)
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable."]
    #[inline(always)]
    pub fn lnbuim(&mut self) -> LNBUIM_W<2> {
        LNBUIM_W::new(self)
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable."]
    #[inline(always)]
    pub fn vcompim(&mut self) -> VCOMPIM_W<3> {
        VCOMPIM_W::new(self)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable."]
    #[inline(always)]
    pub fn berim(&mut self) -> BERIM_W<4> {
        BERIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmsk](index.html) module"]
pub struct INTMSK_SPEC;
impl crate::RegisterSpec for INTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intmsk::R](R) reader structure"]
impl crate::Readable for INTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intmsk::W](W) writer structure"]
impl crate::Writable for INTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTMSK to value 0"]
impl crate::Resettable for INTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
