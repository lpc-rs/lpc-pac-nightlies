#[doc = "Register `MTL_RXQx_CTRL` reader"]
pub struct R(crate::R<MTL_RXQX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_RXQx_CTRL` writer"]
pub struct W(crate::W<MTL_RXQX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQX_CTRL_SPEC>;
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
impl From<crate::W<MTL_RXQX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXQ_WEGT` reader - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
pub type RXQ_WEGT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQ_WEGT` writer - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
pub type RXQ_WEGT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MTL_RXQX_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXQ_FRM_ARBIT` reader - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
pub type RXQ_FRM_ARBIT_R = crate::BitReader<bool>;
#[doc = "Field `RXQ_FRM_ARBIT` writer - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
pub type RXQ_FRM_ARBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&mut self) -> RXQ_WEGT_W<0> {
        RXQ_WEGT_W::new(self)
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&mut self) -> RXQ_FRM_ARBIT_W<3> {
        RXQ_FRM_ARBIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL RxQx Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_ctrl](index.html) module"]
pub struct MTL_RXQX_CTRL_SPEC;
impl crate::RegisterSpec for MTL_RXQX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_rxqx_ctrl::R](R) reader structure"]
impl crate::Readable for MTL_RXQX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_ctrl::W](W) writer structure"]
impl crate::Writable for MTL_RXQX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_RXQx_CTRL to value 0"]
impl crate::Resettable for MTL_RXQX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
