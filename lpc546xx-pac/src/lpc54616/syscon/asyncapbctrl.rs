#[doc = "Register `ASYNCAPBCTRL` reader"]
pub struct R(crate::R<ASYNCAPBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCAPBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCAPBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCAPBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCAPBCTRL` writer"]
pub struct W(crate::W<ASYNCAPBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCAPBCTRL_SPEC>;
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
impl From<crate::W<ASYNCAPBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCAPBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enables the asynchronous APB bridge and subsystem."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enables the asynchronous APB bridge and subsystem.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disabled. Asynchronous APB bridge is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Asynchronous APB bridge is enabled."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Field `ENABLE` writer - Enables the asynchronous APB bridge and subsystem."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNCAPBCTRL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Disabled. Asynchronous APB bridge is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Enabled. Asynchronous APB bridge is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enables the asynchronous APB bridge and subsystem."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the asynchronous APB bridge and subsystem."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous APB Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbctrl](index.html) module"]
pub struct ASYNCAPBCTRL_SPEC;
impl crate::RegisterSpec for ASYNCAPBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asyncapbctrl::R](R) reader structure"]
impl crate::Readable for ASYNCAPBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncapbctrl::W](W) writer structure"]
impl crate::Writable for ASYNCAPBCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCAPBCTRL to value 0x01"]
impl crate::Resettable for ASYNCAPBCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
