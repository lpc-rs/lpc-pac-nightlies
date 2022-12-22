#[doc = "Register `PINASSIGN5` reader"]
pub struct R(crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN5` writer"]
pub struct W(crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>;
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
impl From<crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_MOSI_IO` reader - SPI1_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type SPI1_MOSI_IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI1_MOSI_IO` writer - SPI1_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type SPI1_MOSI_IO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC, u8, u8, 8, O>;
#[doc = "Field `SPI1_MISO_IO` reader - SPI1_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type SPI1_MISO_IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI1_MISO_IO` writer - SPI1_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type SPI1_MISO_IO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC, u8, u8, 8, O>;
#[doc = "Field `SPI1_SSEL_IO` reader - SPI1_SSEL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type SPI1_SSEL_IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI1_SSEL_IO` writer - SPI1_SSEL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type SPI1_SSEL_IO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTIN_0_I` reader - CTIN_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_0_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTIN_0_I` writer - CTIN_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTIN_0_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SPI1_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn spi1_mosi_io(&self) -> SPI1_MOSI_IO_R {
        SPI1_MOSI_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPI1_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn spi1_miso_io(&self) -> SPI1_MISO_IO_R {
        SPI1_MISO_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI1_SSEL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn spi1_ssel_io(&self) -> SPI1_SSEL_IO_R {
        SPI1_SSEL_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CTIN_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_0_i(&self) -> CTIN_0_I_R {
        CTIN_0_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI1_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn spi1_mosi_io(&mut self) -> SPI1_MOSI_IO_W<0> {
        SPI1_MOSI_IO_W::new(self)
    }
    #[doc = "Bits 8:15 - SPI1_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn spi1_miso_io(&mut self) -> SPI1_MISO_IO_W<8> {
        SPI1_MISO_IO_W::new(self)
    }
    #[doc = "Bits 16:23 - SPI1_SSEL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn spi1_ssel_io(&mut self) -> SPI1_SSEL_IO_W<16> {
        SPI1_SSEL_IO_W::new(self)
    }
    #[doc = "Bits 24:31 - CTIN_0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctin_0_i(&mut self) -> CTIN_0_I_W<24> {
        CTIN_0_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 5. Assign movable functions SPI1_MOSI, SPI1_MISO,SPI1_SSEL, CTIN_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign_pinassign_data_pinassign5](index.html) module"]
pub struct PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC;
impl crate::RegisterSpec for PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign_pinassign_data_pinassign5::R](R) reader structure"]
impl crate::Readable for PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign_pinassign_data_pinassign5::W](W) writer structure"]
impl crate::Writable for PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN5 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN_PINASSIGN_DATA_PINASSIGN5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
