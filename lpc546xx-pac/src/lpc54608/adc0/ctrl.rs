#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ASYNMODE` reader - Select clock mode."]
pub type ASYNMODE_R = crate::BitReader<ASYNMODE_A>;
#[doc = "Select clock mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNMODE_A {
    #[doc = "0: Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    SYNCHRONOUS_MODE = 0,
    #[doc = "1: Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    ASYNCHRONOUS_MODE = 1,
}
impl From<ASYNMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ASYNMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ASYNMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASYNMODE_A {
        match self.bits {
            false => ASYNMODE_A::SYNCHRONOUS_MODE,
            true => ASYNMODE_A::ASYNCHRONOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_synchronous_mode(&self) -> bool {
        *self == ASYNMODE_A::SYNCHRONOUS_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == ASYNMODE_A::ASYNCHRONOUS_MODE
    }
}
#[doc = "Field `ASYNMODE` writer - Select clock mode."]
pub type ASYNMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ASYNMODE_A, O>;
impl<'a, const O: u8> ASYNMODE_W<'a, O> {
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    #[inline(always)]
    pub fn synchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODE_A::SYNCHRONOUS_MODE)
    }
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODE_A::ASYNCHRONOUS_MODE)
    }
}
#[doc = "Field `RESOL` reader - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
pub type RESOL_R = crate::FieldReader<u8, RESOL_A>;
#[doc = "The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESOL_A {
    #[doc = "0: 6-bit resolution. An ADC conversion requires 9 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_6_BIT = 0,
    #[doc = "1: 8-bit resolution. An ADC conversion requires 11 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_8_BIT = 1,
    #[doc = "2: 10-bit resolution. An ADC conversion requires 13 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_10_BIT = 2,
    #[doc = "3: 12-bit resolution. An ADC conversion requires 15 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_12_BIT = 3,
}
impl From<RESOL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESOL_A) -> Self {
        variant as _
    }
}
impl RESOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESOL_A {
        match self.bits {
            0 => RESOL_A::RESOLUTION_6_BIT,
            1 => RESOL_A::RESOLUTION_8_BIT,
            2 => RESOL_A::RESOLUTION_10_BIT,
            3 => RESOL_A::RESOLUTION_12_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_6_BIT`"]
    #[inline(always)]
    pub fn is_resolution_6_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_6_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_8_BIT`"]
    #[inline(always)]
    pub fn is_resolution_8_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_8_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_10_BIT`"]
    #[inline(always)]
    pub fn is_resolution_10_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_10_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_12_BIT`"]
    #[inline(always)]
    pub fn is_resolution_12_bit(&self) -> bool {
        *self == RESOL_A::RESOLUTION_12_BIT
    }
}
#[doc = "Field `RESOL` writer - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
pub type RESOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, RESOL_A, 2, O>;
impl<'a, const O: u8> RESOL_W<'a, O> {
    #[doc = "6-bit resolution. An ADC conversion requires 9 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_6_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_6_BIT)
    }
    #[doc = "8-bit resolution. An ADC conversion requires 11 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_8_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_8_BIT)
    }
    #[doc = "10-bit resolution. An ADC conversion requires 13 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_10_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_10_BIT)
    }
    #[doc = "12-bit resolution. An ADC conversion requires 15 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_12_bit(self) -> &'a mut W {
        self.variant(RESOL_A::RESOLUTION_12_BIT)
    }
}
#[doc = "Field `BYPASSCAL` reader - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
pub type BYPASSCAL_R = crate::BitReader<BYPASSCAL_A>;
#[doc = "Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSCAL_A {
    #[doc = "0: Calibrate. The stored calibration value will be applied to the ADC during conversions to compensated for offset error. A calibration cycle must be performed each time the chip is powered-up. Re-calibration may be warranted periodically - especially if operating conditions have changed."]
    CALIBRATE = 0,
    #[doc = "1: Bypass calibration. Calibration is not utilized. Less time is required when enabling the ADC - particularly following chip power-up. Attempts to launch a calibration cycle are blocked when this bit is set."]
    BYPASS_CALIBRATION = 1,
}
impl From<BYPASSCAL_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSCAL_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASSCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSCAL_A {
        match self.bits {
            false => BYPASSCAL_A::CALIBRATE,
            true => BYPASSCAL_A::BYPASS_CALIBRATION,
        }
    }
    #[doc = "Checks if the value of the field is `CALIBRATE`"]
    #[inline(always)]
    pub fn is_calibrate(&self) -> bool {
        *self == BYPASSCAL_A::CALIBRATE
    }
    #[doc = "Checks if the value of the field is `BYPASS_CALIBRATION`"]
    #[inline(always)]
    pub fn is_bypass_calibration(&self) -> bool {
        *self == BYPASSCAL_A::BYPASS_CALIBRATION
    }
}
#[doc = "Field `BYPASSCAL` writer - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
pub type BYPASSCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BYPASSCAL_A, O>;
impl<'a, const O: u8> BYPASSCAL_W<'a, O> {
    #[doc = "Calibrate. The stored calibration value will be applied to the ADC during conversions to compensated for offset error. A calibration cycle must be performed each time the chip is powered-up. Re-calibration may be warranted periodically - especially if operating conditions have changed."]
    #[inline(always)]
    pub fn calibrate(self) -> &'a mut W {
        self.variant(BYPASSCAL_A::CALIBRATE)
    }
    #[doc = "Bypass calibration. Calibration is not utilized. Less time is required when enabling the ADC - particularly following chip power-up. Attempts to launch a calibration cycle are blocked when this bit is set."]
    #[inline(always)]
    pub fn bypass_calibration(self) -> &'a mut W {
        self.variant(BYPASSCAL_A::BYPASS_CALIBRATION)
    }
}
#[doc = "Field `TSAMP` reader - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
pub type TSAMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSAMP` writer - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
pub type TSAMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline(always)]
    pub fn asynmode(&self) -> ASYNMODE_R {
        ASYNMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
    #[inline(always)]
    pub fn resol(&self) -> RESOL_R {
        RESOL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
    #[inline(always)]
    pub fn bypasscal(&self) -> BYPASSCAL_R {
        BYPASSCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
    #[inline(always)]
    pub fn tsamp(&self) -> TSAMP_R {
        TSAMP_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline(always)]
    pub fn asynmode(&mut self) -> ASYNMODE_W<8> {
        ASYNMODE_W::new(self)
    }
    #[doc = "Bits 9:10 - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
    #[inline(always)]
    pub fn resol(&mut self) -> RESOL_W<9> {
        RESOL_W::new(self)
    }
    #[doc = "Bit 11 - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
    #[inline(always)]
    pub fn bypasscal(&mut self) -> BYPASSCAL_W<11> {
        BYPASSCAL_W::new(self)
    }
    #[doc = "Bits 12:14 - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
    #[inline(always)]
    pub fn tsamp(&mut self) -> TSAMP_W<12> {
        TSAMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0600"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
