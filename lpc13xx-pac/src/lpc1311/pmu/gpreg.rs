#[doc = "Register `GPREG%s` reader"]
pub struct R(crate::R<GPREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPREG%s` writer"]
pub struct W(crate::W<GPREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPREG_SPEC>;
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
impl From<crate::W<GPREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPDATA` reader - Data retained during Deep power-down mode."]
pub type GPDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPDATA` writer - Data retained during Deep power-down mode."]
pub type GPDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPREG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&mut self) -> GPDATA_W<0> {
        GPDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpreg](index.html) module"]
pub struct GPREG_SPEC;
impl crate::RegisterSpec for GPREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpreg::R](R) reader structure"]
impl crate::Readable for GPREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpreg::W](W) writer structure"]
impl crate::Writable for GPREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPREG%s to value 0"]
impl crate::Resettable for GPREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
