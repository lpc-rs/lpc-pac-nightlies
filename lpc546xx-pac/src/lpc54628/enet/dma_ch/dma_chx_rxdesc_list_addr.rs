#[doc = "Register `DMA_CHx_RXDESC_LIST_ADDR` reader"]
pub struct R(crate::R<DMA_CHX_RXDESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_RXDESC_LIST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_RXDESC_LIST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_RXDESC_LIST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_RXDESC_LIST_ADDR` writer"]
pub struct W(crate::W<DMA_CHX_RXDESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_RXDESC_LIST_ADDR_SPEC>;
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
impl From<crate::W<DMA_CHX_RXDESC_LIST_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_RXDESC_LIST_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRL` reader - Start of receive list This field contains the base address of the First in the Receive list."]
pub type SRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRL` writer - Start of receive list This field contains the base address of the First in the Receive list."]
pub type SRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CHX_RXDESC_LIST_ADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Start of receive list This field contains the base address of the First in the Receive list."]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of receive list This field contains the base address of the First in the Receive list."]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W<2> {
        SRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_rxdesc_list_addr](index.html) module"]
pub struct DMA_CHX_RXDESC_LIST_ADDR_SPEC;
impl crate::RegisterSpec for DMA_CHX_RXDESC_LIST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_rxdesc_list_addr::R](R) reader structure"]
impl crate::Readable for DMA_CHX_RXDESC_LIST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_rxdesc_list_addr::W](W) writer structure"]
impl crate::Writable for DMA_CHX_RXDESC_LIST_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_RXDESC_LIST_ADDR to value 0"]
impl crate::Resettable for DMA_CHX_RXDESC_LIST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
