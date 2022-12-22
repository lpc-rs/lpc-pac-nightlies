#[doc = "Register `PINASSIGN7` reader"]
pub struct R(crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN7` writer"]
pub struct W(crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>;
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
impl From<crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTOUT_1_O` reader - CTOUT_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_1_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTOUT_1_O` writer - CTOUT_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_1_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTOUT_2_O` reader - CTOUT_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_2_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTOUT_2_O` writer - CTOUT_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_2_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTOUT_3_O` reader - CTOUT_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_3_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTOUT_3_O` writer - CTOUT_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CTOUT_3_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC, u8, u8, 8, O>;
#[doc = "Field `I2C_SDA_IO` reader - I2C_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type I2C_SDA_IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_SDA_IO` writer - I2C_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type I2C_SDA_IO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTOUT_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_1_o(&self) -> CTOUT_1_O_R {
        CTOUT_1_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CTOUT_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_2_o(&self) -> CTOUT_2_O_R {
        CTOUT_2_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CTOUT_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_3_o(&self) -> CTOUT_3_O_R {
        CTOUT_3_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - I2C_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn i2c_sda_io(&self) -> I2C_SDA_IO_R {
        I2C_SDA_IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTOUT_1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_1_o(&mut self) -> CTOUT_1_O_W<0> {
        CTOUT_1_O_W::new(self)
    }
    #[doc = "Bits 8:15 - CTOUT_2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_2_o(&mut self) -> CTOUT_2_O_W<8> {
        CTOUT_2_O_W::new(self)
    }
    #[doc = "Bits 16:23 - CTOUT_3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn ctout_3_o(&mut self) -> CTOUT_3_O_W<16> {
        CTOUT_3_O_W::new(self)
    }
    #[doc = "Bits 24:31 - I2C_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn i2c_sda_io(&mut self) -> I2C_SDA_IO_W<24> {
        I2C_SDA_IO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 7. Assign movable functions CTOUT_1, CTOUT_2, CTOUT_3,I2C_SDA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign_pinassign_data_pinassign7](index.html) module"]
pub struct PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC;
impl crate::RegisterSpec for PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign_pinassign_data_pinassign7::R](R) reader structure"]
impl crate::Readable for PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign_pinassign_data_pinassign7::W](W) writer structure"]
impl crate::Writable for PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN7 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN_PINASSIGN_DATA_PINASSIGN7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
