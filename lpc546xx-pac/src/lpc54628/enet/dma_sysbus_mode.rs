#[doc = "Register `DMA_SYSBUS_MODE` reader"]
pub struct R(crate::R<DMA_SYSBUS_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SYSBUS_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SYSBUS_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SYSBUS_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SYSBUS_MODE` writer"]
pub struct W(crate::W<DMA_SYSBUS_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SYSBUS_MODE_SPEC>;
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
impl From<crate::W<DMA_SYSBUS_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SYSBUS_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB` reader - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE)."]
pub type FB_R = crate::BitReader<bool>;
#[doc = "Field `FB` writer - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE)."]
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SYSBUS_MODE_SPEC, bool, O>;
#[doc = "Field `AAL` reader - Address-Aligned Beats When this bit is set to 1, the AHB master performs address-aligned burst transfers on Read and Write channels."]
pub type AAL_R = crate::BitReader<bool>;
#[doc = "Field `AAL` writer - Address-Aligned Beats When this bit is set to 1, the AHB master performs address-aligned burst transfers on Read and Write channels."]
pub type AAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SYSBUS_MODE_SPEC, bool, O>;
#[doc = "Field `MB` reader - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more."]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more."]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SYSBUS_MODE_SPEC, bool, O>;
#[doc = "Field `RB` reader - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or EarlyBurst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLEtransfers."]
pub type RB_R = crate::BitReader<bool>;
#[doc = "Field `RB` writer - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or EarlyBurst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLEtransfers."]
pub type RB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SYSBUS_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE)."]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats When this bit is set to 1, the AHB master performs address-aligned burst transfers on Read and Write channels."]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more."]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or EarlyBurst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLEtransfers."]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length When this bit is set to 1, the AHB master will initiate burst transfers of specified length (INCRx or SINGLE)."]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<0> {
        FB_W::new(self)
    }
    #[doc = "Bit 12 - Address-Aligned Beats When this bit is set to 1, the AHB master performs address-aligned burst transfers on Read and Write channels."]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W<12> {
        AAL_W::new(self)
    }
    #[doc = "Bit 14 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master performs undefined bursts transfers (INCR) for burst length of 16 or more."]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<14> {
        MB_W::new(self)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst When this bit is set high and the AHB master gets SPLIT, RETRY, or EarlyBurst Termination (EBT) response, the AHB master interface rebuilds the pending beats of any initiated burst transfer with INCRx and SINGLEtransfers."]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W<15> {
        RB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA System Bus mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sysbus_mode](index.html) module"]
pub struct DMA_SYSBUS_MODE_SPEC;
impl crate::RegisterSpec for DMA_SYSBUS_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_sysbus_mode::R](R) reader structure"]
impl crate::Readable for DMA_SYSBUS_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_sysbus_mode::W](W) writer structure"]
impl crate::Writable for DMA_SYSBUS_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SYSBUS_MODE to value 0"]
impl crate::Resettable for DMA_SYSBUS_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
