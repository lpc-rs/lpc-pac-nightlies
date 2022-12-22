#[doc = "Register `IOCONCLKDIV3` reader"]
pub struct R(crate::R<IOCONCLKDIV3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCONCLKDIV3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCONCLKDIV3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCONCLKDIV3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCONCLKDIV3` writer"]
pub struct W(crate::W<IOCONCLKDIV3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCONCLKDIV3_SPEC>;
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
impl From<crate::W<IOCONCLKDIV3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCONCLKDIV3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCONCLKDIV3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
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
#[doc = "Peripheral clock 3 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv3](index.html) module"]
pub struct IOCONCLKDIV3_SPEC;
impl crate::RegisterSpec for IOCONCLKDIV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioconclkdiv3::R](R) reader structure"]
impl crate::Readable for IOCONCLKDIV3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv3::W](W) writer structure"]
impl crate::Writable for IOCONCLKDIV3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCONCLKDIV3 to value 0"]
impl crate::Resettable for IOCONCLKDIV3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
