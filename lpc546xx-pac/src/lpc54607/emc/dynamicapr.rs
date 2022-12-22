#[doc = "Register `DYNAMICAPR` reader"]
pub struct R(crate::R<DYNAMICAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICAPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICAPR` writer"]
pub struct W(crate::W<DYNAMICAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICAPR_SPEC>;
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
impl From<crate::W<DYNAMICAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICAPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAPR` reader - Last-data-out to active command time."]
pub type TAPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAPR` writer - Last-data-out to active command time."]
pub type TAPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICAPR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Last-data-out to active command time."]
    #[inline(always)]
    pub fn tapr(&self) -> TAPR_R {
        TAPR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Last-data-out to active command time."]
    #[inline(always)]
    pub fn tapr(&mut self) -> TAPR_W<0> {
        TAPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last-data-out to active command time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicapr](index.html) module"]
pub struct DYNAMICAPR_SPEC;
impl crate::RegisterSpec for DYNAMICAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicapr::R](R) reader structure"]
impl crate::Readable for DYNAMICAPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicapr::W](W) writer structure"]
impl crate::Writable for DYNAMICAPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICAPR to value 0x0f"]
impl crate::Resettable for DYNAMICAPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
