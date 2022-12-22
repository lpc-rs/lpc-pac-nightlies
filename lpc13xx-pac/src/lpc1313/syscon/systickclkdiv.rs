#[doc = "Register `SYSTICKCLKDIV` reader"]
pub struct R(crate::R<SYSTICKCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTICKCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTICKCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTICKCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTICKCLKDIV` writer"]
pub struct W(crate::W<SYSTICKCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTICKCLKDIV_SPEC>;
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
impl From<crate::W<SYSTICKCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTICKCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - SYSTICK clock divider values. 0: Disable SYSTICK timer clock. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - SYSTICK clock divider values. 0: Disable SYSTICK timer clock. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTICKCLKDIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SYSTICK clock divider values. 0: Disable SYSTICK timer clock. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SYSTICK clock divider values. 0: Disable SYSTICK timer clock. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTICK clock divder\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systickclkdiv](index.html) module"]
pub struct SYSTICKCLKDIV_SPEC;
impl crate::RegisterSpec for SYSTICKCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systickclkdiv::R](R) reader structure"]
impl crate::Readable for SYSTICKCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systickclkdiv::W](W) writer structure"]
impl crate::Writable for SYSTICKCLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTICKCLKDIV to value 0"]
impl crate::Resettable for SYSTICKCLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
