#[doc = "Register `DEVINTSET` writer"]
pub struct W(crate::W<DEVINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVINTSET_SPEC>;
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
impl From<crate::W<DEVINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_SET` writer - Frame interrupt . For isochronous packet transfers. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type FRAME_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP0_SET` writer - USB core interrupt for physical endpoint 0. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP0_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP1_SET` writer - USB core interrupt for physical endpoint 1. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP1_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP2_SET` writer - USB core interrupt for physical endpoint 2. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP2_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP3_SET` writer - USB core interrupt for physical endpoint 3. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP3_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP4_SET` writer - USB core interrupt for physical endpoint 4. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP4_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP5_SET` writer - USB core interrupt for physical endpoint 5. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP5_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP6_SET` writer - USB core interrupt for physical endpoint 6. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP6_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `EP7_SET` writer - USB core interrupt for physical endpoint 7. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type EP7_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `DEV_STAT_SET` writer - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type DEV_STAT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `CC_EMPTY_SET` writer - The command code register (USBCmdCode) is empty (New command can be written). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type CC_EMPTY_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `CD_FULL_SET` writer - Command data register (USBCmdData) is full (Data can be read now). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type CD_FULL_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `RXENDPKT_SET` writer - The current packet in the endpoint buffer is transferred to the CPU. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type RXENDPKT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
#[doc = "Field `TXENDPKT_SET` writer - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
pub type TXENDPKT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTSET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Frame interrupt . For isochronous packet transfers. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn frame_set(&mut self) -> FRAME_SET_W<0> {
        FRAME_SET_W::new(self)
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep0_set(&mut self) -> EP0_SET_W<1> {
        EP0_SET_W::new(self)
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep1_set(&mut self) -> EP1_SET_W<2> {
        EP1_SET_W::new(self)
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep2_set(&mut self) -> EP2_SET_W<3> {
        EP2_SET_W::new(self)
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep3_set(&mut self) -> EP3_SET_W<4> {
        EP3_SET_W::new(self)
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep4_set(&mut self) -> EP4_SET_W<5> {
        EP4_SET_W::new(self)
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep5_set(&mut self) -> EP5_SET_W<6> {
        EP5_SET_W::new(self)
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep6_set(&mut self) -> EP6_SET_W<7> {
        EP6_SET_W::new(self)
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep7_set(&mut self) -> EP7_SET_W<8> {
        EP7_SET_W::new(self)
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn dev_stat_set(&mut self) -> DEV_STAT_SET_W<9> {
        DEV_STAT_SET_W::new(self)
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cc_empty_set(&mut self) -> CC_EMPTY_SET_W<10> {
        CC_EMPTY_SET_W::new(self)
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cd_full_set(&mut self) -> CD_FULL_SET_W<11> {
        CD_FULL_SET_W::new(self)
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn rxendpkt_set(&mut self) -> RXENDPKT_SET_W<12> {
        RXENDPKT_SET_W::new(self)
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn txendpkt_set(&mut self) -> TXENDPKT_SET_W<13> {
        TXENDPKT_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintset](index.html) module"]
pub struct DEVINTSET_SPEC;
impl crate::RegisterSpec for DEVINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [devintset::W](W) writer structure"]
impl crate::Writable for DEVINTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVINTSET to value 0"]
impl crate::Resettable for DEVINTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
