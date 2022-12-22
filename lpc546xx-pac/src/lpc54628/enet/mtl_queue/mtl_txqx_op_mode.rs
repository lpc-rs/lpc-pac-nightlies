#[doc = "Register `MTL_TXQx_OP_MODE` reader"]
pub struct R(crate::R<MTL_TXQX_OP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_OP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_OP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_OP_MODE` writer"]
pub struct W(crate::W<MTL_TXQX_OP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_OP_MODE_SPEC>;
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
impl From<crate::W<MTL_TXQX_OP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_OP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTQ` reader - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
pub type FTQ_R = crate::BitReader<bool>;
#[doc = "Field `FTQ` writer - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
pub type FTQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_OP_MODE_SPEC, bool, O>;
#[doc = "Field `TSF` reader - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_OP_MODE_SPEC, bool, O>;
#[doc = "Field `TXQEN` reader - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
pub type TXQEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXQEN` writer - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
pub type TXQEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MTL_TXQX_OP_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `TTC` reader - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
pub type TTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` writer - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
pub type TTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTL_TXQX_OP_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `TQS` reader - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
pub type TQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TQS` writer - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
pub type TQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTL_TXQX_OP_MODE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W<0> {
        FTQ_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<1> {
        TSF_W::new(self)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&mut self) -> TXQEN_W<2> {
        TXQEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<4> {
        TTC_W::new(self)
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W<16> {
        TQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL TxQx Operation Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_op_mode](index.html) module"]
pub struct MTL_TXQX_OP_MODE_SPEC;
impl crate::RegisterSpec for MTL_TXQX_OP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_op_mode::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_OP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_op_mode::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_OP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_OP_MODE to value 0"]
impl crate::Resettable for MTL_TXQX_OP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
