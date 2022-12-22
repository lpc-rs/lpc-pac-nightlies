#[doc = "Register `WDTCLKUEN` reader"]
pub struct R(crate::R<WDTCLKUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCLKUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCLKUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCLKUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCLKUEN` writer"]
pub struct W(crate::W<WDTCLKUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCLKUEN_SPEC>;
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
impl From<crate::W<WDTCLKUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCLKUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - Enable WDT clock source update"]
pub type ENA_R = crate::BitReader<ENA_A>;
#[doc = "Enable WDT clock source update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_A {
    #[doc = "0: No change"]
    NO_CHANGE = 0,
    #[doc = "1: Update clock source"]
    UPDATE_CLOCK_SOURCE = 1,
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
            true => ENA_A::UPDATE_CLOCK_SOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == ENA_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `UPDATE_CLOCK_SOURCE`"]
    #[inline(always)]
    pub fn is_update_clock_source(&self) -> bool {
        *self == ENA_A::UPDATE_CLOCK_SOURCE
    }
}
#[doc = "Field `ENA` writer - Enable WDT clock source update"]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTCLKUEN_SPEC, ENA_A, O>;
impl<'a, const O: u8> ENA_W<'a, O> {
    #[doc = "No change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ENA_A::NO_CHANGE)
    }
    #[doc = "Update clock source"]
    #[inline(always)]
    pub fn update_clock_source(self) -> &'a mut W {
        self.variant(ENA_A::UPDATE_CLOCK_SOURCE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable WDT clock source update"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WDT clock source update"]
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
#[doc = "WDT clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclkuen](index.html) module"]
pub struct WDTCLKUEN_SPEC;
impl crate::RegisterSpec for WDTCLKUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtclkuen::R](R) reader structure"]
impl crate::Readable for WDTCLKUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtclkuen::W](W) writer structure"]
impl crate::Writable for WDTCLKUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCLKUEN to value 0"]
impl crate::Resettable for WDTCLKUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
