#[doc = "Register `MTL_RXQ_DMA_MAP` reader"]
pub struct R(crate::R<MTL_RXQ_DMA_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQ_DMA_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQ_DMA_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQ_DMA_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_RXQ_DMA_MAP` writer"]
pub struct W(crate::W<MTL_RXQ_DMA_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQ_DMA_MAP_SPEC>;
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
impl From<crate::W<MTL_RXQ_DMA_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQ_DMA_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Q0MDMACH` reader - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
pub type Q0MDMACH_R = crate::BitReader<bool>;
#[doc = "Field `Q0MDMACH` writer - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
pub type Q0MDMACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQ_DMA_MAP_SPEC, bool, O>;
#[doc = "Field `Q0DDMACH` reader - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
pub type Q0DDMACH_R = crate::BitReader<bool>;
#[doc = "Field `Q0DDMACH` writer - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
pub type Q0DDMACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQ_DMA_MAP_SPEC, bool, O>;
#[doc = "Field `Q1MDMACH` reader - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
pub type Q1MDMACH_R = crate::BitReader<bool>;
#[doc = "Field `Q1MDMACH` writer - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
pub type Q1MDMACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQ_DMA_MAP_SPEC, bool, O>;
#[doc = "Field `Q1DDMACH` reader - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
pub type Q1DDMACH_R = crate::BitReader<bool>;
#[doc = "Field `Q1DDMACH` writer - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
pub type Q1DDMACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQ_DMA_MAP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
    #[inline(always)]
    pub fn q0mdmach(&self) -> Q0MDMACH_R {
        Q0MDMACH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q0ddmach(&self) -> Q0DDMACH_R {
        Q0DDMACH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
    #[inline(always)]
    pub fn q1mdmach(&self) -> Q1MDMACH_R {
        Q1MDMACH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q1ddmach(&self) -> Q1DDMACH_R {
        Q1DDMACH_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
    #[inline(always)]
    pub fn q0mdmach(&mut self) -> Q0MDMACH_W<0> {
        Q0MDMACH_W::new(self)
    }
    #[doc = "Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q0ddmach(&mut self) -> Q0DDMACH_W<4> {
        Q0DDMACH_W::new(self)
    }
    #[doc = "Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
    #[inline(always)]
    pub fn q1mdmach(&mut self) -> Q1MDMACH_W<8> {
        Q1MDMACH_W::new(self)
    }
    #[doc = "Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q1ddmach(&mut self) -> Q1DDMACH_W<12> {
        Q1DDMACH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL Receive Queue and DMA Channel Mapping register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxq_dma_map](index.html) module"]
pub struct MTL_RXQ_DMA_MAP_SPEC;
impl crate::RegisterSpec for MTL_RXQ_DMA_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_rxq_dma_map::R](R) reader structure"]
impl crate::Readable for MTL_RXQ_DMA_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_rxq_dma_map::W](W) writer structure"]
impl crate::Writable for MTL_RXQ_DMA_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_RXQ_DMA_MAP to value 0"]
impl crate::Resettable for MTL_RXQ_DMA_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
