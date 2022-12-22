#[doc = "Register `MTL_RXQx_OP_MODE` reader"]
pub struct R(crate::R<MTL_RXQX_OP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQX_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQX_OP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQX_OP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_RXQx_OP_MODE` writer"]
pub struct W(crate::W<MTL_RXQX_OP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQX_OP_MODE_SPEC>;
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
impl From<crate::W<MTL_RXQX_OP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQX_OP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
pub type RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC` writer - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTL_RXQX_OP_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `FUP` reader - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
pub type FUP_R = crate::BitReader<bool>;
#[doc = "Field `FUP` writer - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
pub type FUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQX_OP_MODE_SPEC, bool, O>;
#[doc = "Field `FEP` reader - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
pub type FEP_R = crate::BitReader<bool>;
#[doc = "Field `FEP` writer - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
pub type FEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQX_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RSF` reader - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQX_OP_MODE_SPEC, bool, O>;
#[doc = "Field `DIS_TCP_EF` reader - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
pub type DIS_TCP_EF_R = crate::BitReader<bool>;
#[doc = "Field `DIS_TCP_EF` writer - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
pub type DIS_TCP_EF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_RXQX_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RQS` reader - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
pub type RQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RQS` writer - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
pub type RQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTL_RXQX_OP_MODE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<0> {
        RTC_W::new(self)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W<3> {
        FUP_W::new(self)
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&mut self) -> FEP_W<4> {
        FEP_W::new(self)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<6> {
        DIS_TCP_EF_W::new(self)
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&mut self) -> RQS_W<20> {
        RQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL RxQx Operation Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_op_mode](index.html) module"]
pub struct MTL_RXQX_OP_MODE_SPEC;
impl crate::RegisterSpec for MTL_RXQX_OP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_rxqx_op_mode::R](R) reader structure"]
impl crate::Readable for MTL_RXQX_OP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_op_mode::W](W) writer structure"]
impl crate::Writable for MTL_RXQX_OP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_RXQx_OP_MODE to value 0"]
impl crate::Resettable for MTL_RXQX_OP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
