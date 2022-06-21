#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPONEXIT_A {
    #[doc = "0: o not sleep when returning to Thread mode"]
    SLEEPONEXIT_0 = 0,
    #[doc = "1: enter sleep, or deep sleep, on return from an ISR"]
    SLEEPONEXIT_1 = 1,
}
impl From<SLEEPONEXIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPONEXIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPONEXIT` reader - no description available"]
pub type SLEEPONEXIT_R = crate::BitReader<SLEEPONEXIT_A>;
impl SLEEPONEXIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPONEXIT_A {
        match self.bits {
            false => SLEEPONEXIT_A::SLEEPONEXIT_0,
            true => SLEEPONEXIT_A::SLEEPONEXIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEPONEXIT_0`"]
    #[inline(always)]
    pub fn is_sleeponexit_0(&self) -> bool {
        *self == SLEEPONEXIT_A::SLEEPONEXIT_0
    }
    #[doc = "Checks if the value of the field is `SLEEPONEXIT_1`"]
    #[inline(always)]
    pub fn is_sleeponexit_1(&self) -> bool {
        *self == SLEEPONEXIT_A::SLEEPONEXIT_1
    }
}
#[doc = "Field `SLEEPONEXIT` writer - no description available"]
pub type SLEEPONEXIT_W<'a> = crate::BitWriter<'a, u32, SCR_SPEC, SLEEPONEXIT_A, 1>;
impl<'a> SLEEPONEXIT_W<'a> {
    #[doc = "o not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn sleeponexit_0(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::SLEEPONEXIT_0)
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn sleeponexit_1(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::SLEEPONEXIT_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPDEEP_A {
    #[doc = "0: sleep"]
    SLEEPDEEP_0 = 0,
    #[doc = "1: deep sleep"]
    SLEEPDEEP_1 = 1,
}
impl From<SLEEPDEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPDEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPDEEP` reader - no description available"]
pub type SLEEPDEEP_R = crate::BitReader<SLEEPDEEP_A>;
impl SLEEPDEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPDEEP_A {
        match self.bits {
            false => SLEEPDEEP_A::SLEEPDEEP_0,
            true => SLEEPDEEP_A::SLEEPDEEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEPDEEP_0`"]
    #[inline(always)]
    pub fn is_sleepdeep_0(&self) -> bool {
        *self == SLEEPDEEP_A::SLEEPDEEP_0
    }
    #[doc = "Checks if the value of the field is `SLEEPDEEP_1`"]
    #[inline(always)]
    pub fn is_sleepdeep_1(&self) -> bool {
        *self == SLEEPDEEP_A::SLEEPDEEP_1
    }
}
#[doc = "Field `SLEEPDEEP` writer - no description available"]
pub type SLEEPDEEP_W<'a> = crate::BitWriter<'a, u32, SCR_SPEC, SLEEPDEEP_A, 2>;
impl<'a> SLEEPDEEP_W<'a> {
    #[doc = "sleep"]
    #[inline(always)]
    pub fn sleepdeep_0(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::SLEEPDEEP_0)
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn sleepdeep_1(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::SLEEPDEEP_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPEND_A {
    #[doc = "0: only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    SEVONPEND_0 = 0,
    #[doc = "1: enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    SEVONPEND_1 = 1,
}
impl From<SEVONPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SEVONPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVONPEND` reader - no description available"]
pub type SEVONPEND_R = crate::BitReader<SEVONPEND_A>;
impl SEVONPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEVONPEND_A {
        match self.bits {
            false => SEVONPEND_A::SEVONPEND_0,
            true => SEVONPEND_A::SEVONPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEVONPEND_0`"]
    #[inline(always)]
    pub fn is_sevonpend_0(&self) -> bool {
        *self == SEVONPEND_A::SEVONPEND_0
    }
    #[doc = "Checks if the value of the field is `SEVONPEND_1`"]
    #[inline(always)]
    pub fn is_sevonpend_1(&self) -> bool {
        *self == SEVONPEND_A::SEVONPEND_1
    }
}
#[doc = "Field `SEVONPEND` writer - no description available"]
pub type SEVONPEND_W<'a> = crate::BitWriter<'a, u32, SCR_SPEC, SEVONPEND_A, 4>;
impl<'a> SEVONPEND_W<'a> {
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn sevonpend_0(self) -> &'a mut W {
        self.variant(SEVONPEND_A::SEVONPEND_0)
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn sevonpend_1(self) -> &'a mut W {
        self.variant(SEVONPEND_A::SEVONPEND_1)
    }
}
impl R {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W {
        SEVONPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
