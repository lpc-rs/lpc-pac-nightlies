#[doc = "Register `PINASSIGN8` reader"]
pub struct R(crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN8` writer"]
pub struct W(crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>;
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
impl From<crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SCL_IO` reader - I2C_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type I2C_SCL_IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_SCL_IO` writer - I2C_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type I2C_SCL_IO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC, u8, u8, 8, O>;
#[doc = "Field `ACMP_O_O` reader - ACMP_O_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type ACMP_O_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACMP_O_O` writer - ACMP_O_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type ACMP_O_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKOUT_O` reader - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CLKOUT_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKOUT_O` writer - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type CLKOUT_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC, u8, u8, 8, O>;
#[doc = "Field `GPIO_INT_BMAT_O` reader - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type GPIO_INT_BMAT_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_INT_BMAT_O` writer - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
pub type GPIO_INT_BMAT_O_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - I2C_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn i2c_scl_io(&self) -> I2C_SCL_IO_R {
        I2C_SCL_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ACMP_O_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn acmp_o_o(&self) -> ACMP_O_O_R {
        ACMP_O_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn clkout_o(&self) -> CLKOUT_O_R {
        CLKOUT_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&self) -> GPIO_INT_BMAT_O_R {
        GPIO_INT_BMAT_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn i2c_scl_io(&mut self) -> I2C_SCL_IO_W<0> {
        I2C_SCL_IO_W::new(self)
    }
    #[doc = "Bits 8:15 - ACMP_O_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn acmp_o_o(&mut self) -> ACMP_O_O_W<8> {
        ACMP_O_O_W::new(self)
    }
    #[doc = "Bits 16:23 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn clkout_o(&mut self) -> CLKOUT_O_W<16> {
        CLKOUT_O_W::new(self)
    }
    #[doc = "Bits 24:31 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_17 (= 0x11)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&mut self) -> GPIO_INT_BMAT_O_W<24> {
        GPIO_INT_BMAT_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 8. Assign movable functions I2C_SCL, ACMP_O, CLKOUT,GPIO_INT_BMAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign_pinassign_data_pinassign8](index.html) module"]
pub struct PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC;
impl crate::RegisterSpec for PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign_pinassign_data_pinassign8::R](R) reader structure"]
impl crate::Readable for PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign_pinassign_data_pinassign8::W](W) writer structure"]
impl crate::Writable for PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN8 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN_PINASSIGN_DATA_PINASSIGN8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
