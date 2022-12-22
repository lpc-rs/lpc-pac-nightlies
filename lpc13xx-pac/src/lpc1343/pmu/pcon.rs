#[doc = "Register `PCON` reader"]
pub struct R(crate::R<PCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCON` writer"]
pub struct W(crate::W<PCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCON_SPEC>;
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
impl From<crate::W<PCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPDEN` reader - Deep power-down mode enable"]
pub type DPDEN_R = crate::BitReader<DPDEN_A>;
#[doc = "Deep power-down mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDEN_A {
    #[doc = "0: ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M3 core turned off)."]
    SLEEP_DEEPSLEEP = 0,
    #[doc = "1: ARM WFI will enter Deep-power down mode (ARM Cortex-M3 core powered-down)."]
    DEEPPOWERDOWN = 1,
}
impl From<DPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DPDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DPDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDEN_A {
        match self.bits {
            false => DPDEN_A::SLEEP_DEEPSLEEP,
            true => DPDEN_A::DEEPPOWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEP_DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_sleep_deepsleep(&self) -> bool {
        *self == DPDEN_A::SLEEP_DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == DPDEN_A::DEEPPOWERDOWN
    }
}
#[doc = "Field `DPDEN` writer - Deep power-down mode enable"]
pub type DPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, DPDEN_A, O>;
impl<'a, const O: u8> DPDEN_W<'a, O> {
    #[doc = "ARM WFI will enter Sleep or Deep-sleep mode (clock to ARM Cortex-M3 core turned off)."]
    #[inline(always)]
    pub fn sleep_deepsleep(self) -> &'a mut W {
        self.variant(DPDEN_A::SLEEP_DEEPSLEEP)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M3 core powered-down)."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(DPDEN_A::DEEPPOWERDOWN)
    }
}
#[doc = "Field `SLEEPFLAG` reader - Sleep mode flag"]
pub type SLEEPFLAG_R = crate::BitReader<SLEEPFLAG_A>;
#[doc = "Sleep mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAG_A {
    #[doc = "0: Read: No power-down mode entered. LPC13xx is in Run mode. Write: No effect."]
    NO_POWER_DOWN_ = 0,
    #[doc = "1: Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    POWERDOWN = 1,
}
impl From<SLEEPFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPFLAG_A {
        match self.bits {
            false => SLEEPFLAG_A::NO_POWER_DOWN_,
            true => SLEEPFLAG_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POWER_DOWN_`"]
    #[inline(always)]
    pub fn is_no_power_down_(&self) -> bool {
        *self == SLEEPFLAG_A::NO_POWER_DOWN_
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == SLEEPFLAG_A::POWERDOWN
    }
}
#[doc = "Field `SLEEPFLAG` writer - Sleep mode flag"]
pub type SLEEPFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, SLEEPFLAG_A, O>;
impl<'a, const O: u8> SLEEPFLAG_W<'a, O> {
    #[doc = "Read: No power-down mode entered. LPC13xx is in Run mode. Write: No effect."]
    #[inline(always)]
    pub fn no_power_down_(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::NO_POWER_DOWN_)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::POWERDOWN)
    }
}
#[doc = "Field `DPDFLAG` reader - Deep power-down flag"]
pub type DPDFLAG_R = crate::BitReader<DPDFLAG_A>;
#[doc = "Deep power-down flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAG_A {
    #[doc = "0: Read: Deep power-down mode not entered. Write: No effect."]
    NO_DEEPPOWERDOWN = 0,
    #[doc = "1: Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DEEPPOWERDOWN = 1,
}
impl From<DPDFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DPDFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl DPDFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDFLAG_A {
        match self.bits {
            false => DPDFLAG_A::NO_DEEPPOWERDOWN,
            true => DPDFLAG_A::DEEPPOWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_no_deeppowerdown(&self) -> bool {
        *self == DPDFLAG_A::NO_DEEPPOWERDOWN
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == DPDFLAG_A::DEEPPOWERDOWN
    }
}
#[doc = "Field `DPDFLAG` writer - Deep power-down flag"]
pub type DPDFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCON_SPEC, DPDFLAG_A, O>;
impl<'a, const O: u8> DPDFLAG_W<'a, O> {
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn no_deeppowerdown(self) -> &'a mut W {
        self.variant(DPDFLAG_A::NO_DEEPPOWERDOWN)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(DPDFLAG_A::DEEPPOWERDOWN)
    }
}
impl R {
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline(always)]
    pub fn dpden(&self) -> DPDEN_R {
        DPDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&self) -> SLEEPFLAG_R {
        SLEEPFLAG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Deep power-down mode enable"]
    #[inline(always)]
    pub fn dpden(&mut self) -> DPDEN_W<1> {
        DPDEN_W::new(self)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&mut self) -> SLEEPFLAG_W<8> {
        SLEEPFLAG_W::new(self)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&mut self) -> DPDFLAG_W<11> {
        DPDFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](index.html) module"]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcon::R](R) reader structure"]
impl crate::Readable for PCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcon::W](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
