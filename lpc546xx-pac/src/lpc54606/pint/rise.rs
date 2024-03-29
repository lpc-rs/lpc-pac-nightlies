#[doc = "Register `RISE` reader"]
pub struct R(crate::R<RISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISE` writer"]
pub struct W(crate::W<RISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISE_SPEC>;
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
impl From<crate::W<RISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDET` reader - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
pub type RDET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDET` writer - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
pub type RDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RISE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&self) -> RDET_R {
        RDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&mut self) -> RDET_W<0> {
        RDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt rising edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rise](index.html) module"]
pub struct RISE_SPEC;
impl crate::RegisterSpec for RISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rise::R](R) reader structure"]
impl crate::Readable for RISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rise::W](W) writer structure"]
impl crate::Writable for RISE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RISE to value 0"]
impl crate::Resettable for RISE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
