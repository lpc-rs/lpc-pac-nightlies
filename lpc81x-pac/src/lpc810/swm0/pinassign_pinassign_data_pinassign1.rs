#[doc = "Register `PINASSIGN1` reader"]
pub struct R(crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN1` writer"]
pub struct W(crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>;
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
impl From<crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U0_SCLK_IO` reader - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_SCLK_IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U0_SCLK_IO` writer - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_SCLK_IO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC, u8, u8, 8, O>;
#[doc = "Field `U1_TXD_O` reader - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U1_TXD_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U1_TXD_O` writer - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U1_TXD_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC, u8, u8, 8, O>;
#[doc = "Field `U1_RXD_I` reader - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U1_RXD_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U1_RXD_I` writer - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U1_RXD_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC, u8, u8, 8, O>;
#[doc = "Field `U1_RTS_O` reader - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U1_RTS_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U1_RTS_O` writer - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U1_RTS_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_sclk_io(&self) -> U0_SCLK_IO_R {
        U0_SCLK_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u1_txd_o(&self) -> U1_TXD_O_R {
        U1_TXD_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u1_rxd_i(&self) -> U1_RXD_I_R {
        U1_RXD_I_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u1_rts_o(&self) -> U1_RTS_O_R {
        U1_RTS_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_sclk_io(&mut self) -> U0_SCLK_IO_W<0> {
        U0_SCLK_IO_W::new(self)
    }
    #[doc = "Bits 8:15 - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u1_txd_o(&mut self) -> U1_TXD_O_W<8> {
        U1_TXD_O_W::new(self)
    }
    #[doc = "Bits 16:23 - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u1_rxd_i(&mut self) -> U1_RXD_I_W<16> {
        U1_RXD_I_W::new(self)
    }
    #[doc = "Bits 24:31 - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u1_rts_o(&mut self) -> U1_RTS_O_W<24> {
        U1_RTS_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign_pinassign_data_pinassign1](index.html) module"]
pub struct PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC;
impl crate::RegisterSpec for PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign_pinassign_data_pinassign1::R](R) reader structure"]
impl crate::Readable for PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign_pinassign_data_pinassign1::W](W) writer structure"]
impl crate::Writable for PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN1 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN_PINASSIGN_DATA_PINASSIGN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
