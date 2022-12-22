#[doc = "Register `PIO0_4` reader"]
pub struct R(crate::R<PIO0_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO0_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO0_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO0_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO0_4` writer"]
pub struct W(crate::W<PIO0_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO0_4_SPEC>;
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
impl From<crate::W<PIO0_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO0_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Selects pin function. All other values are reserved."]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "Selects pin function. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Selects function PIO0_4 (open-drain pin)."]
    PIO = 0,
    #[doc = "1: Selects I2C function SCL (open-drain pin)."]
    SELECTS_I2C_FUNCTION = 1,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
impl FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::PIO),
            1 => Some(FUNC_A::SELECTS_I2C_FUNCTION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PIO`"]
    #[inline(always)]
    pub fn is_pio(&self) -> bool {
        *self == FUNC_A::PIO
    }
    #[doc = "Checks if the value of the field is `SELECTS_I2C_FUNCTION`"]
    #[inline(always)]
    pub fn is_selects_i2c_function(&self) -> bool {
        *self == FUNC_A::SELECTS_I2C_FUNCTION
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. All other values are reserved."]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO0_4_SPEC, u8, FUNC_A, 3, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "Selects function PIO0_4 (open-drain pin)."]
    #[inline(always)]
    pub fn pio(self) -> &'a mut W {
        self.variant(FUNC_A::PIO)
    }
    #[doc = "Selects I2C function SCL (open-drain pin)."]
    #[inline(always)]
    pub fn selects_i2c_function(self) -> &'a mut W {
        self.variant(FUNC_A::SELECTS_I2C_FUNCTION)
    }
}
#[doc = "Field `I2CMODE` reader - Selects I2C mode. Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub type I2CMODE_R = crate::FieldReader<u8, I2CMODE_A>;
#[doc = "Selects I2C mode. Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2CMODE_A {
    #[doc = "0: Standard mode/ Fast-mode I2C"]
    STANDARDFAST_ = 0,
    #[doc = "1: Standard I/O functionality"]
    STANDARDIO = 1,
    #[doc = "2: Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C = 2,
}
impl From<I2CMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: I2CMODE_A) -> Self {
        variant as _
    }
}
impl I2CMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CMODE_A {
        match self.bits {
            0 => I2CMODE_A::STANDARDFAST_,
            1 => I2CMODE_A::STANDARDIO,
            2 => I2CMODE_A::FAST_MODE_PLUS_I2C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARDFAST_`"]
    #[inline(always)]
    pub fn is_standardfast_(&self) -> bool {
        *self == I2CMODE_A::STANDARDFAST_
    }
    #[doc = "Checks if the value of the field is `STANDARDIO`"]
    #[inline(always)]
    pub fn is_standardio(&self) -> bool {
        *self == I2CMODE_A::STANDARDIO
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_I2C`"]
    #[inline(always)]
    pub fn is_fast_mode_plus_i2c(&self) -> bool {
        *self == I2CMODE_A::FAST_MODE_PLUS_I2C
    }
}
#[doc = "Field `I2CMODE` writer - Selects I2C mode. Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
pub type I2CMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO0_4_SPEC, u8, I2CMODE_A, 2, O>;
impl<'a, const O: u8> I2CMODE_W<'a, O> {
    #[doc = "Standard mode/ Fast-mode I2C"]
    #[inline(always)]
    pub fn standardfast_(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARDFAST_)
    }
    #[doc = "Standard I/O functionality"]
    #[inline(always)]
    pub fn standardio(self) -> &'a mut W {
        self.variant(I2CMODE_A::STANDARDIO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline(always)]
    pub fn fast_mode_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODE_A::FAST_MODE_PLUS_I2C)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&self) -> I2CMODE_R {
        I2CMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. All other values are reserved."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects I2C mode. Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&mut self) -> I2CMODE_W<8> {
        I2CMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration for pin PIO0_4/SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_4](index.html) module"]
pub struct PIO0_4_SPEC;
impl crate::RegisterSpec for PIO0_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio0_4::R](R) reader structure"]
impl crate::Readable for PIO0_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio0_4::W](W) writer structure"]
impl crate::Writable for PIO0_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO0_4 to value 0"]
impl crate::Resettable for PIO0_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
