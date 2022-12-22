#[doc = "Register `DMA_CHx_STAT` reader"]
pub struct R(crate::R<DMA_CHX_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_STAT` writer"]
pub struct W(crate::W<DMA_CHX_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_STAT_SPEC>;
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
impl From<crate::W<DMA_CHX_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt This bit indicates that the packet transmission is complete."]
pub type TI_R = crate::BitReader<bool>;
#[doc = "Field `TI` writer - Transmit Interrupt This bit indicates that the packet transmission is complete."]
pub type TI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `TPS` reader - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TPS_R = crate::BitReader<bool>;
#[doc = "Field `TPS` writer - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `TBU` reader - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the transmit list, and the DMA cannot acquire it."]
pub type TBU_R = crate::BitReader<bool>;
#[doc = "Field `TBU` writer - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the transmit list, and the DMA cannot acquire it."]
pub type TBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `RI` reader - Receive Interrupt This bit indicates that the packet reception is complete."]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - Receive Interrupt This bit indicates that the packet reception is complete."]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `RBU` reader - Receive Buffer Unavailable This bit indicates that the application owns the next in the receive list, and the DMA cannot acquire it."]
pub type RBU_R = crate::BitReader<bool>;
#[doc = "Field `RBU` writer - Receive Buffer Unavailable This bit indicates that the application owns the next in the receive list, and the DMA cannot acquire it."]
pub type RBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `RPS` reader - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
pub type RPS_R = crate::BitReader<bool>;
#[doc = "Field `RPS` writer - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
pub type RPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `RWT` reader - Receive Watchdog time out This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
pub type RWT_R = crate::BitReader<bool>;
#[doc = "Field `RWT` writer - Receive Watchdog time out This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
pub type RWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
pub type ETI_R = crate::BitReader<bool>;
#[doc = "Field `ETI` writer - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
pub type ETI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `ERI` reader - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet."]
pub type ERI_R = crate::BitReader<bool>;
#[doc = "Field `ERI` writer - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet."]
pub type ERI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `FBE` reader - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
pub type FBE_R = crate::BitReader<bool>;
#[doc = "Field `FBE` writer - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
pub type FBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Ear1y Transmit Interrupt Bit 12: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Ear1y Transmit Interrupt Bit 12: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
pub type AIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA Channel Interrupt Enable register Table 778) affect the Normal Interrupt Summary bit."]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA Channel Interrupt Enable register Table 778) affect the Normal Interrupt Summary bit."]
pub type NIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_STAT_SPEC, bool, O>;
#[doc = "Field `EB` reader - DMA Error Bits This field indicates the type of error that caused a Bus Error."]
pub type EB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EB` writer - DMA Error Bits This field indicates the type of error that caused a Bus Error."]
pub type EB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_CHX_STAT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the packet transmission is complete."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the transmit list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the packet reception is complete."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the application owns the next in the receive list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog time out This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet."]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Ear1y Transmit Interrupt Bit 12: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA Channel Interrupt Enable register Table 778) affect the Normal Interrupt Summary bit."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the packet transmission is complete."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W<0> {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W<1> {
        TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the transmit list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W<2> {
        TBU_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the packet reception is complete."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W<6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the application owns the next in the receive list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn rbu(&mut self) -> RBU_W<7> {
        RBU_W::new(self)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W<8> {
        RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog time out This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<9> {
        RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
    #[inline(always)]
    pub fn eti(&mut self) -> ETI_W<10> {
        ETI_W::new(self)
    }
    #[doc = "Bit 11 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet."]
    #[inline(always)]
    pub fn eri(&mut self) -> ERI_W<11> {
        ERI_W::new(self)
    }
    #[doc = "Bit 12 - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W<12> {
        FBE_W::new(self)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Ear1y Transmit Interrupt Bit 12: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W<14> {
        AIS_W::new(self)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA Channel Interrupt Enable register Table 778) affect the Normal Interrupt Summary bit."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W<15> {
        NIS_W::new(self)
    }
    #[doc = "Bits 16:18 - DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    #[inline(always)]
    pub fn eb(&mut self) -> EB_W<16> {
        EB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channelx DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_stat](index.html) module"]
pub struct DMA_CHX_STAT_SPEC;
impl crate::RegisterSpec for DMA_CHX_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_stat::R](R) reader structure"]
impl crate::Readable for DMA_CHX_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_stat::W](W) writer structure"]
impl crate::Writable for DMA_CHX_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_STAT to value 0"]
impl crate::Resettable for DMA_CHX_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
