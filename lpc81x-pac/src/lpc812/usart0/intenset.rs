#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDYEN` reader - When 1, enables an interrupt when there is a received character available to be read from the RXDAT register."]
pub type RXRDYEN_R = crate::BitReader<bool>;
#[doc = "Field `RXRDYEN` writer - When 1, enables an interrupt when there is a received character available to be read from the RXDAT register."]
pub type RXRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `TXRDYEN` reader - When 1, enables an interrupt when the TXDAT register is available to take another character to transmit."]
pub type TXRDYEN_R = crate::BitReader<bool>;
#[doc = "Field `TXRDYEN` writer - When 1, enables an interrupt when the TXDAT register is available to take another character to transmit."]
pub type TXRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DELTACTSEN` reader - When 1, enables an interrupt when there is a change in the state of the CTS input."]
pub type DELTACTSEN_R = crate::BitReader<bool>;
#[doc = "Field `DELTACTSEN` writer - When 1, enables an interrupt when there is a change in the state of the CTS input."]
pub type DELTACTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `TXDISEN` reader - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
pub type TXDISEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDISEN` writer - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
pub type TXDISEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `OVERRUNEN` reader - When 1, enables an interrupt when an overrun error occurred."]
pub type OVERRUNEN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUNEN` writer - When 1, enables an interrupt when an overrun error occurred."]
pub type OVERRUNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DELTARXBRKEN` reader - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
pub type DELTARXBRKEN_R = crate::BitReader<bool>;
#[doc = "Field `DELTARXBRKEN` writer - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
pub type DELTARXBRKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `STARTEN` reader - When 1, enables an interrupt when a received start bit has been detected."]
pub type STARTEN_R = crate::BitReader<bool>;
#[doc = "Field `STARTEN` writer - When 1, enables an interrupt when a received start bit has been detected."]
pub type STARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `FRAMERREN` reader - When 1, enables an interrupt when a framing error has been detected."]
pub type FRAMERREN_R = crate::BitReader<bool>;
#[doc = "Field `FRAMERREN` writer - When 1, enables an interrupt when a framing error has been detected."]
pub type FRAMERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `PARITYERREN` reader - When 1, enables an interrupt when a parity error has been detected."]
pub type PARITYERREN_R = crate::BitReader<bool>;
#[doc = "Field `PARITYERREN` writer - When 1, enables an interrupt when a parity error has been detected."]
pub type PARITYERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `RXNOISEEN` reader - When 1, enables an interrupt when noise is detected."]
pub type RXNOISEEN_R = crate::BitReader<bool>;
#[doc = "Field `RXNOISEEN` writer - When 1, enables an interrupt when noise is detected."]
pub type RXNOISEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When 1, enables an interrupt when there is a received character available to be read from the RXDAT register."]
    #[inline(always)]
    pub fn rxrdyen(&self) -> RXRDYEN_R {
        RXRDYEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, enables an interrupt when the TXDAT register is available to take another character to transmit."]
    #[inline(always)]
    pub fn txrdyen(&self) -> TXRDYEN_R {
        TXRDYEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&self) -> DELTACTSEN_R {
        DELTACTSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&self) -> TXDISEN_R {
        TXDISEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, enables an interrupt when an overrun error occurred."]
    #[inline(always)]
    pub fn overrunen(&self) -> OVERRUNEN_R {
        OVERRUNEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&self) -> DELTARXBRKEN_R {
        DELTARXBRKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&self) -> STARTEN_R {
        STARTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&self) -> FRAMERREN_R {
        FRAMERREN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&self) -> PARITYERREN_R {
        PARITYERREN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected."]
    #[inline(always)]
    pub fn rxnoiseen(&self) -> RXNOISEEN_R {
        RXNOISEEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables an interrupt when there is a received character available to be read from the RXDAT register."]
    #[inline(always)]
    pub fn rxrdyen(&mut self) -> RXRDYEN_W<0> {
        RXRDYEN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, enables an interrupt when the TXDAT register is available to take another character to transmit."]
    #[inline(always)]
    pub fn txrdyen(&mut self) -> TXRDYEN_W<2> {
        TXRDYEN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&mut self) -> DELTACTSEN_W<5> {
        DELTACTSEN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&mut self) -> TXDISEN_W<6> {
        TXDISEN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, enables an interrupt when an overrun error occurred."]
    #[inline(always)]
    pub fn overrunen(&mut self) -> OVERRUNEN_W<8> {
        OVERRUNEN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&mut self) -> DELTARXBRKEN_W<11> {
        DELTARXBRKEN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&mut self) -> STARTEN_W<12> {
        STARTEN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&mut self) -> FRAMERREN_W<13> {
        FRAMERREN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&mut self) -> PARITYERREN_W<14> {
        PARITYERREN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected."]
    #[inline(always)]
    pub fn rxnoiseen(&mut self) -> RXNOISEEN_W<15> {
        RXNOISEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable read and Set register. Contains an individual interrupt enable bit for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
