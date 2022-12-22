#[doc = "Register `DMA_DBG_STAT` reader"]
pub struct R(crate::R<DMA_DBG_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_DBG_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_DBG_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_DBG_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_DBG_STAT` writer"]
pub struct W(crate::W<DMA_DBG_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_DBG_STAT_SPEC>;
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
impl From<crate::W<DMA_DBG_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_DBG_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHSTS` reader - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
pub type AHSTS_R = crate::BitReader<bool>;
#[doc = "Field `AHSTS` writer - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
pub type AHSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_DBG_STAT_SPEC, bool, O>;
#[doc = "Field `RPS0` reader - DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0: 0x0: Stopped (Reset or Stop Receive Command issued) 0x1: Running (Fetching Rx Transfer ) 0x2: Reserved 0x3: Running (Waiting for Rx packet) 0x4: Suspended (Rx Unavailable) 0x5: Running (Closing the Rx) 0x6: Timestamp write state 0x7: Running (Transferring the received packet data from the Rx buffer to the system memory) This field does not generate an interrupt."]
pub type RPS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPS0` reader - DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0: 000: Stopped (Reset or Stop Transmit Command issued) 0x1: Running (Fetching Tx Transfer) 0x2: Running (Waiting for status) 0x3: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO)) 0x4: Timestamp write state 0x5: Reserved for future use 0x6: Suspended (Tx Unavailable or Tx Buffer Underflow) 0x7: Running (Closing Tx ) This field does not generate an interrupt."]
pub type TPS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPS1` reader - DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1."]
pub type RPS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPS1` reader - DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1."]
pub type TPS1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&self) -> AHSTS_R {
        AHSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0: 0x0: Stopped (Reset or Stop Receive Command issued) 0x1: Running (Fetching Rx Transfer ) 0x2: Reserved 0x3: Running (Waiting for Rx packet) 0x4: Suspended (Rx Unavailable) 0x5: Running (Closing the Rx) 0x6: Timestamp write state 0x7: Running (Transferring the received packet data from the Rx buffer to the system memory) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0: 000: Stopped (Reset or Stop Transmit Command issued) 0x1: Running (Fetching Tx Transfer) 0x2: Running (Waiting for status) 0x3: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO)) 0x4: Timestamp write state 0x5: Reserved for future use 0x6: Suspended (Tx Unavailable or Tx Buffer Underflow) 0x7: Running (Closing Tx ) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&mut self) -> AHSTS_W<0> {
        AHSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Debug Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dbg_stat](index.html) module"]
pub struct DMA_DBG_STAT_SPEC;
impl crate::RegisterSpec for DMA_DBG_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_dbg_stat::R](R) reader structure"]
impl crate::Readable for DMA_DBG_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_dbg_stat::W](W) writer structure"]
impl crate::Writable for DMA_DBG_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_DBG_STAT to value 0"]
impl crate::Resettable for DMA_DBG_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
