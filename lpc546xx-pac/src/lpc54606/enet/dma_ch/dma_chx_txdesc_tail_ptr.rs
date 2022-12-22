#[doc = "Register `DMA_CHx_TXDESC_TAIL_PTR` reader"]
pub struct R(crate::R<DMA_CHX_TXDESC_TAIL_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_TXDESC_TAIL_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_TXDESC_TAIL_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_TXDESC_TAIL_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_TXDESC_TAIL_PTR` writer"]
pub struct W(crate::W<DMA_CHX_TXDESC_TAIL_PTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_TXDESC_TAIL_PTR_SPEC>;
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
impl From<crate::W<DMA_CHX_TXDESC_TAIL_PTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_TXDESC_TAIL_PTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDTP` reader - Transmit Tail Pointer This field contains the tail pointer for the Tx ring."]
pub type TDTP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TDTP` writer - Transmit Tail Pointer This field contains the tail pointer for the Tx ring."]
pub type TDTP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CHX_TXDESC_TAIL_PTR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Transmit Tail Pointer This field contains the tail pointer for the Tx ring."]
    #[inline(always)]
    pub fn tdtp(&self) -> TDTP_R {
        TDTP_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Tail Pointer This field contains the tail pointer for the Tx ring."]
    #[inline(always)]
    pub fn tdtp(&mut self) -> TDTP_W<2> {
        TDTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_txdesc_tail_ptr](index.html) module"]
pub struct DMA_CHX_TXDESC_TAIL_PTR_SPEC;
impl crate::RegisterSpec for DMA_CHX_TXDESC_TAIL_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_txdesc_tail_ptr::R](R) reader structure"]
impl crate::Readable for DMA_CHX_TXDESC_TAIL_PTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_txdesc_tail_ptr::W](W) writer structure"]
impl crate::Writable for DMA_CHX_TXDESC_TAIL_PTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_TXDESC_TAIL_PTR to value 0"]
impl crate::Resettable for DMA_CHX_TXDESC_TAIL_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
