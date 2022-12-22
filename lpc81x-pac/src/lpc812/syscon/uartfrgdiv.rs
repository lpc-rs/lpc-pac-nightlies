#[doc = "Register `UARTFRGDIV` reader"]
pub struct R(crate::R<UARTFRGDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTFRGDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTFRGDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTFRGDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTFRGDIV` writer"]
pub struct W(crate::W<UARTFRGDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTFRGDIV_SPEC>;
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
impl From<crate::W<UARTFRGDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTFRGDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UARTFRGDIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
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
#[doc = "USART common fractional generator divider value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartfrgdiv](index.html) module"]
pub struct UARTFRGDIV_SPEC;
impl crate::RegisterSpec for UARTFRGDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartfrgdiv::R](R) reader structure"]
impl crate::Readable for UARTFRGDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartfrgdiv::W](W) writer structure"]
impl crate::Writable for UARTFRGDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UARTFRGDIV to value 0"]
impl crate::Resettable for UARTFRGDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
