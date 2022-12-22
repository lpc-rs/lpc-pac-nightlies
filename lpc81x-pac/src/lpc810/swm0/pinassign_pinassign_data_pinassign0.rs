#[doc = "Register `PINASSIGN0` reader"]
pub struct R(crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN0` writer"]
pub struct W(crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>;
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
impl From<crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U0_TXD_O` reader - U0_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_TXD_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U0_TXD_O` writer - U0_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_TXD_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC, u8, u8, 8, O>;
#[doc = "Field `U0_RXD_I` reader - U0_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_RXD_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U0_RXD_I` writer - U0_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_RXD_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC, u8, u8, 8, O>;
#[doc = "Field `U0_RTS_O` reader - U0_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_RTS_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U0_RTS_O` writer - U0_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_RTS_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC, u8, u8, 8, O>;
#[doc = "Field `U0_CTS_I` reader - U0_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_CTS_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `U0_CTS_I` writer - U0_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type U0_CTS_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - U0_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_txd_o(&self) -> U0_TXD_O_R {
        U0_TXD_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U0_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_rxd_i(&self) -> U0_RXD_I_R {
        U0_RXD_I_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U0_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_rts_o(&self) -> U0_RTS_O_R {
        U0_RTS_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U0_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_cts_i(&self) -> U0_CTS_I_R {
        U0_CTS_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U0_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_txd_o(&mut self) -> U0_TXD_O_W<0> {
        U0_TXD_O_W::new(self)
    }
    #[doc = "Bits 8:15 - U0_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_rxd_i(&mut self) -> U0_RXD_I_W<8> {
        U0_RXD_I_W::new(self)
    }
    #[doc = "Bits 16:23 - U0_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_rts_o(&mut self) -> U0_RTS_O_W<16> {
        U0_RTS_O_W::new(self)
    }
    #[doc = "Bits 24:31 - U0_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn u0_cts_i(&mut self) -> U0_CTS_I_W<24> {
        U0_CTS_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign_pinassign_data_pinassign0](index.html) module"]
pub struct PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC;
impl crate::RegisterSpec for PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign_pinassign_data_pinassign0::R](R) reader structure"]
impl crate::Readable for PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign_pinassign_data_pinassign0::W](W) writer structure"]
impl crate::Writable for PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN0 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN_PINASSIGN_DATA_PINASSIGN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
