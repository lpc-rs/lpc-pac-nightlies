#[doc = "Register `MTL_TXQx_INTCTRL_STAT` reader"]
pub struct R(crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_INTCTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_INTCTRL_STAT` writer"]
pub struct W(crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>;
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
impl From<crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_INTCTRL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXUNFIS` reader - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
pub type TXUNFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXUNFIS` writer - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
pub type TXUNFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MTL_TXQX_INTCTRL_STAT_SPEC, bool, O>;
#[doc = "Field `ABPSIS` reader - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
pub type ABPSIS_R = crate::BitReader<bool>;
#[doc = "Field `ABPSIS` writer - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
pub type ABPSIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_INTCTRL_STAT_SPEC, bool, O>;
#[doc = "Field `TXUIE` reader - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
pub type TXUIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUIE` writer - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
pub type TXUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_INTCTRL_STAT_SPEC, bool, O>;
#[doc = "Field `ABPSIE` reader - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
pub type ABPSIE_R = crate::BitReader<bool>;
#[doc = "Field `ABPSIE` writer - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
pub type ABPSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_INTCTRL_STAT_SPEC, bool, O>;
#[doc = "Field `RXOVFIS` reader - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
pub type RXOVFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXOVFIS` writer - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
pub type RXOVFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MTL_TXQX_INTCTRL_STAT_SPEC, bool, O>;
#[doc = "Field `RXOIE` reader - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
pub type RXOIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOIE` writer - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
pub type RXOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTL_TXQX_INTCTRL_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    #[inline(always)]
    pub fn txunfis(&mut self) -> TXUNFIS_W<0> {
        TXUNFIS_W::new(self)
    }
    #[doc = "Bit 1 - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    #[inline(always)]
    pub fn abpsis(&mut self) -> ABPSIS_W<1> {
        ABPSIS_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    #[inline(always)]
    pub fn txuie(&mut self) -> TXUIE_W<8> {
        TXUIE_W::new(self)
    }
    #[doc = "Bit 9 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
    #[inline(always)]
    pub fn abpsie(&mut self) -> ABPSIE_W<9> {
        ABPSIE_W::new(self)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<16> {
        RXOVFIS_W::new(self)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W<24> {
        RXOIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_intctrl_stat](index.html) module"]
pub struct MTL_TXQX_INTCTRL_STAT_SPEC;
impl crate::RegisterSpec for MTL_TXQX_INTCTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_intctrl_stat::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_INTCTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_intctrl_stat::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_INTCTRL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_INTCTRL_STAT to value 0"]
impl crate::Resettable for MTL_TXQX_INTCTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
