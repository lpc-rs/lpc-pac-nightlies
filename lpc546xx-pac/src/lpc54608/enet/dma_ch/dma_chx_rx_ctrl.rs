#[doc = "Register `DMA_CHx_RX_CTRL` reader"]
pub struct R(crate::R<DMA_CHX_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_RX_CTRL` writer"]
pub struct W(crate::W<DMA_CHX_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_RX_CTRL_SPEC>;
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
impl From<crate::W<DMA_CHX_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `RBSZ` reader - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
pub type RBSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RBSZ` writer - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
pub type RBSZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CHX_RX_CTRL_SPEC, u16, u16, 12, O>;
#[doc = "Field `RxPBL` reader - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
pub type RX_PBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RxPBL` writer - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
pub type RX_PBL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CHX_RX_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `RPF` reader - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
pub type RPF_R = crate::BitReader<bool>;
#[doc = "Field `RPF` writer - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
pub type RPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_RX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 3) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn rx_pbl(&self) -> RX_PBL_R {
        RX_PBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    #[doc = "Bits 3:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W<3> {
        RBSZ_W::new(self)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn rx_pbl(&mut self) -> RX_PBL_W<16> {
        RX_PBL_W::new(self)
    }
    #[doc = "Bit 31 - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W<31> {
        RPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channelx Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_rx_ctrl](index.html) module"]
pub struct DMA_CHX_RX_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CHX_RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_rx_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_CHX_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_rx_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_CHX_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_RX_CTRL to value 0"]
impl crate::Resettable for DMA_CHX_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
