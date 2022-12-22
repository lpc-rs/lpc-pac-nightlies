#[doc = "Register `SYSOSCCTRL` reader"]
pub struct R(crate::R<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSOSCCTRL` writer"]
pub struct W(crate::W<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSOSCCTRL_SPEC>;
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
impl From<crate::W<SYSOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Bypass system oscillator"]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
#[doc = "Bypass system oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Oscillator is not bypassed."]
    OSCILLATOR_IS_NOT_BY = 0,
    #[doc = "1: Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins."]
    BYPASS_ENABLED_PLL_ = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::OSCILLATOR_IS_NOT_BY,
            true => BYPASS_A::BYPASS_ENABLED_PLL_,
        }
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_IS_NOT_BY`"]
    #[inline(always)]
    pub fn is_oscillator_is_not_by(&self) -> bool {
        *self == BYPASS_A::OSCILLATOR_IS_NOT_BY
    }
    #[doc = "Checks if the value of the field is `BYPASS_ENABLED_PLL_`"]
    #[inline(always)]
    pub fn is_bypass_enabled_pll_(&self) -> bool {
        *self == BYPASS_A::BYPASS_ENABLED_PLL_
    }
}
#[doc = "Field `BYPASS` writer - Bypass system oscillator"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSOSCCTRL_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "Oscillator is not bypassed."]
    #[inline(always)]
    pub fn oscillator_is_not_by(self) -> &'a mut W {
        self.variant(BYPASS_A::OSCILLATOR_IS_NOT_BY)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN and XTALOUT pins."]
    #[inline(always)]
    pub fn bypass_enabled_pll_(self) -> &'a mut W {
        self.variant(BYPASS_A::BYPASS_ENABLED_PLL_)
    }
}
#[doc = "Field `FREQRANGE` reader - Determines frequency range for Low-power oscillator."]
pub type FREQRANGE_R = crate::BitReader<FREQRANGE_A>;
#[doc = "Determines frequency range for Low-power oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGE_A {
    #[doc = "0: 1 - 20 MHz frequency range."]
    _1__20_MHZ_FREQUENCY = 0,
    #[doc = "1: 15 - 25 MHz frequency range"]
    _15__25_MHZ_FREQUENC = 1,
}
impl From<FREQRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQRANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQRANGE_A {
        match self.bits {
            false => FREQRANGE_A::_1__20_MHZ_FREQUENCY,
            true => FREQRANGE_A::_15__25_MHZ_FREQUENC,
        }
    }
    #[doc = "Checks if the value of the field is `_1__20_MHZ_FREQUENCY`"]
    #[inline(always)]
    pub fn is_1__20_mhz_frequency(&self) -> bool {
        *self == FREQRANGE_A::_1__20_MHZ_FREQUENCY
    }
    #[doc = "Checks if the value of the field is `_15__25_MHZ_FREQUENC`"]
    #[inline(always)]
    pub fn is_15__25_mhz_frequenc(&self) -> bool {
        *self == FREQRANGE_A::_15__25_MHZ_FREQUENC
    }
}
#[doc = "Field `FREQRANGE` writer - Determines frequency range for Low-power oscillator."]
pub type FREQRANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSOSCCTRL_SPEC, FREQRANGE_A, O>;
impl<'a, const O: u8> FREQRANGE_W<'a, O> {
    #[doc = "1 - 20 MHz frequency range."]
    #[inline(always)]
    pub fn _1__20_mhz_frequency(self) -> &'a mut W {
        self.variant(FREQRANGE_A::_1__20_MHZ_FREQUENCY)
    }
    #[doc = "15 - 25 MHz frequency range"]
    #[inline(always)]
    pub fn _15__25_mhz_frequenc(self) -> &'a mut W {
        self.variant(FREQRANGE_A::_15__25_MHZ_FREQUENC)
    }
}
impl R {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FREQRANGE_W<1> {
        FREQRANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscctrl](index.html) module"]
pub struct SYSOSCCTRL_SPEC;
impl crate::RegisterSpec for SYSOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysoscctrl::R](R) reader structure"]
impl crate::Readable for SYSOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysoscctrl::W](W) writer structure"]
impl crate::Writable for SYSOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSOSCCTRL to value 0"]
impl crate::Resettable for SYSOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
