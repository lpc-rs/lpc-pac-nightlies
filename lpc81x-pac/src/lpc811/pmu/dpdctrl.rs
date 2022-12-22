#[doc = "Register `DPDCTRL` reader"]
pub struct R(crate::R<DPDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPDCTRL` writer"]
pub struct W(crate::W<DPDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPDCTRL_SPEC>;
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
impl From<crate::W<DPDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUPHYS` reader - WAKEUP pin hysteresis enable"]
pub type WAKEUPHYS_R = crate::BitReader<WAKEUPHYS_A>;
#[doc = "WAKEUP pin hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYS_A {
    #[doc = "0: Disabled. Hysteresis for WAKEUP pin disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Hysteresis for WAKEUP pin enabled."]
    ENABLED = 1,
}
impl From<WAKEUPHYS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPHYS_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUPHYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPHYS_A {
        match self.bits {
            false => WAKEUPHYS_A::DISABLED,
            true => WAKEUPHYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPHYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPHYS_A::ENABLED
    }
}
#[doc = "Field `WAKEUPHYS` writer - WAKEUP pin hysteresis enable"]
pub type WAKEUPHYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPDCTRL_SPEC, WAKEUPHYS_A, O>;
impl<'a, const O: u8> WAKEUPHYS_W<'a, O> {
    #[doc = "Disabled. Hysteresis for WAKEUP pin disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::DISABLED)
    }
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::ENABLED)
    }
}
#[doc = "Field `WAKEPAD_DISABLE` reader - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used."]
pub type WAKEPAD_DISABLE_R = crate::BitReader<WAKEPAD_DISABLE_A>;
#[doc = "WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEPAD_DISABLE_A {
    #[doc = "0: Enabled. The wake-up function is enabled on pin PIO0_4."]
    ENABLED = 0,
    #[doc = "1: Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    DISABLED = 1,
}
impl From<WAKEPAD_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEPAD_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEPAD_DISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEPAD_DISABLE_A {
        match self.bits {
            false => WAKEPAD_DISABLE_A::ENABLED,
            true => WAKEPAD_DISABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEPAD_DISABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEPAD_DISABLE_A::DISABLED
    }
}
#[doc = "Field `WAKEPAD_DISABLE` writer - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used."]
pub type WAKEPAD_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DPDCTRL_SPEC, WAKEPAD_DISABLE_A, O>;
impl<'a, const O: u8> WAKEPAD_DISABLE_W<'a, O> {
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLE_A::ENABLED)
    }
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLE_A::DISABLED)
    }
}
#[doc = "Field `LPOSCEN` reader - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
pub type LPOSCEN_R = crate::BitReader<LPOSCEN_A>;
#[doc = "Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<LPOSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPOSCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPOSCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOSCEN_A {
        match self.bits {
            false => LPOSCEN_A::DISABLED,
            true => LPOSCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPOSCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPOSCEN_A::ENABLED
    }
}
#[doc = "Field `LPOSCEN` writer - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
pub type LPOSCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPDCTRL_SPEC, LPOSCEN_A, O>;
impl<'a, const O: u8> LPOSCEN_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPOSCEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPOSCEN_A::ENABLED)
    }
}
#[doc = "Field `LPOSCDPDEN` reader - causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
pub type LPOSCDPDEN_R = crate::BitReader<LPOSCDPDEN_A>;
#[doc = "causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCDPDEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<LPOSCDPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPOSCDPDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPOSCDPDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOSCDPDEN_A {
        match self.bits {
            false => LPOSCDPDEN_A::DISABLED,
            true => LPOSCDPDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPOSCDPDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPOSCDPDEN_A::ENABLED
    }
}
#[doc = "Field `LPOSCDPDEN` writer - causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
pub type LPOSCDPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPDCTRL_SPEC, LPOSCDPDEN_A, O>;
impl<'a, const O: u8> LPOSCDPDEN_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPOSCDPDEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPOSCDPDEN_A::ENABLED)
    }
}
#[doc = "Field `GPDATA` reader - Data retained during Deep power-down mode."]
pub type GPDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPDATA` writer - Data retained during Deep power-down mode."]
pub type GPDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPDCTRL_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&self) -> WAKEUPHYS_R {
        WAKEUPHYS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline(always)]
    pub fn wakepad_disable(&self) -> WAKEPAD_DISABLE_R {
        WAKEPAD_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
    #[inline(always)]
    pub fn lposcen(&self) -> LPOSCEN_R {
        LPOSCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
    #[inline(always)]
    pub fn lposcdpden(&self) -> LPOSCDPDEN_R {
        LPOSCDPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&mut self) -> WAKEUPHYS_W<0> {
        WAKEUPHYS_W::new(self)
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline(always)]
    pub fn wakepad_disable(&mut self) -> WAKEPAD_DISABLE_W<1> {
        WAKEPAD_DISABLE_W::new(self)
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
    #[inline(always)]
    pub fn lposcen(&mut self) -> LPOSCEN_W<2> {
        LPOSCEN_W::new(self)
    }
    #[doc = "Bit 3 - causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
    #[inline(always)]
    pub fn lposcdpden(&mut self) -> LPOSCDPDEN_W<3> {
        LPOSCDPDEN_W::new(self)
    }
    #[doc = "Bits 4:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&mut self) -> GPDATA_W<4> {
        GPDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep power-down control register. Also includes bits for general purpose storage.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpdctrl](index.html) module"]
pub struct DPDCTRL_SPEC;
impl crate::RegisterSpec for DPDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpdctrl::R](R) reader structure"]
impl crate::Readable for DPDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpdctrl::W](W) writer structure"]
impl crate::Writable for DPDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPDCTRL to value 0"]
impl crate::Resettable for DPDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
