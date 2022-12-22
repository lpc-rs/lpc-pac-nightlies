#[doc = "Register `DEVINTCTRL` writer"]
pub struct W(crate::W<DEVINTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVINTCTRL_SPEC>;
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
impl From<crate::W<DEVINTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVINTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_CLR` writer - Frame interrupt . For isochronous packet transfers. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type FRAME_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP0_CLR` writer - USB core interrupt for physical endpoint 0. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP0_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP1_CLR` writer - USB core interrupt for physical endpoint 1. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP1_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP2_CLR` writer - USB core interrupt for physical endpoint 2. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP2_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP3_CLR` writer - USB core interrupt for physical endpoint 3. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP3_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP4_CLR` writer - USB core interrupt for physical endpoint 4. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP4_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP5_CLR` writer - USB core interrupt for physical endpoint 5. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP5_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP6_CLR` writer - USB core interrupt for physical endpoint 6. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP6_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `EP7_CLR` writer - USB core interrupt for physical endpoint 7. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type EP7_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `DEV_STAT_CLR` writer - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type DEV_STAT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `CC_EMPTY_CLR` writer - The command code register (USBCmdCode) is empty (New command can be written). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type CC_EMPTY_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `CD_FULL_CLR` writer - Command data register (USBCmdData) is full (Data can be read now). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type CD_FULL_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `RXENDPKT_CLR` writer - The current packet in the endpoint buffer is transferred to the CPU. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type RXENDPKT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
#[doc = "Field `TXENDPKT_CLR` writer - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
pub type TXENDPKT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTCTRL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Frame interrupt . For isochronous packet transfers. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn frame_clr(&mut self) -> FRAME_CLR_W<0> {
        FRAME_CLR_W::new(self)
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep0_clr(&mut self) -> EP0_CLR_W<1> {
        EP0_CLR_W::new(self)
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep1_clr(&mut self) -> EP1_CLR_W<2> {
        EP1_CLR_W::new(self)
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep2_clr(&mut self) -> EP2_CLR_W<3> {
        EP2_CLR_W::new(self)
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep3_clr(&mut self) -> EP3_CLR_W<4> {
        EP3_CLR_W::new(self)
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep4_clr(&mut self) -> EP4_CLR_W<5> {
        EP4_CLR_W::new(self)
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep5_clr(&mut self) -> EP5_CLR_W<6> {
        EP5_CLR_W::new(self)
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep6_clr(&mut self) -> EP6_CLR_W<7> {
        EP6_CLR_W::new(self)
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn ep7_clr(&mut self) -> EP7_CLR_W<8> {
        EP7_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn dev_stat_clr(&mut self) -> DEV_STAT_CLR_W<9> {
        DEV_STAT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn cc_empty_clr(&mut self) -> CC_EMPTY_CLR_W<10> {
        CC_EMPTY_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn cd_full_clr(&mut self) -> CD_FULL_CLR_W<11> {
        CD_FULL_CLR_W::new(self)
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn rxendpkt_clr(&mut self) -> RXENDPKT_CLR_W<12> {
        RXENDPKT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no effect. 1 = the corresponding bit in USBDevIntSt is cleared."]
    #[inline(always)]
    pub fn txendpkt_clr(&mut self) -> TXENDPKT_CLR_W<13> {
        TXENDPKT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintctrl](index.html) module"]
pub struct DEVINTCTRL_SPEC;
impl crate::RegisterSpec for DEVINTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [devintctrl::W](W) writer structure"]
impl crate::Writable for DEVINTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVINTCTRL to value 0"]
impl crate::Resettable for DEVINTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
