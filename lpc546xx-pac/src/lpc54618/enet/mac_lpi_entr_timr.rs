#[doc = "Register `MAC_LPI_ENTR_TIMR` reader"]
pub struct R(crate::R<MAC_LPI_ENTR_TIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_LPI_ENTR_TIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_LPI_ENTR_TIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_LPI_ENTR_TIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_LPI_ENTR_TIMR` writer"]
pub struct W(crate::W<MAC_LPI_ENTR_TIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_LPI_ENTR_TIMR_SPEC>;
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
impl From<crate::W<MAC_LPI_ENTR_TIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_LPI_ENTR_TIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPIET` reader - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames."]
pub type LPIET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LPIET` writer - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames."]
pub type LPIET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_LPI_ENTR_TIMR_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 3:19 - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames."]
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new(((self.bits >> 3) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:19 - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames."]
    #[inline(always)]
    pub fn lpiet(&mut self) -> LPIET_W<3> {
        LPIET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI entry Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_lpi_entr_timr](index.html) module"]
pub struct MAC_LPI_ENTR_TIMR_SPEC;
impl crate::RegisterSpec for MAC_LPI_ENTR_TIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_lpi_entr_timr::R](R) reader structure"]
impl crate::Readable for MAC_LPI_ENTR_TIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_lpi_entr_timr::W](W) writer structure"]
impl crate::Writable for MAC_LPI_ENTR_TIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_LPI_ENTR_TIMR to value 0"]
impl crate::Resettable for MAC_LPI_ENTR_TIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
