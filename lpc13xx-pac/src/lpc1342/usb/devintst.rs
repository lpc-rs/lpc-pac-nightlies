#[doc = "Register `DEVINTST` reader"]
pub struct R(crate::R<DEVINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME` reader - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers. 0 = no interrupt. 1 = interrupt pending."]
pub type FRAME_R = crate::BitReader<bool>;
#[doc = "Field `EP0` reader - USB core interrupt for physical endpoint 0. 0 = no interrupt. 1 = interrupt pending."]
pub type EP0_R = crate::BitReader<bool>;
#[doc = "Field `EP1` reader - USB core interrupt for physical endpoint 1. 0 = no interrupt. 1 = interrupt pending."]
pub type EP1_R = crate::BitReader<bool>;
#[doc = "Field `EP2` reader - USB core interrupt for physical endpoint 2. 0 = no interrupt. 1 = interrupt pending."]
pub type EP2_R = crate::BitReader<bool>;
#[doc = "Field `EP3` reader - USB core interrupt for physical endpoint 3. 0 = no interrupt. 1 = interrupt pending."]
pub type EP3_R = crate::BitReader<bool>;
#[doc = "Field `EP4` reader - USB core interrupt for physical endpoint 4. 0 = no interrupt. 1 = interrupt pending."]
pub type EP4_R = crate::BitReader<bool>;
#[doc = "Field `EP5` reader - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
pub type EP5_R = crate::BitReader<bool>;
#[doc = "Field `EP6` reader - USB core interrupt for physical endpoint 6. 0 = no interrupt. 1 = interrupt pending."]
pub type EP6_R = crate::BitReader<bool>;
#[doc = "Field `EP7` reader - USB core interrupt for physical endpoint 7. 0 = no interrupt. 1 = interrupt pending."]
pub type EP7_R = crate::BitReader<bool>;
#[doc = "Field `DEV_STAT` reader - Set when USB Bus reset, USB suspend change, or Connect change event occurs. Refer to Section 10.11.7. 0 = no interrupt. 1 = interrupt pending."]
pub type DEV_STAT_R = crate::BitReader<bool>;
#[doc = "Field `CC_EMPTY` reader - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt. 1 = interrupt pending."]
pub type CC_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `CD_FULL` reader - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt. 1 = interrupt pending."]
pub type CD_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RxENDPKT` reader - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt. 1 = interrupt pending."]
pub type RX_ENDPKT_R = crate::BitReader<bool>;
#[doc = "Field `TxENDPKT` reader - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt. 1 = interrupt pending."]
pub type TX_ENDPKT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep0(&self) -> EP0_R {
        EP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep1(&self) -> EP1_R {
        EP1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep2(&self) -> EP2_R {
        EP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep3(&self) -> EP3_R {
        EP3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep4(&self) -> EP4_R {
        EP4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep5(&self) -> EP5_R {
        EP5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep6(&self) -> EP6_R {
        EP6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep7(&self) -> EP7_R {
        EP7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. Refer to Section 10.11.7. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn dev_stat(&self) -> DEV_STAT_R {
        DEV_STAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn cc_empty(&self) -> CC_EMPTY_R {
        CC_EMPTY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn cd_full(&self) -> CD_FULL_R {
        CD_FULL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn rx_endpkt(&self) -> RX_ENDPKT_R {
        RX_ENDPKT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn tx_endpkt(&self) -> TX_ENDPKT_R {
        TX_ENDPKT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "USB Device Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintst](index.html) module"]
pub struct DEVINTST_SPEC;
impl crate::RegisterSpec for DEVINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devintst::R](R) reader structure"]
impl crate::Readable for DEVINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVINTST to value 0x10"]
impl crate::Resettable for DEVINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
