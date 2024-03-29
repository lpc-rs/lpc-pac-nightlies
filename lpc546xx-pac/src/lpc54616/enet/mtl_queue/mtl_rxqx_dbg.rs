#[doc = "Register `MTL_RXQx_DBG` reader"]
pub struct R(crate::R<MTL_RXQX_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQX_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQX_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQX_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_RXQx_DBG` writer"]
pub struct W(crate::W<MTL_RXQX_DBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQX_DBG_SPEC>;
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
impl From<crate::W<MTL_RXQX_DBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQX_DBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
pub type RWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `RWCSTS` writer - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
pub type RWCSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQX_DBG_SPEC, bool, O>;
#[doc = "Field `RRCSTS` reader - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller: 00: Idle state 01: Reading packet data 10: Reading packet status (or timestamp) 11: Flushing the packet data and status."]
pub type RRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue: 0x0: Rx Queue empty 0x1: Rx Queue fill-level below flow-control deactivate threshold 0x2: Rx Queue fill-level above flow-control activate threshold 0x3: Rx Queue full."]
pub type RXQSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRXQ` reader - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
pub type PRXQ_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller: 00: Idle state 01: Reading packet data 10: Reading packet status (or timestamp) 11: Flushing the packet data and status."]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue: 0x0: Rx Queue empty 0x1: Rx Queue fill-level below flow-control deactivate threshold 0x2: Rx Queue fill-level above flow-control activate threshold 0x3: Rx Queue full."]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    #[inline(always)]
    pub fn rwcsts(&mut self) -> RWCSTS_W<0> {
        RWCSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL RxQx Debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_dbg](index.html) module"]
pub struct MTL_RXQX_DBG_SPEC;
impl crate::RegisterSpec for MTL_RXQX_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_rxqx_dbg::R](R) reader structure"]
impl crate::Readable for MTL_RXQX_DBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_dbg::W](W) writer structure"]
impl crate::Writable for MTL_RXQX_DBG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_RXQx_DBG to value 0"]
impl crate::Resettable for MTL_RXQX_DBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
