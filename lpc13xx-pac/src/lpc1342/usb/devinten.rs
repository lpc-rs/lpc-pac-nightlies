#[doc = "Register `DEVINTEN` reader"]
pub struct R(crate::R<DEVINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVINTEN` writer"]
pub struct W(crate::W<DEVINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVINTEN_SPEC>;
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
impl From<crate::W<DEVINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_EN` reader - Frame interrupt . For isochronous packet transfers. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type FRAME_EN_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_EN` writer - Frame interrupt . For isochronous packet transfers. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type FRAME_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP0_EN` reader - USB core interrupt for physical endpoint 0. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP0_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP0_EN` writer - USB core interrupt for physical endpoint 0. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP1_EN` reader - USB core interrupt for physical endpoint 1. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP1_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP1_EN` writer - USB core interrupt for physical endpoint 1. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP2_EN` reader - USB core interrupt for physical endpoint 2. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP2_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP2_EN` writer - USB core interrupt for physical endpoint 2. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP3_EN` reader - USB core interrupt for physical endpoint 3. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP3_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP3_EN` writer - USB core interrupt for physical endpoint 3. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP4_EN` reader - USB core interrupt for physical endpoint 4. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP4_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP4_EN` writer - USB core interrupt for physical endpoint 4. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP5_EN` reader - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
pub type EP5_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP5_EN` writer - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
pub type EP5_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP6_EN` reader - USB core interrupt for physical endpoint 6. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP6_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP6_EN` writer - USB core interrupt for physical endpoint 6. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP6_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `EP7_EN` reader - USB core interrupt for physical endpoint 7. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP7_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP7_EN` writer - USB core interrupt for physical endpoint 7. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type EP7_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `DEV_STAT_EN` reader - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type DEV_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEV_STAT_EN` writer - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type DEV_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `CC_EMPTY_EN` reader - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type CC_EMPTY_EN_R = crate::BitReader<bool>;
#[doc = "Field `CC_EMPTY_EN` writer - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type CC_EMPTY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `CD_FULL_EN` reader - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type CD_FULL_EN_R = crate::BitReader<bool>;
#[doc = "Field `CD_FULL_EN` writer - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type CD_FULL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `RXENDPKT_EN` reader - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type RXENDPKT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RXENDPKT_EN` writer - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type RXENDPKT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
#[doc = "Field `TXENDPKT_EN` reader - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type TXENDPKT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TXENDPKT_EN` writer - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
pub type TXENDPKT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVINTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Frame interrupt . For isochronous packet transfers. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn frame_en(&self) -> FRAME_EN_R {
        FRAME_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep0_en(&self) -> EP0_EN_R {
        EP0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep1_en(&self) -> EP1_EN_R {
        EP1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep2_en(&self) -> EP2_EN_R {
        EP2_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep3_en(&self) -> EP3_EN_R {
        EP3_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep4_en(&self) -> EP4_EN_R {
        EP4_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep5_en(&self) -> EP5_EN_R {
        EP5_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep6_en(&self) -> EP6_EN_R {
        EP6_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep7_en(&self) -> EP7_EN_R {
        EP7_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn dev_stat_en(&self) -> DEV_STAT_EN_R {
        DEV_STAT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cc_empty_en(&self) -> CC_EMPTY_EN_R {
        CC_EMPTY_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cd_full_en(&self) -> CD_FULL_EN_R {
        CD_FULL_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn rxendpkt_en(&self) -> RXENDPKT_EN_R {
        RXENDPKT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn txendpkt_en(&self) -> TXENDPKT_EN_R {
        TXENDPKT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame interrupt . For isochronous packet transfers. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn frame_en(&mut self) -> FRAME_EN_W<0> {
        FRAME_EN_W::new(self)
    }
    #[doc = "Bit 1 - USB core interrupt for physical endpoint 0. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep0_en(&mut self) -> EP0_EN_W<1> {
        EP0_EN_W::new(self)
    }
    #[doc = "Bit 2 - USB core interrupt for physical endpoint 1. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep1_en(&mut self) -> EP1_EN_W<2> {
        EP1_EN_W::new(self)
    }
    #[doc = "Bit 3 - USB core interrupt for physical endpoint 2. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep2_en(&mut self) -> EP2_EN_W<3> {
        EP2_EN_W::new(self)
    }
    #[doc = "Bit 4 - USB core interrupt for physical endpoint 3. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep3_en(&mut self) -> EP3_EN_W<4> {
        EP3_EN_W::new(self)
    }
    #[doc = "Bit 5 - USB core interrupt for physical endpoint 4. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep4_en(&mut self) -> EP4_EN_W<5> {
        EP4_EN_W::new(self)
    }
    #[doc = "Bit 6 - USB core interrupt for physical endpoint 5. 0 = no interrupt. 1 = interrupt pending."]
    #[inline(always)]
    pub fn ep5_en(&mut self) -> EP5_EN_W<6> {
        EP5_EN_W::new(self)
    }
    #[doc = "Bit 7 - USB core interrupt for physical endpoint 6. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep6_en(&mut self) -> EP6_EN_W<7> {
        EP6_EN_W::new(self)
    }
    #[doc = "Bit 8 - USB core interrupt for physical endpoint 7. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn ep7_en(&mut self) -> EP7_EN_W<8> {
        EP7_EN_W::new(self)
    }
    #[doc = "Bit 9 - Set when USB Bus reset, USB suspend change, or Connect change event occurs. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn dev_stat_en(&mut self) -> DEV_STAT_EN_W<9> {
        DEV_STAT_EN_W::new(self)
    }
    #[doc = "Bit 10 - The command code register (USBCmdCode) is empty (New command can be written). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cc_empty_en(&mut self) -> CC_EMPTY_EN_W<10> {
        CC_EMPTY_EN_W::new(self)
    }
    #[doc = "Bit 11 - Command data register (USBCmdData) is full (Data can be read now). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn cd_full_en(&mut self) -> CD_FULL_EN_W<11> {
        CD_FULL_EN_W::new(self)
    }
    #[doc = "Bit 12 - The current packet in the endpoint buffer is transferred to the CPU. 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn rxendpkt_en(&mut self) -> RXENDPKT_EN_W<12> {
        RXENDPKT_EN_W::new(self)
    }
    #[doc = "Bit 13 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen). 0 = no interrupt generated. 1 = interrupt generated when the corresponding bit in USBDevIntSt is set."]
    #[inline(always)]
    pub fn txendpkt_en(&mut self) -> TXENDPKT_EN_W<13> {
        TXENDPKT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinten](index.html) module"]
pub struct DEVINTEN_SPEC;
impl crate::RegisterSpec for DEVINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devinten::R](R) reader structure"]
impl crate::Readable for DEVINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devinten::W](W) writer structure"]
impl crate::Writable for DEVINTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVINTEN to value 0"]
impl crate::Resettable for DEVINTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
