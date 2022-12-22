#[doc = "Register `SYSPLLCLKUEN` reader"]
pub struct R(crate::R<SYSPLLCLKUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLCLKUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLCLKUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLCLKUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLCLKUEN` writer"]
pub struct W(crate::W<SYSPLLCLKUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLCLKUEN_SPEC>;
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
impl From<crate::W<SYSPLLCLKUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLCLKUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - Enable system PLL clock source update"]
pub type ENA_R = crate::BitReader<ENA_A>;
#[doc = "Enable system PLL clock source update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_A {
    #[doc = "0: no change"]
    NO_CHANGE = 0,
    #[doc = "1: update clock source"]
    UPDATED = 1,
}
impl From<ENA_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_A {
        match self.bits {
            false => ENA_A::NO_CHANGE,
            true => ENA_A::UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == ENA_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `UPDATED`"]
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == ENA_A::UPDATED
    }
}
#[doc = "Field `ENA` writer - Enable system PLL clock source update"]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLCLKUEN_SPEC, ENA_A, O>;
impl<'a, const O: u8> ENA_W<'a, O> {
    #[doc = "no change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ENA_A::NO_CHANGE)
    }
    #[doc = "update clock source"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(ENA_A::UPDATED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable system PLL clock source update"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable system PLL clock source update"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<0> {
        ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL clock source update enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllclkuen](index.html) module"]
pub struct SYSPLLCLKUEN_SPEC;
impl crate::RegisterSpec for SYSPLLCLKUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllclkuen::R](R) reader structure"]
impl crate::Readable for SYSPLLCLKUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllclkuen::W](W) writer structure"]
impl crate::Writable for SYSPLLCLKUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPLLCLKUEN to value 0"]
impl crate::Resettable for SYSPLLCLKUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
