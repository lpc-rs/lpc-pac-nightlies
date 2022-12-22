#[doc = "Register `PDSLEEPCFG` reader"]
pub struct R(crate::R<PDSLEEPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSLEEPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSLEEPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSLEEPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSLEEPCFG` writer"]
pub struct W(crate::W<PDSLEEPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSLEEPCFG_SPEC>;
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
impl From<crate::W<PDSLEEPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSLEEPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD_PD` reader - BOD power-down control for Deep-sleep and Power-down mode"]
pub type BOD_PD_R = crate::BitReader<BOD_PD_A>;
#[doc = "BOD power-down control for Deep-sleep and Power-down mode\n\nValue on reset: 1"]
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
#[doc = "Field `BOD_PD` writer - BOD power-down control for Deep-sleep and Power-down mode"]
pub type BOD_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSLEEPCFG_SPEC, BOD_PD_A, O>;
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
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running."]
pub type WDTOSC_PD_R = crate::BitReader<WDTOSC_PD_A>;
#[doc = "Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running.\n\nValue on reset: 1"]
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
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running."]
pub type WDTOSC_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDSLEEPCFG_SPEC, WDTOSC_PD_A, O>;
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
impl R {
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running."]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W<3> {
        BOD_PD_W::new(self)
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running."]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W<6> {
        WDTOSC_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep-sleep configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg](index.html) module"]
pub struct PDSLEEPCFG_SPEC;
impl crate::RegisterSpec for PDSLEEPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsleepcfg::R](R) reader structure"]
impl crate::Readable for PDSLEEPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg::W](W) writer structure"]
impl crate::Writable for PDSLEEPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDSLEEPCFG to value 0xffff"]
impl crate::Resettable for PDSLEEPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
