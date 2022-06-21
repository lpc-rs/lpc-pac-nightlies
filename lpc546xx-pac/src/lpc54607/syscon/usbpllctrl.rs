#[doc = "Register `USBPLLCTRL` reader"]
pub struct R(crate::R<USBPLLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLCTRL` writer"]
pub struct W(crate::W<USBPLLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLCTRL_SPEC>;
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
impl From<crate::W<USBPLLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSEL` reader - PLL feedback Divider value."]
pub type MSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSEL` writer - PLL feedback Divider value."]
pub type MSEL_W<'a> = crate::FieldWriter<'a, u32, USBPLLCTRL_SPEC, u8, u8, 8, 0>;
#[doc = "Field `PSEL` reader - PLL Divider value."]
pub type PSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEL` writer - PLL Divider value."]
pub type PSEL_W<'a> = crate::FieldWriter<'a, u32, USBPLLCTRL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `NSEL` reader - PLL feedback Divider value."]
pub type NSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSEL` writer - PLL feedback Divider value."]
pub type NSEL_W<'a> = crate::FieldWriter<'a, u32, USBPLLCTRL_SPEC, u8, u8, 2, 10>;
#[doc = "Direct CCO clock output control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECT_A {
    #[doc = "0: CCO Clock signal goes through post divider."]
    DISABLED = 0,
    #[doc = "1: CCO Clock signal goes directly to output(s).."]
    ENABLED = 1,
}
impl From<DIRECT_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECT` reader - Direct CCO clock output control."]
pub type DIRECT_R = crate::BitReader<DIRECT_A>;
impl DIRECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECT_A {
        match self.bits {
            false => DIRECT_A::DISABLED,
            true => DIRECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIRECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIRECT_A::ENABLED
    }
}
#[doc = "Field `DIRECT` writer - Direct CCO clock output control."]
pub type DIRECT_W<'a> = crate::BitWriter<'a, u32, USBPLLCTRL_SPEC, DIRECT_A, 12>;
impl<'a> DIRECT_W<'a> {
    #[doc = "CCO Clock signal goes through post divider."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECT_A::DISABLED)
    }
    #[doc = "CCO Clock signal goes directly to output(s).."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECT_A::ENABLED)
    }
}
#[doc = "Input clock bypass control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: CCO clock is sent to post dividers.."]
    DISABLED = 0,
    #[doc = "1: PLL input clock is sent to post dividers.."]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Input clock bypass control."]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
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
#[doc = "Field `BYPASS` writer - Input clock bypass control."]
pub type BYPASS_W<'a> = crate::BitWriter<'a, u32, USBPLLCTRL_SPEC, BYPASS_A, 13>;
impl<'a> BYPASS_W<'a> {
    #[doc = "CCO clock is sent to post dividers.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "PLL input clock is sent to post dividers.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
}
#[doc = "Field `FBSEL` reader - Feedback divider input clock control."]
pub type FBSEL_R = crate::BitReader<bool>;
#[doc = "Field `FBSEL` writer - Feedback divider input clock control."]
pub type FBSEL_W<'a> = crate::BitWriter<'a, u32, USBPLLCTRL_SPEC, bool, 14>;
impl R {
    #[doc = "Bits 0:7 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - PLL Divider value."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn nsel(&self) -> NSEL_R {
        NSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Direct CCO clock output control."]
    #[inline(always)]
    pub fn direct(&self) -> DIRECT_R {
        DIRECT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input clock bypass control."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Feedback divider input clock control."]
    #[inline(always)]
    pub fn fbsel(&self) -> FBSEL_R {
        FBSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - PLL Divider value."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn nsel(&mut self) -> NSEL_W {
        NSEL_W::new(self)
    }
    #[doc = "Bit 12 - Direct CCO clock output control."]
    #[inline(always)]
    pub fn direct(&mut self) -> DIRECT_W {
        DIRECT_W::new(self)
    }
    #[doc = "Bit 13 - Input clock bypass control."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 14 - Feedback divider input clock control."]
    #[inline(always)]
    pub fn fbsel(&mut self) -> FBSEL_W {
        FBSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllctrl](index.html) module"]
pub struct USBPLLCTRL_SPEC;
impl crate::RegisterSpec for USBPLLCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpllctrl::R](R) reader structure"]
impl crate::Readable for USBPLLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllctrl::W](W) writer structure"]
impl crate::Writable for USBPLLCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPLLCTRL to value 0"]
impl crate::Resettable for USBPLLCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
