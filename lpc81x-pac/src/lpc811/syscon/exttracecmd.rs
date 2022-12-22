#[doc = "Register `EXTTRACECMD` reader"]
pub struct R(crate::R<EXTTRACECMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTTRACECMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTTRACECMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTTRACECMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTTRACECMD` writer"]
pub struct W(crate::W<EXTTRACECMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTTRACECMD_SPEC>;
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
impl From<crate::W<EXTTRACECMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTTRACECMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Trace start command. Writing a one to this bit sets the TSTART signal to the MTB to HIGH and starts tracing if the TSTARTEN bit in the MTB master register is set to one as well."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Trace start command. Writing a one to this bit sets the TSTART signal to the MTB to HIGH and starts tracing if the TSTARTEN bit in the MTB master register is set to one as well."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRACECMD_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Trace stop command. Writing a one to this bit sets the TSTOP signal in the MTB to HIGH and stops tracing if the TSTOPEN bit in the MTB master register is set to one as well."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Trace stop command. Writing a one to this bit sets the TSTOP signal in the MTB to HIGH and stops tracing if the TSTOPEN bit in the MTB master register is set to one as well."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRACECMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Trace start command. Writing a one to this bit sets the TSTART signal to the MTB to HIGH and starts tracing if the TSTARTEN bit in the MTB master register is set to one as well."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trace stop command. Writing a one to this bit sets the TSTOP signal in the MTB to HIGH and stops tracing if the TSTOPEN bit in the MTB master register is set to one as well."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trace start command. Writing a one to this bit sets the TSTART signal to the MTB to HIGH and starts tracing if the TSTARTEN bit in the MTB master register is set to one as well."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Trace stop command. Writing a one to this bit sets the TSTOP signal in the MTB to HIGH and stops tracing if the TSTOPEN bit in the MTB master register is set to one as well."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External trace buffer command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exttracecmd](index.html) module"]
pub struct EXTTRACECMD_SPEC;
impl crate::RegisterSpec for EXTTRACECMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exttracecmd::R](R) reader structure"]
impl crate::Readable for EXTTRACECMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exttracecmd::W](W) writer structure"]
impl crate::Writable for EXTTRACECMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTTRACECMD to value 0"]
impl crate::Resettable for EXTTRACECMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
