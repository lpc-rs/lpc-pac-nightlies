#[doc = "Register `MTL_TXQx_DBG` reader"]
pub struct R(crate::R<MTL_TXQX_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXQPAUSED` reader - Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
pub type TXQPAUSED_R = crate::BitReader<bool>;
#[doc = "Field `TRCSTS` reader - MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller: 00: Idle state 01: Read state (transferring data to the MAC transmitter) 10: Waiting for pending Tx Status from the MAC transmitter 11: Flushing the Tx queue because of the Packet Abort request from the MAC."]
pub type TRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWCSTS` reader - MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
pub type TWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXQSTS` reader - MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
pub type TXQSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
pub type TXSTSFSTS_R = crate::BitReader<bool>;
#[doc = "Field `PTXQ` reader - Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
pub type PTXQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STSXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
pub type STSXSTSF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller: 00: Idle state 01: Read state (transferring data to the MAC transmitter) 10: Waiting for pending Tx Status from the MAC transmitter 11: Flushing the Tx queue because of the Packet Abort request from the MAC."]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
    #[inline(always)]
    pub fn stsxstsf(&self) -> STSXSTSF_R {
        STSXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "MTL TxQx Debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_dbg](index.html) module"]
pub struct MTL_TXQX_DBG_SPEC;
impl crate::RegisterSpec for MTL_TXQX_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_dbg::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_DBG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTL_TXQx_DBG to value 0"]
impl crate::Resettable for MTL_TXQX_DBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
