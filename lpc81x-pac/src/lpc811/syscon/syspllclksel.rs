#[doc = "Register `SYSPLLCLKSEL` reader"]
pub struct R(crate::R<SYSPLLCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLCLKSEL` writer"]
pub struct W(crate::W<SYSPLLCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLCLKSEL_SPEC>;
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
impl From<crate::W<SYSPLLCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - System PLL clock source"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "System PLL clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC"]
    IRC = 0,
    #[doc = "1: Crystal Oscillator (SYSOSC)"]
    SYSOSC = 1,
    #[doc = "3: CLKIN. External clock input."]
    CLKIN = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::IRC),
            1 => Some(SEL_A::SYSOSC),
            3 => Some(SEL_A::CLKIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == SEL_A::IRC
    }
    #[doc = "Checks if the value of the field is `SYSOSC`"]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == SEL_A::SYSOSC
    }
    #[doc = "Checks if the value of the field is `CLKIN`"]
    #[inline(always)]
    pub fn is_clkin(&self) -> bool {
        *self == SEL_A::CLKIN
    }
}
#[doc = "Field `SEL` writer - System PLL clock source"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLCLKSEL_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(SEL_A::IRC)
    }
    #[doc = "Crystal Oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut W {
        self.variant(SEL_A::SYSOSC)
    }
    #[doc = "CLKIN. External clock input."]
    #[inline(always)]
    pub fn clkin(self) -> &'a mut W {
        self.variant(SEL_A::CLKIN)
    }
}
impl R {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllclksel](index.html) module"]
pub struct SYSPLLCLKSEL_SPEC;
impl crate::RegisterSpec for SYSPLLCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllclksel::R](R) reader structure"]
impl crate::Readable for SYSPLLCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllclksel::W](W) writer structure"]
impl crate::Writable for SYSPLLCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPLLCLKSEL to value 0"]
impl crate::Resettable for SYSPLLCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
