#[doc = "Register `SYSPLLCTRL` reader"]
pub struct R(crate::R<SYSPLLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLCTRL` writer"]
pub struct W(crate::W<SYSPLLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLCTRL_SPEC>;
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
impl From<crate::W<SYSPLLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELR` reader - Bandwidth select R value."]
pub type SELR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELR` writer - Bandwidth select R value."]
pub type SELR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SELI` reader - Bandwidth select I value."]
pub type SELI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELI` writer - Bandwidth select I value."]
pub type SELI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLCTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `SELP` reader - Bandwidth select P value."]
pub type SELP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELP` writer - Bandwidth select P value."]
pub type SELP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `BYPASS` reader - PLL bypass control."]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
#[doc = "PLL bypass control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Bypass disabled. PLL CCO is sent to the PLL post-dividers."]
    DISABLED = 0,
    #[doc = "1: Bypass enabled. PLL input clock is sent directly to the PLL output (default)."]
    ENABLED = 1,
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
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::ENABLED
    }
}
#[doc = "Field `BYPASS` writer - PLL bypass control."]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLCTRL_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "Bypass disabled. PLL CCO is sent to the PLL post-dividers."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "Bypass enabled. PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
}
#[doc = "Field `UPLIMOFF` reader - Disable upper frequency limiter."]
pub type UPLIMOFF_R = crate::BitReader<bool>;
#[doc = "Field `UPLIMOFF` writer - Disable upper frequency limiter."]
pub type UPLIMOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLCTRL_SPEC, bool, O>;
#[doc = "Field `DIRECTI` reader - PLL0 direct input enable."]
pub type DIRECTI_R = crate::BitReader<bool>;
#[doc = "Field `DIRECTI` writer - PLL0 direct input enable."]
pub type DIRECTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLCTRL_SPEC, bool, O>;
#[doc = "Field `DIRECTO` reader - PLL0 direct output enable."]
pub type DIRECTO_R = crate::BitReader<DIRECTO_A>;
#[doc = "PLL0 direct output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTO_A {
    #[doc = "0: Disabled. The PLL output divider (P divider) is used to create the PLL output."]
    DISABLED = 0,
    #[doc = "1: Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output."]
    ENABLED = 1,
}
impl From<DIRECTO_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTO_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRECTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECTO_A {
        match self.bits {
            false => DIRECTO_A::DISABLED,
            true => DIRECTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIRECTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIRECTO_A::ENABLED
    }
}
#[doc = "Field `DIRECTO` writer - PLL0 direct output enable."]
pub type DIRECTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLCTRL_SPEC, DIRECTO_A, O>;
impl<'a, const O: u8> DIRECTO_W<'a, O> {
    #[doc = "Disabled. The PLL output divider (P divider) is used to create the PLL output."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECTO_A::DISABLED)
    }
    #[doc = "Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECTO_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&self) -> SELR_R {
        SELR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&self) -> SELI_R {
        SELI_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&self) -> SELP_R {
        SELP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - PLL bypass control."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable upper frequency limiter."]
    #[inline(always)]
    pub fn uplimoff(&self) -> UPLIMOFF_R {
        UPLIMOFF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - PLL0 direct input enable."]
    #[inline(always)]
    pub fn directi(&self) -> DIRECTI_R {
        DIRECTI_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PLL0 direct output enable."]
    #[inline(always)]
    pub fn directo(&self) -> DIRECTO_R {
        DIRECTO_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&mut self) -> SELR_W<0> {
        SELR_W::new(self)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&mut self) -> SELI_W<4> {
        SELI_W::new(self)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&mut self) -> SELP_W<10> {
        SELP_W::new(self)
    }
    #[doc = "Bit 15 - PLL bypass control."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<15> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 17 - Disable upper frequency limiter."]
    #[inline(always)]
    pub fn uplimoff(&mut self) -> UPLIMOFF_W<17> {
        UPLIMOFF_W::new(self)
    }
    #[doc = "Bit 19 - PLL0 direct input enable."]
    #[inline(always)]
    pub fn directi(&mut self) -> DIRECTI_W<19> {
        DIRECTI_W::new(self)
    }
    #[doc = "Bit 20 - PLL0 direct output enable."]
    #[inline(always)]
    pub fn directo(&mut self) -> DIRECTO_W<20> {
        DIRECTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllctrl](index.html) module"]
pub struct SYSPLLCTRL_SPEC;
impl crate::RegisterSpec for SYSPLLCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllctrl::R](R) reader structure"]
impl crate::Readable for SYSPLLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllctrl::W](W) writer structure"]
impl crate::Writable for SYSPLLCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPLLCTRL to value 0"]
impl crate::Resettable for SYSPLLCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
