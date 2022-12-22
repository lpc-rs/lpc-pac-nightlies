#[doc = "Register `EMCDLYCAL` reader"]
pub struct R(crate::R<EMCDLYCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCDLYCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCDLYCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCDLYCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMCDLYCAL` writer"]
pub struct W(crate::W<EMCDLYCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCDLYCAL_SPEC>;
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
impl From<crate::W<EMCDLYCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCDLYCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALVALUE` reader - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz."]
pub type CALVALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALVALUE` writer - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz."]
pub type CALVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMCDLYCAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `START` reader - Start control bit for the EMC calibration counter."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start control bit for the EMC calibration counter."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMCDLYCAL_SPEC, bool, O>;
#[doc = "Field `DONE` reader - Measurement completion flag."]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Measurement completion flag."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMCDLYCAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz."]
    #[inline(always)]
    pub fn calvalue(&self) -> CALVALUE_R {
        CALVALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - Start control bit for the EMC calibration counter."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Measurement completion flag."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz."]
    #[inline(always)]
    pub fn calvalue(&mut self) -> CALVALUE_W<0> {
        CALVALUE_W::new(self)
    }
    #[doc = "Bit 14 - Start control bit for the EMC calibration counter."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<14> {
        START_W::new(self)
    }
    #[doc = "Bit 15 - Measurement completion flag."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<15> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMC delay chain calibration control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcdlycal](index.html) module"]
pub struct EMCDLYCAL_SPEC;
impl crate::RegisterSpec for EMCDLYCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emcdlycal::R](R) reader structure"]
impl crate::Readable for EMCDLYCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emcdlycal::W](W) writer structure"]
impl crate::Writable for EMCDLYCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMCDLYCAL to value 0"]
impl crate::Resettable for EMCDLYCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
