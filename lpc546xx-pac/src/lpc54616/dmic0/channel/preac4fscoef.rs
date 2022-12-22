#[doc = "Register `PREAC4FSCOEF` reader"]
pub struct R(crate::R<PREAC4FSCOEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREAC4FSCOEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREAC4FSCOEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREAC4FSCOEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREAC4FSCOEF` writer"]
pub struct W(crate::W<PREAC4FSCOEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREAC4FSCOEF_SPEC>;
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
impl From<crate::W<PREAC4FSCOEF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREAC4FSCOEF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Pre-emphasis filer coefficient for 4 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
pub type COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP` writer - Pre-emphasis filer coefficient for 4 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREAC4FSCOEF_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Pre-emphasis filer coefficient for 4 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pre-emphasis filer coefficient for 4 FS mode. 0 = Compensation = 0 1 = Compensation = 16 2 = Compensation = 15 3 = Compensation = 13"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<0> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pre-Emphasis Filter Coefficient for 4 FS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preac4fscoef](index.html) module"]
pub struct PREAC4FSCOEF_SPEC;
impl crate::RegisterSpec for PREAC4FSCOEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [preac4fscoef::R](R) reader structure"]
impl crate::Readable for PREAC4FSCOEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preac4fscoef::W](W) writer structure"]
impl crate::Writable for PREAC4FSCOEF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREAC4FSCOEF to value 0"]
impl crate::Resettable for PREAC4FSCOEF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
