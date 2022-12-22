#[doc = "Register `SYSTCKCAL` reader"]
pub struct R(crate::R<SYSTCKCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTCKCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTCKCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTCKCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTCKCAL` writer"]
pub struct W(crate::W<SYSTCKCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTCKCAL_SPEC>;
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
impl From<crate::W<SYSTCKCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTCKCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - System tick timer calibration value."]
pub type CAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAL` writer - System tick timer calibration value."]
pub type CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTCKCAL_SPEC, u32, u32, 24, O>;
#[doc = "Field `SKEW` reader - Initial value for the Systick timer."]
pub type SKEW_R = crate::BitReader<bool>;
#[doc = "Field `SKEW` writer - Initial value for the Systick timer."]
pub type SKEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSTCKCAL_SPEC, bool, O>;
#[doc = "Field `NOREF` reader - Initial value for the Systick timer."]
pub type NOREF_R = crate::BitReader<bool>;
#[doc = "Field `NOREF` writer - Initial value for the Systick timer."]
pub type NOREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSTCKCAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - System tick timer calibration value."]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - System tick timer calibration value."]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W<24> {
        SKEW_W::new(self)
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W<25> {
        NOREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick counter calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systckcal](index.html) module"]
pub struct SYSTCKCAL_SPEC;
impl crate::RegisterSpec for SYSTCKCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systckcal::R](R) reader structure"]
impl crate::Readable for SYSTCKCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systckcal::W](W) writer structure"]
impl crate::Writable for SYSTCKCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTCKCAL to value 0"]
impl crate::Resettable for SYSTCKCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
