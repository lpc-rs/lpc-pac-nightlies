#[doc = "Register `FFLR` reader"]
pub struct R(crate::R<FFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFLR` writer"]
pub struct W(crate::W<FFLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFLR_SPEC>;
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
impl From<crate::W<FFLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOFullLevel` reader - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
pub type FIFOFULL_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFOFullLevel` writer - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
pub type FIFOFULL_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
    #[inline(always)]
    pub fn fifofull_level(&self) -> FIFOFULL_LEVEL_R {
        FIFOFULL_LEVEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
    #[inline(always)]
    pub fn fifofull_level(&mut self) -> FIFOFULL_LEVEL_W<0> {
        FIFOFULL_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFOFULL Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fflr](index.html) module"]
pub struct FFLR_SPEC;
impl crate::RegisterSpec for FFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fflr::R](R) reader structure"]
impl crate::Readable for FFLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fflr::W](W) writer structure"]
impl crate::Writable for FFLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFLR to value 0"]
impl crate::Resettable for FFLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
