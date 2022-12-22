#[doc = "Register `PIO014` reader"]
pub struct R(crate::R<PIO014_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO014_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO014_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO014_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO014` writer"]
pub struct W(crate::W<PIO014_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO014_SPEC>;
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
impl From<crate::W<PIO014_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO014_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Selects pin function."]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "Selects pin function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Alternative connection 0."]
    ALT0 = 0,
    #[doc = "1: Alternative connection 1."]
    ALT1 = 1,
    #[doc = "2: Alternative connection 2."]
    ALT2 = 2,
    #[doc = "3: Alternative connection 3."]
    ALT3 = 3,
    #[doc = "4: Alternative connection 4."]
    ALT4 = 4,
    #[doc = "5: Alternative connection 5."]
    ALT5 = 5,
    #[doc = "6: Alternative connection 6."]
    ALT6 = 6,
    #[doc = "7: Alternative connection 7."]
    ALT7 = 7,
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
            0 => Some(FUNC_A::ALT0),
            1 => Some(FUNC_A::ALT1),
            2 => Some(FUNC_A::ALT2),
            3 => Some(FUNC_A::ALT3),
            4 => Some(FUNC_A::ALT4),
            5 => Some(FUNC_A::ALT5),
            6 => Some(FUNC_A::ALT6),
            7 => Some(FUNC_A::ALT7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == FUNC_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == FUNC_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == FUNC_A::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == FUNC_A::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == FUNC_A::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == FUNC_A::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == FUNC_A::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == FUNC_A::ALT7
    }
}
#[doc = "Field `FUNC` writer - Selects pin function."]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO014_SPEC, u8, FUNC_A, 4, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "Alternative connection 0."]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(FUNC_A::ALT0)
    }
    #[doc = "Alternative connection 1."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(FUNC_A::ALT1)
    }
    #[doc = "Alternative connection 2."]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(FUNC_A::ALT2)
    }
    #[doc = "Alternative connection 3."]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(FUNC_A::ALT3)
    }
    #[doc = "Alternative connection 4."]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(FUNC_A::ALT4)
    }
    #[doc = "Alternative connection 5."]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(FUNC_A::ALT5)
    }
    #[doc = "Alternative connection 6."]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut W {
        self.variant(FUNC_A::ALT6)
    }
    #[doc = "Alternative connection 7."]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut W {
        self.variant(FUNC_A::ALT7)
    }
}
#[doc = "Field `I2CSLEW` reader - Controls slew rate of I2C pad."]
pub type I2CSLEW_R = crate::BitReader<I2CSLEW_A>;
#[doc = "Controls slew rate of I2C pad.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CSLEW_A {
    #[doc = "0: I2C mode."]
    I2C_MODE = 0,
    #[doc = "1: GPIO mode."]
    GPIO_MODE = 1,
}
impl From<I2CSLEW_A> for bool {
    #[inline(always)]
    fn from(variant: I2CSLEW_A) -> Self {
        variant as u8 != 0
    }
}
impl I2CSLEW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CSLEW_A {
        match self.bits {
            false => I2CSLEW_A::I2C_MODE,
            true => I2CSLEW_A::GPIO_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_MODE`"]
    #[inline(always)]
    pub fn is_i2c_mode(&self) -> bool {
        *self == I2CSLEW_A::I2C_MODE
    }
    #[doc = "Checks if the value of the field is `GPIO_MODE`"]
    #[inline(always)]
    pub fn is_gpio_mode(&self) -> bool {
        *self == I2CSLEW_A::GPIO_MODE
    }
}
#[doc = "Field `I2CSLEW` writer - Controls slew rate of I2C pad."]
pub type I2CSLEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO014_SPEC, I2CSLEW_A, O>;
impl<'a, const O: u8> I2CSLEW_W<'a, O> {
    #[doc = "I2C mode."]
    #[inline(always)]
    pub fn i2c_mode(self) -> &'a mut W {
        self.variant(I2CSLEW_A::I2C_MODE)
    }
    #[doc = "GPIO mode."]
    #[inline(always)]
    pub fn gpio_mode(self) -> &'a mut W {
        self.variant(I2CSLEW_A::GPIO_MODE)
    }
}
#[doc = "Field `INVERT` reader - Input polarity."]
pub type INVERT_R = crate::BitReader<INVERT_A>;
#[doc = "Input polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERT_A {
    #[doc = "0: Disabled. Input function is not inverted."]
    DISABLED = 0,
    #[doc = "1: Enabled. Input is function inverted."]
    ENABLED = 1,
}
impl From<INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: INVERT_A) -> Self {
        variant as u8 != 0
    }
}
impl INVERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVERT_A {
        match self.bits {
            false => INVERT_A::DISABLED,
            true => INVERT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INVERT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INVERT_A::ENABLED
    }
}
#[doc = "Field `INVERT` writer - Input polarity."]
pub type INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO014_SPEC, INVERT_A, O>;
impl<'a, const O: u8> INVERT_W<'a, O> {
    #[doc = "Disabled. Input function is not inverted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INVERT_A::DISABLED)
    }
    #[doc = "Enabled. Input is function inverted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INVERT_A::ENABLED)
    }
}
#[doc = "Field `DIGIMODE` reader - Select Analog/Digital mode."]
pub type DIGIMODE_R = crate::BitReader<DIGIMODE_A>;
#[doc = "Select Analog/Digital mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGIMODE_A {
    #[doc = "0: Analog mode."]
    ANALOG = 0,
    #[doc = "1: Digital mode."]
    DIGITAL = 1,
}
impl From<DIGIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DIGIMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DIGIMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGIMODE_A {
        match self.bits {
            false => DIGIMODE_A::ANALOG,
            true => DIGIMODE_A::DIGITAL,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == DIGIMODE_A::ANALOG
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == DIGIMODE_A::DIGITAL
    }
}
#[doc = "Field `DIGIMODE` writer - Select Analog/Digital mode."]
pub type DIGIMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO014_SPEC, DIGIMODE_A, O>;
impl<'a, const O: u8> DIGIMODE_W<'a, O> {
    #[doc = "Analog mode."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(DIGIMODE_A::ANALOG)
    }
    #[doc = "Digital mode."]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(DIGIMODE_A::DIGITAL)
    }
}
#[doc = "Field `FILTEROFF` reader - Controls input glitch filter."]
pub type FILTEROFF_R = crate::BitReader<FILTEROFF_A>;
#[doc = "Controls input glitch filter.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEROFF_A {
    #[doc = "0: Filter enabled. Noise pulses below approximately 10 ns are filtered out."]
    ENABLED = 0,
    #[doc = "1: Filter disabled. No input filtering is done."]
    DISABLED = 1,
}
impl From<FILTEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: FILTEROFF_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTEROFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTEROFF_A {
        match self.bits {
            false => FILTEROFF_A::ENABLED,
            true => FILTEROFF_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FILTEROFF_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FILTEROFF_A::DISABLED
    }
}
#[doc = "Field `FILTEROFF` writer - Controls input glitch filter."]
pub type FILTEROFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO014_SPEC, FILTEROFF_A, O>;
impl<'a, const O: u8> FILTEROFF_W<'a, O> {
    #[doc = "Filter enabled. Noise pulses below approximately 10 ns are filtered out."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTEROFF_A::ENABLED)
    }
    #[doc = "Filter disabled. No input filtering is done."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTEROFF_A::DISABLED)
    }
}
#[doc = "Field `I2CDRIVE` reader - Controls the current sink capability of the pin."]
pub type I2CDRIVE_R = crate::BitReader<I2CDRIVE_A>;
#[doc = "Controls the current sink capability of the pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CDRIVE_A {
    #[doc = "0: Low drive. Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    LOW = 0,
    #[doc = "1: High drive. Output drive sink is 20 mA. This is needed for Fast Mode Plus I 2C. Refer to the appropriate specific device data sheet for details."]
    HIGH = 1,
}
impl From<I2CDRIVE_A> for bool {
    #[inline(always)]
    fn from(variant: I2CDRIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl I2CDRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CDRIVE_A {
        match self.bits {
            false => I2CDRIVE_A::LOW,
            true => I2CDRIVE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == I2CDRIVE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == I2CDRIVE_A::HIGH
    }
}
#[doc = "Field `I2CDRIVE` writer - Controls the current sink capability of the pin."]
pub type I2CDRIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO014_SPEC, I2CDRIVE_A, O>;
impl<'a, const O: u8> I2CDRIVE_W<'a, O> {
    #[doc = "Low drive. Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(I2CDRIVE_A::LOW)
    }
    #[doc = "High drive. Output drive sink is 20 mA. This is needed for Fast Mode Plus I 2C. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(I2CDRIVE_A::HIGH)
    }
}
#[doc = "Field `I2CFILTER` reader - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
pub type I2CFILTER_R = crate::BitReader<I2CFILTER_A>;
#[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CFILTER_A {
    #[doc = "0: Enabled. I2C 50 ns glitch filter enabled."]
    ENABLED = 0,
    #[doc = "1: Disabled. I2C 50 ns glitch filter disabled."]
    DISABLED = 1,
}
impl From<I2CFILTER_A> for bool {
    #[inline(always)]
    fn from(variant: I2CFILTER_A) -> Self {
        variant as u8 != 0
    }
}
impl I2CFILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CFILTER_A {
        match self.bits {
            false => I2CFILTER_A::ENABLED,
            true => I2CFILTER_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2CFILTER_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2CFILTER_A::DISABLED
    }
}
#[doc = "Field `I2CFILTER` writer - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
pub type I2CFILTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO014_SPEC, I2CFILTER_A, O>;
impl<'a, const O: u8> I2CFILTER_W<'a, O> {
    #[doc = "Enabled. I2C 50 ns glitch filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2CFILTER_A::ENABLED)
    }
    #[doc = "Disabled. I2C 50 ns glitch filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2CFILTER_A::DISABLED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Controls slew rate of I2C pad."]
    #[inline(always)]
    pub fn i2cslew(&self) -> I2CSLEW_R {
        I2CSLEW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Analog/Digital mode."]
    #[inline(always)]
    pub fn digimode(&self) -> DIGIMODE_R {
        DIGIMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&self) -> FILTEROFF_R {
        FILTEROFF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls the current sink capability of the pin."]
    #[inline(always)]
    pub fn i2cdrive(&self) -> I2CDRIVE_R {
        I2CDRIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn i2cfilter(&self) -> I2CFILTER_R {
        I2CFILTER_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bit 6 - Controls slew rate of I2C pad."]
    #[inline(always)]
    pub fn i2cslew(&mut self) -> I2CSLEW_W<6> {
        I2CSLEW_W::new(self)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&mut self) -> INVERT_W<7> {
        INVERT_W::new(self)
    }
    #[doc = "Bit 8 - Select Analog/Digital mode."]
    #[inline(always)]
    pub fn digimode(&mut self) -> DIGIMODE_W<8> {
        DIGIMODE_W::new(self)
    }
    #[doc = "Bit 9 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&mut self) -> FILTEROFF_W<9> {
        FILTEROFF_W::new(self)
    }
    #[doc = "Bit 10 - Controls the current sink capability of the pin."]
    #[inline(always)]
    pub fn i2cdrive(&mut self) -> I2CDRIVE_W<10> {
        I2CDRIVE_W::new(self)
    }
    #[doc = "Bit 11 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn i2cfilter(&mut self) -> I2CFILTER_W<11> {
        I2CFILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio014](index.html) module"]
pub struct PIO014_SPEC;
impl crate::RegisterSpec for PIO014_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio014::R](R) reader structure"]
impl crate::Readable for PIO014_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio014::W](W) writer structure"]
impl crate::Writable for PIO014_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO014 to value 0x0340"]
impl crate::Resettable for PIO014_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0340
    }
}
