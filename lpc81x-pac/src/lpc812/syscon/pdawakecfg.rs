#[doc = "Register `PDAWAKECFG` reader"]
pub struct R(crate::R<PDAWAKECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDAWAKECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDAWAKECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDAWAKECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDAWAKECFG` writer"]
pub struct W(crate::W<PDAWAKECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDAWAKECFG_SPEC>;
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
impl From<crate::W<PDAWAKECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDAWAKECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRCOUT_PD` reader - IRC oscillator output wake-up configuration"]
pub type IRCOUT_PD_R = crate::BitReader<IRCOUT_PD_A>;
#[doc = "IRC oscillator output wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCOUT_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<IRCOUT_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IRCOUT_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl IRCOUT_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCOUT_PD_A {
        match self.bits {
            false => IRCOUT_PD_A::POWERED,
            true => IRCOUT_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRCOUT_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRCOUT_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `IRCOUT_PD` writer - IRC oscillator output wake-up configuration"]
pub type IRCOUT_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, IRCOUT_PD_A, O>;
impl<'a, const O: u8> IRCOUT_PD_W<'a, O> {
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRCOUT_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRCOUT_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `IRC_PD` reader - IRC oscillator power-down wake-up configuration"]
pub type IRC_PD_R = crate::BitReader<IRC_PD_A>;
#[doc = "IRC oscillator power-down wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<IRC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IRC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl IRC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC_PD_A {
        match self.bits {
            false => IRC_PD_A::POWERED,
            true => IRC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRC_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `IRC_PD` writer - IRC oscillator power-down wake-up configuration"]
pub type IRC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, IRC_PD_A, O>;
impl<'a, const O: u8> IRC_PD_W<'a, O> {
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRC_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRC_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `FLASH_PD` reader - Flash wake-up configuration"]
pub type FLASH_PD_R = crate::BitReader<FLASH_PD_A>;
#[doc = "Flash wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<FLASH_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PD_A {
        match self.bits {
            false => FLASH_PD_A::POWERED,
            true => FLASH_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == FLASH_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == FLASH_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `FLASH_PD` writer - Flash wake-up configuration"]
pub type FLASH_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, FLASH_PD_A, O>;
impl<'a, const O: u8> FLASH_PD_W<'a, O> {
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `BOD_PD` reader - BOD wake-up configuration"]
pub type BOD_PD_R = crate::BitReader<BOD_PD_A>;
#[doc = "BOD wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl BOD_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `BOD_PD` writer - BOD wake-up configuration"]
pub type BOD_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, BOD_PD_A, O>;
impl<'a, const O: u8> BOD_PD_W<'a, O> {
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `SYSOSC_PD` reader - Crystal oscillator wake-up configuration"]
pub type SYSOSC_PD_R = crate::BitReader<SYSOSC_PD_A>;
#[doc = "Crystal oscillator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSOSC_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<SYSOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSOSC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSOSC_PD_A {
        match self.bits {
            false => SYSOSC_PD_A::POWERED,
            true => SYSOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SYSOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSOSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Field `SYSOSC_PD` writer - Crystal oscillator wake-up configuration"]
pub type SYSOSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, SYSOSC_PD_A, O>;
impl<'a, const O: u8> SYSOSC_PD_W<'a, O> {
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
pub type WDTOSC_PD_R = crate::BitReader<WDTOSC_PD_A>;
#[doc = "Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTOSC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::DISABLED,
            true => WDTOSC_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDTOSC_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDTOSC_PD_A::ENABLED
    }
}
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
pub type WDTOSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, WDTOSC_PD_A, O>;
impl<'a, const O: u8> WDTOSC_PD_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::ENABLED)
    }
}
#[doc = "Field `SYSPLL_PD` reader - System PLL wake-up configuration"]
pub type SYSPLL_PD_R = crate::BitReader<SYSPLL_PD_A>;
#[doc = "System PLL wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPLL_PD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SYSPLL_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPLL_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSPLL_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPLL_PD_A {
        match self.bits {
            false => SYSPLL_PD_A::DISABLED,
            true => SYSPLL_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSPLL_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSPLL_PD_A::ENABLED
    }
}
#[doc = "Field `SYSPLL_PD` writer - System PLL wake-up configuration"]
pub type SYSPLL_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, SYSPLL_PD_A, O>;
impl<'a, const O: u8> SYSPLL_PD_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::ENABLED)
    }
}
#[doc = "Field `ACMP` reader - Analog comparator wake-up configuration"]
pub type ACMP_R = crate::BitReader<ACMP_A>;
#[doc = "Analog comparator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::DISABLED,
            true => ACMP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_A::ENABLED
    }
}
#[doc = "Field `ACMP` writer - Analog comparator wake-up configuration"]
pub type ACMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDAWAKECFG_SPEC, ACMP_A, O>;
impl<'a, const O: u8> ACMP_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn ircout_pd(&self) -> IRCOUT_PD_R {
        IRCOUT_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn irc_pd(&self) -> IRC_PD_R {
        IRC_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&self) -> FLASH_PD_R {
        FLASH_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&self) -> SYSOSC_PD_R {
        SYSOSC_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&self) -> SYSPLL_PD_R {
        SYSPLL_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog comparator wake-up configuration"]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn ircout_pd(&mut self) -> IRCOUT_PD_W<0> {
        IRCOUT_PD_W::new(self)
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn irc_pd(&mut self) -> IRC_PD_W<1> {
        IRC_PD_W::new(self)
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&mut self) -> FLASH_PD_W<2> {
        FLASH_PD_W::new(self)
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W<3> {
        BOD_PD_W::new(self)
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&mut self) -> SYSOSC_PD_W<5> {
        SYSOSC_PD_W::new(self)
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W<6> {
        WDTOSC_PD_W::new(self)
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&mut self) -> SYSPLL_PD_W<7> {
        SYSPLL_PD_W::new(self)
    }
    #[doc = "Bit 15 - Analog comparator wake-up configuration"]
    #[inline(always)]
    pub fn acmp(&mut self) -> ACMP_W<15> {
        ACMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdawakecfg](index.html) module"]
pub struct PDAWAKECFG_SPEC;
impl crate::RegisterSpec for PDAWAKECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdawakecfg::R](R) reader structure"]
impl crate::Readable for PDAWAKECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdawakecfg::W](W) writer structure"]
impl crate::Writable for PDAWAKECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDAWAKECFG to value 0xedf8"]
impl crate::Resettable for PDAWAKECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xedf8
    }
}
