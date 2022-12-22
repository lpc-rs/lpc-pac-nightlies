#[doc = "Register `DMA_CHx_TX_CTRL` reader"]
pub struct R(crate::R<DMA_CHX_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_TX_CTRL` writer"]
pub struct W(crate::W<DMA_CHX_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_TX_CTRL_SPEC>;
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
impl From<crate::W<DMA_CHX_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_TX_CTRL_SPEC, bool, O>;
#[doc = "Field `TCW` reader - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel."]
pub type TCW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCW` writer - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel."]
pub type TCW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_CHX_TX_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `OSF` reader - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
pub type OSF_R = crate::BitReader<bool>;
#[doc = "Field `OSF` writer - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_TX_CTRL_SPEC, bool, O>;
#[doc = "Field `TxPBL` reader - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
pub type TX_PBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TxPBL` writer - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
pub type TX_PBL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CHX_TX_CTRL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel."]
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn tx_pbl(&self) -> TX_PBL_R {
        TX_PBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<0> {
        ST_W::new(self)
    }
    #[doc = "Bits 1:3 - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel."]
    #[inline(always)]
    pub fn tcw(&mut self) -> TCW_W<1> {
        TCW_W::new(self)
    }
    #[doc = "Bit 4 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<4> {
        OSF_W::new(self)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn tx_pbl(&mut self) -> TX_PBL_W<16> {
        TX_PBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channelx Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_tx_ctrl](index.html) module"]
pub struct DMA_CHX_TX_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CHX_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_tx_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_CHX_TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_tx_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_CHX_TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_TX_CTRL to value 0"]
impl crate::Resettable for DMA_CHX_TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
