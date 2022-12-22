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
#[doc = "Field `CLKSEL` reader - Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set."]
pub type CLKSEL_R = crate::BitReader<CLKSEL_A>;
#[doc = "Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSEL_A {
    #[doc = "0: Divided IRC clock. This clock runs at 750 kHz and provides time-out periods of up to approximately 95 minutes in 1.33 us increments. Remark: This clock is not available in not available in Deep-sleep, power-down, deep power-down modes. Do not select this option if the timer is to be used to wake up from one of these modes."]
    DIVIDED_IRC_CLOCK = 0,
    #[doc = "1: This is the (nominally) 10 kHz clock and provides time-out periods of up to approximately 119 hours in 100 us increments. The accuracy of this clock is limited to +/- 40 % over temperature and processing. Remark: This clock is available in all power modes. Prior to use, the low-power oscillator must be enabled. The oscillator must also be set to remain active in Deep power-down if needed."]
    LOW_POWER_CLOCK = 1,
}
impl From<CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            false => CLKSEL_A::DIVIDED_IRC_CLOCK,
            true => CLKSEL_A::LOW_POWER_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDED_IRC_CLOCK`"]
    #[inline(always)]
    pub fn is_divided_irc_clock(&self) -> bool {
        *self == CLKSEL_A::DIVIDED_IRC_CLOCK
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_CLOCK`"]
    #[inline(always)]
    pub fn is_low_power_clock(&self) -> bool {
        *self == CLKSEL_A::LOW_POWER_CLOCK
    }
}
#[doc = "Field `CLKSEL` writer - Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set."]
pub type CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CLKSEL_A, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "Divided IRC clock. This clock runs at 750 kHz and provides time-out periods of up to approximately 95 minutes in 1.33 us increments. Remark: This clock is not available in not available in Deep-sleep, power-down, deep power-down modes. Do not select this option if the timer is to be used to wake up from one of these modes."]
    #[inline(always)]
    pub fn divided_irc_clock(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIVIDED_IRC_CLOCK)
    }
    #[doc = "This is the (nominally) 10 kHz clock and provides time-out periods of up to approximately 119 hours in 100 us increments. The accuracy of this clock is limited to +/- 40 % over temperature and processing. Remark: This clock is available in all power modes. Prior to use, the low-power oscillator must be enabled. The oscillator must also be set to remain active in Deep power-down if needed."]
    #[inline(always)]
    pub fn low_power_clock(self) -> &'a mut W {
        self.variant(CLKSEL_A::LOW_POWER_CLOCK)
    }
}
#[doc = "Field `ALARMFLAG` reader - Wake-up or alarm timer flag."]
pub type ALARMFLAG_R = crate::BitReader<ALARMFLAG_A>;
#[doc = "Wake-up or alarm timer flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMFLAG_A {
    #[doc = "0: No time-out. The self wake-up timer has not timed out. Writing a 0 to has no effect."]
    NO_TIME_OUT = 0,
    #[doc = "1: Time-out. The self wake-up timer has timed out. This flag generates an interrupt request which can wake up the part from any reduced power mode including Deep power-down if the clock source is the low power oscillator. Writing a 1 clears this status bit."]
    TIME_OUT = 1,
}
impl From<ALARMFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: ALARMFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARMFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARMFLAG_A {
        match self.bits {
            false => ALARMFLAG_A::NO_TIME_OUT,
            true => ALARMFLAG_A::TIME_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_OUT`"]
    #[inline(always)]
    pub fn is_no_time_out(&self) -> bool {
        *self == ALARMFLAG_A::NO_TIME_OUT
    }
    #[doc = "Checks if the value of the field is `TIME_OUT`"]
    #[inline(always)]
    pub fn is_time_out(&self) -> bool {
        *self == ALARMFLAG_A::TIME_OUT
    }
}
#[doc = "Field `ALARMFLAG` writer - Wake-up or alarm timer flag."]
pub type ALARMFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ALARMFLAG_A, O>;
impl<'a, const O: u8> ALARMFLAG_W<'a, O> {
    #[doc = "No time-out. The self wake-up timer has not timed out. Writing a 0 to has no effect."]
    #[inline(always)]
    pub fn no_time_out(self) -> &'a mut W {
        self.variant(ALARMFLAG_A::NO_TIME_OUT)
    }
    #[doc = "Time-out. The self wake-up timer has timed out. This flag generates an interrupt request which can wake up the part from any reduced power mode including Deep power-down if the clock source is the low power oscillator. Writing a 1 clears this status bit."]
    #[inline(always)]
    pub fn time_out(self) -> &'a mut W {
        self.variant(ALARMFLAG_A::TIME_OUT)
    }
}
#[doc = "Field `CLEARCTR` reader - Clears the self wake-up timer."]
pub type CLEARCTR_R = crate::BitReader<CLEARCTR_A>;
#[doc = "Clears the self wake-up timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEARCTR_A {
    #[doc = "0: No effect. Reading this bit always returns 0."]
    NO_EFFECT = 0,
    #[doc = "1: Clear the counter. Counting is halted until a new count value is loaded."]
    CLEAR_THE_COUNTER = 1,
}
impl From<CLEARCTR_A> for bool {
    #[inline(always)]
    fn from(variant: CLEARCTR_A) -> Self {
        variant as u8 != 0
    }
}
impl CLEARCTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLEARCTR_A {
        match self.bits {
            false => CLEARCTR_A::NO_EFFECT,
            true => CLEARCTR_A::CLEAR_THE_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLEARCTR_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_COUNTER`"]
    #[inline(always)]
    pub fn is_clear_the_counter(&self) -> bool {
        *self == CLEARCTR_A::CLEAR_THE_COUNTER
    }
}
#[doc = "Field `CLEARCTR` writer - Clears the self wake-up timer."]
pub type CLEARCTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CLEARCTR_A, O>;
impl<'a, const O: u8> CLEARCTR_W<'a, O> {
    #[doc = "No effect. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLEARCTR_A::NO_EFFECT)
    }
    #[doc = "Clear the counter. Counting is halted until a new count value is loaded."]
    #[inline(always)]
    pub fn clear_the_counter(self) -> &'a mut W {
        self.variant(CLEARCTR_A::CLEAR_THE_COUNTER)
    }
}
impl R {
    #[doc = "Bit 0 - Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up or alarm timer flag."]
    #[inline(always)]
    pub fn alarmflag(&self) -> ALARMFLAG_R {
        ALARMFLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clears the self wake-up timer."]
    #[inline(always)]
    pub fn clearctr(&self) -> CLEARCTR_R {
        CLEARCTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 1 - Wake-up or alarm timer flag."]
    #[inline(always)]
    pub fn alarmflag(&mut self) -> ALARMFLAG_W<1> {
        ALARMFLAG_W::new(self)
    }
    #[doc = "Bit 2 - Clears the self wake-up timer."]
    #[inline(always)]
    pub fn clearctr(&mut self) -> CLEARCTR_W<2> {
        CLEARCTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Self wake-up timer control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
