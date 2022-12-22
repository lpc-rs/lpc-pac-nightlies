#[doc = "Register `CLKOUTUEN` reader"]
pub struct R(crate::R<CLKOUTUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTUEN` writer"]
pub struct W(crate::W<CLKOUTUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTUEN_SPEC>;
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
impl From<crate::W<CLKOUTUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - Enable CLKOUT clock source update."]
pub type ENA_R = crate::BitReader<ENA_A>;
#[doc = "Enable CLKOUT clock source update.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_A {
    #[doc = "0: No change"]
    ENA_0 = 0,
    #[doc = "1: Update clock source"]
    ENA_1 = 1,
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
            false => ENA_A::ENA_0,
            true => ENA_A::ENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENA_0`"]
    #[inline(always)]
    pub fn is_ena_0(&self) -> bool {
        *self == ENA_A::ENA_0
    }
    #[doc = "Checks if the value of the field is `ENA_1`"]
    #[inline(always)]
    pub fn is_ena_1(&self) -> bool {
        *self == ENA_A::ENA_1
    }
}
#[doc = "Field `ENA` writer - Enable CLKOUT clock source update."]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKOUTUEN_SPEC, ENA_A, O>;
impl<'a, const O: u8> ENA_W<'a, O> {
    #[doc = "No change"]
    #[inline(always)]
    pub fn ena_0(self) -> &'a mut W {
        self.variant(ENA_A::ENA_0)
    }
    #[doc = "Update clock source"]
    #[inline(always)]
    pub fn ena_1(self) -> &'a mut W {
        self.variant(ENA_A::ENA_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable CLKOUT clock source update."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CLKOUT clock source update."]
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
#[doc = "CLKOUT clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutuen](index.html) module"]
pub struct CLKOUTUEN_SPEC;
impl crate::RegisterSpec for CLKOUTUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutuen::R](R) reader structure"]
impl crate::Readable for CLKOUTUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutuen::W](W) writer structure"]
impl crate::Writable for CLKOUTUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKOUTUEN to value 0"]
impl crate::Resettable for CLKOUTUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
