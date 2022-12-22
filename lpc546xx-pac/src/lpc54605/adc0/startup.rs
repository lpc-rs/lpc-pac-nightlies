#[doc = "Register `STARTUP` reader"]
pub struct R(crate::R<STARTUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTUP` writer"]
pub struct W(crate::W<STARTUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTUP_SPEC>;
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
impl From<crate::W<STARTUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_ENA` reader - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
pub type ADC_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ENA` writer - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
pub type ADC_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTUP_SPEC, bool, O>;
#[doc = "Field `ADC_INIT` reader - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
pub type ADC_INIT_R = crate::BitReader<bool>;
#[doc = "Field `ADC_INIT` writer - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
pub type ADC_INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTUP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
    #[inline(always)]
    pub fn adc_ena(&self) -> ADC_ENA_R {
        ADC_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
    #[inline(always)]
    pub fn adc_init(&self) -> ADC_INIT_R {
        ADC_INIT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
    #[inline(always)]
    pub fn adc_ena(&mut self) -> ADC_ENA_W<0> {
        ADC_ENA_W::new(self)
    }
    #[doc = "Bit 1 - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
    #[inline(always)]
    pub fn adc_init(&mut self) -> ADC_INIT_W<1> {
        ADC_INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Startup register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startup](index.html) module"]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startup::R](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startup::W](W) writer structure"]
impl crate::Writable for STARTUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTUP to value 0"]
impl crate::Resettable for STARTUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
