#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEN_AW {
    #[doc = "0: UART FIFOs are disabled. Must not be used in the application."]
    DISABLED = 0,
    #[doc = "1: Active high enable for both UART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the UART FIFOs."]
    ENABLED = 1,
}
impl From<FIFOEN_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, FIFOEN_AW, O>;
impl<'a, const O: u8> FIFOEN_W<'a, O> {
    #[doc = "UART FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::DISABLED)
    }
    #[doc = "Active high enable for both UART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the UART FIFOs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::ENABLED)
    }
}
#[doc = "RX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOR_AW {
    #[doc = "0: No impact on either of UART FIFOs."]
    NOACTION = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[1\\]
will clear all bytes in UART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<RXFIFOR_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOR` writer - RX FIFO Reset"]
pub type RXFIFOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, RXFIFOR_AW, O>;
impl<'a, const O: u8> RXFIFOR_W<'a, O> {
    #[doc = "No impact on either of UART FIFOs."]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut W {
        self.variant(RXFIFOR_AW::NOACTION)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\]
will clear all bytes in UART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFOR_AW::CLEAR)
    }
}
#[doc = "TX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOR_AW {
    #[doc = "0: No impact on either of UART FIFOs."]
    NOACTION = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[2\\]
will clear all bytes in UART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<TXFIFOR_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOR` writer - TX FIFO Reset"]
pub type TXFIFOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, TXFIFOR_AW, O>;
impl<'a, const O: u8> TXFIFOR_W<'a, O> {
    #[doc = "No impact on either of UART FIFOs."]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut W {
        self.variant(TXFIFOR_AW::NOACTION)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\]
will clear all bytes in UART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFOR_AW::CLEAR)
    }
}
#[doc = "RX Trigger Level. These two bits determine how many receiver UART FIFO characters must be written before an interrupt is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXTLVL_AW {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    TRIGGER_LEVEL_0_1_C = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    TRIGGER_LEVEL_1_4_C = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    TRIGGER_LEVEL_2_8_C = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    TRIGGER_LEVEL_3_14_ = 3,
}
impl From<RXTLVL_AW> for u8 {
    #[inline(always)]
    fn from(variant: RXTLVL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RXTLVL` writer - RX Trigger Level. These two bits determine how many receiver UART FIFO characters must be written before an interrupt is activated."]
pub type RXTLVL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCR_SPEC, u8, RXTLVL_AW, 2, O>;
impl<'a, const O: u8> RXTLVL_W<'a, O> {
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn trigger_level_0_1_c(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_0_1_C)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn trigger_level_1_4_c(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_1_4_C)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn trigger_level_2_8_c(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_2_8_C)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn trigger_level_3_14_(self) -> &'a mut W {
        self.variant(RXTLVL_AW::TRIGGER_LEVEL_3_14_)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<0> {
        FIFOEN_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO Reset"]
    #[inline(always)]
    pub fn rxfifor(&mut self) -> RXFIFOR_W<1> {
        RXFIFOR_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Reset"]
    #[inline(always)]
    pub fn txfifor(&mut self) -> TXFIFOR_W<2> {
        TXFIFOR_W::new(self)
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART FIFO characters must be written before an interrupt is activated."]
    #[inline(always)]
    pub fn rxtlvl(&mut self) -> RXTLVL_W<6> {
        RXTLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register. Controls UART FIFO usage and modes.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
