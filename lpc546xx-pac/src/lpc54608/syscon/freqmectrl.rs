#[doc = "Register `FREQMECTRL` reader"]
pub struct R(crate::R<FREQMECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQMECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQMECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQMECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQMECTRL` writer"]
pub struct W(crate::W<FREQMECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQMECTRL_SPEC>;
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
impl From<crate::W<FREQMECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQMECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPVAL` reader - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only."]
pub type CAPVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPVAL` writer - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only."]
pub type CAPVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQMECTRL_SPEC, u16, u16, 14, O>;
#[doc = "Field `PROG` reader - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0)."]
pub type PROG_R = crate::BitReader<bool>;
#[doc = "Field `PROG` writer - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0)."]
pub type PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FREQMECTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only."]
    #[inline(always)]
    pub fn capval(&self) -> CAPVAL_R {
        CAPVAL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0)."]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only."]
    #[inline(always)]
    pub fn capval(&mut self) -> CAPVAL_W<0> {
        CAPVAL_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0)."]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W<31> {
        PROG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency measure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmectrl](index.html) module"]
pub struct FREQMECTRL_SPEC;
impl crate::RegisterSpec for FREQMECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqmectrl::R](R) reader structure"]
impl crate::Readable for FREQMECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqmectrl::W](W) writer structure"]
impl crate::Writable for FREQMECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQMECTRL to value 0"]
impl crate::Resettable for FREQMECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
