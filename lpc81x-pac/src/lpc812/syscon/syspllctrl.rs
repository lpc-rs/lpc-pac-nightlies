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
#[doc = "Field `MSEL` reader - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
pub type MSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSEL` writer - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
pub type MSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PSEL` reader - Post divider ratio P. The division ratio is 2 x P."]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "Post divider ratio P. The division ratio is 2 x P.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: P = 1"]
    PSEL_0 = 0,
    #[doc = "1: P = 2"]
    PSEL_1 = 1,
    #[doc = "2: P = 4"]
    PSEL_2 = 2,
    #[doc = "3: P = 8"]
    PSEL_3 = 3,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::PSEL_0,
            1 => PSEL_A::PSEL_1,
            2 => PSEL_A::PSEL_2,
            3 => PSEL_A::PSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == PSEL_A::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == PSEL_A::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == PSEL_A::PSEL_2
    }
    #[doc = "Checks if the value of the field is `PSEL_3`"]
    #[inline(always)]
    pub fn is_psel_3(&self) -> bool {
        *self == PSEL_A::PSEL_3
    }
}
#[doc = "Field `PSEL` writer - Post divider ratio P. The division ratio is 2 x P."]
pub type PSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SYSPLLCTRL_SPEC, u8, PSEL_A, 2, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "P = 1"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_0)
    }
    #[doc = "P = 2"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_1)
    }
    #[doc = "P = 4"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_2)
    }
    #[doc = "P = 8"]
    #[inline(always)]
    pub fn psel_3(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_3)
    }
}
impl R {
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Feedback divider value. The division value M is the programmed MSEL value + 1. 00000: Division ratio M = 1 to 11111: Division ratio M = 32"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W<0> {
        MSEL_W::new(self)
    }
    #[doc = "Bits 5:6 - Post divider ratio P. The division ratio is 2 x P."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W<5> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllctrl](index.html) module"]
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
