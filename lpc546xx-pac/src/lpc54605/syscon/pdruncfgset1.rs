#[doc = "Register `PDRUNCFGSET1` reader"]
pub struct R(crate::R<PDRUNCFGSET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRUNCFGSET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRUNCFGSET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRUNCFGSET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRUNCFGSET1` writer"]
pub struct W(crate::W<PDRUNCFGSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFGSET1_SPEC>;
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
impl From<crate::W<PDRUNCFGSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFGSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDEN_USB1_PHY` reader - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
pub type PDEN_USB1_PHY_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_USB1_PHY` writer - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
pub type PDEN_USB1_PHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET1_SPEC, bool, O>;
#[doc = "Field `PDEN_USB1_PLL` reader - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
pub type PDEN_USB1_PLL_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_USB1_PLL` writer - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
pub type PDEN_USB1_PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET1_SPEC, bool, O>;
#[doc = "Field `PDEN_AUD_PLL` reader - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
pub type PDEN_AUD_PLL_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_AUD_PLL` writer - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
pub type PDEN_AUD_PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET1_SPEC, bool, O>;
#[doc = "Field `PDEN_SYSOSC` reader - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
pub type PDEN_SYSOSC_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_SYSOSC` writer - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
pub type PDEN_SYSOSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET1_SPEC, bool, O>;
#[doc = "Field `PDEN_EEPROM` reader - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
pub type PDEN_EEPROM_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_EEPROM` writer - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
pub type PDEN_EEPROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET1_SPEC, bool, O>;
#[doc = "Field `PDEN_RNG` reader - Random Number Generator Power."]
pub type PDEN_RNG_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_RNG` writer - Random Number Generator Power."]
pub type PDEN_RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_phy(&self) -> PDEN_USB1_PHY_R {
        PDEN_USB1_PHY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_pll(&self) -> PDEN_USB1_PLL_R {
        PDEN_USB1_PLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_aud_pll(&self) -> PDEN_AUD_PLL_R {
        PDEN_AUD_PLL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_sysosc(&self) -> PDEN_SYSOSC_R {
        PDEN_SYSOSC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_eeprom(&self) -> PDEN_EEPROM_R {
        PDEN_EEPROM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Random Number Generator Power."]
    #[inline(always)]
    pub fn pden_rng(&self) -> PDEN_RNG_R {
        PDEN_RNG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_phy(&mut self) -> PDEN_USB1_PHY_W<0> {
        PDEN_USB1_PHY_W::new(self)
    }
    #[doc = "Bit 1 - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_pll(&mut self) -> PDEN_USB1_PLL_W<1> {
        PDEN_USB1_PLL_W::new(self)
    }
    #[doc = "Bit 2 - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_aud_pll(&mut self) -> PDEN_AUD_PLL_W<2> {
        PDEN_AUD_PLL_W::new(self)
    }
    #[doc = "Bit 3 - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_sysosc(&mut self) -> PDEN_SYSOSC_W<3> {
        PDEN_SYSOSC_W::new(self)
    }
    #[doc = "Bit 5 - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_eeprom(&mut self) -> PDEN_EEPROM_W<5> {
        PDEN_EEPROM_W::new(self)
    }
    #[doc = "Bit 7 - Random Number Generator Power."]
    #[inline(always)]
    pub fn pden_rng(&mut self) -> PDEN_RNG_W<7> {
        PDEN_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power configuration set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgset1](index.html) module"]
pub struct PDRUNCFGSET1_SPEC;
impl crate::RegisterSpec for PDRUNCFGSET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdruncfgset1::R](R) reader structure"]
impl crate::Readable for PDRUNCFGSET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdruncfgset1::W](W) writer structure"]
impl crate::Writable for PDRUNCFGSET1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDRUNCFGSET1 to value 0"]
impl crate::Resettable for PDRUNCFGSET1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
