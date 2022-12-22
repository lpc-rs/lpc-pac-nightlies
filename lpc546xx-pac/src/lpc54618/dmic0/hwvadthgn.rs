#[doc = "Register `HWVADTHGN` reader"]
pub struct R(crate::R<HWVADTHGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADTHGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADTHGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADTHGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVADTHGN` writer"]
pub struct W(crate::W<HWVADTHGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADTHGN_SPEC>;
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
impl From<crate::W<HWVADTHGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADTHGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THGN` reader - Gain value for the noise estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
pub type THGN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THGN` writer - Gain value for the noise estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
pub type THGN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVADTHGN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Gain value for the noise estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
    #[inline(always)]
    pub fn thgn(&self) -> THGN_R {
        THGN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Gain value for the noise estimator. Values 0 to 14. 0 corresponds to a gain of 1."]
    #[inline(always)]
    pub fn thgn(&mut self) -> THGN_W<0> {
        THGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HWVAD noise estimator gain register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadthgn](index.html) module"]
pub struct HWVADTHGN_SPEC;
impl crate::RegisterSpec for HWVADTHGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadthgn::R](R) reader structure"]
impl crate::Readable for HWVADTHGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwvadthgn::W](W) writer structure"]
impl crate::Writable for HWVADTHGN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWVADTHGN to value 0"]
impl crate::Resettable for HWVADTHGN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
