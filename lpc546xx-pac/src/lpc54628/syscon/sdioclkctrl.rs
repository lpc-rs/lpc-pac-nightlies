#[doc = "Register `SDIOCLKCTRL` reader"]
pub struct R(crate::R<SDIOCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIOCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIOCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIOCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIOCLKCTRL` writer"]
pub struct W(crate::W<SDIOCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIOCLKCTRL_SPEC>;
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
impl From<crate::W<SDIOCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIOCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLK_DRV_PHASE` reader - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
pub type CCLK_DRV_PHASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLK_DRV_PHASE` writer - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
pub type CCLK_DRV_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDIOCLKCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CCLK_SAMPLE_PHASE` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_PHASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLK_SAMPLE_PHASE` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDIOCLKCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PHASE_ACTIVE` reader - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv."]
pub type PHASE_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `PHASE_ACTIVE` writer - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv."]
pub type PHASE_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIOCLKCTRL_SPEC, bool, O>;
#[doc = "Field `CCLK_DRV_DELAY` reader - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
pub type CCLK_DRV_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLK_DRV_DELAY` writer - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
pub type CCLK_DRV_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDIOCLKCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CCLK_DRV_DELAY_ACTIVE` reader - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
pub type CCLK_DRV_DELAY_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `CCLK_DRV_DELAY_ACTIVE` writer - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
pub type CCLK_DRV_DELAY_ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIOCLKCTRL_SPEC, bool, O>;
#[doc = "Field `CCLK_SAMPLE_DELAY` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLK_SAMPLE_DELAY` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDIOCLKCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CCLK_SAMPLE_DELAY_ACTIVE` reader - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
pub type CCLK_SAMPLE_DELAY_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `CCLK_SAMPLE_DELAY_ACTIVE` writer - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
pub type CCLK_SAMPLE_DELAY_ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIOCLKCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&self) -> CCLK_DRV_PHASE_R {
        CCLK_DRV_PHASE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&self) -> CCLK_SAMPLE_PHASE_R {
        CCLK_SAMPLE_PHASE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv."]
    #[inline(always)]
    pub fn phase_active(&self) -> PHASE_ACTIVE_R {
        PHASE_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&self) -> CCLK_DRV_DELAY_R {
        CCLK_DRV_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&self) -> CCLK_DRV_DELAY_ACTIVE_R {
        CCLK_DRV_DELAY_ACTIVE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&self) -> CCLK_SAMPLE_DELAY_R {
        CCLK_SAMPLE_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&self) -> CCLK_SAMPLE_DELAY_ACTIVE_R {
        CCLK_SAMPLE_DELAY_ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&mut self) -> CCLK_DRV_PHASE_W<0> {
        CCLK_DRV_PHASE_W::new(self)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&mut self) -> CCLK_SAMPLE_PHASE_W<2> {
        CCLK_SAMPLE_PHASE_W::new(self)
    }
    #[doc = "Bit 7 - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv."]
    #[inline(always)]
    pub fn phase_active(&mut self) -> PHASE_ACTIVE_W<7> {
        PHASE_ACTIVE_W::new(self)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&mut self) -> CCLK_DRV_DELAY_W<16> {
        CCLK_DRV_DELAY_W::new(self)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&mut self) -> CCLK_DRV_DELAY_ACTIVE_W<23> {
        CCLK_DRV_DELAY_ACTIVE_W::new(self)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&mut self) -> CCLK_SAMPLE_DELAY_W<24> {
        CCLK_SAMPLE_DELAY_W::new(self)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&mut self) -> CCLK_SAMPLE_DELAY_ACTIVE_W<31> {
        CCLK_SAMPLE_DELAY_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO CCLKIN phase and delay control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioclkctrl](index.html) module"]
pub struct SDIOCLKCTRL_SPEC;
impl crate::RegisterSpec for SDIOCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdioclkctrl::R](R) reader structure"]
impl crate::Readable for SDIOCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdioclkctrl::W](W) writer structure"]
impl crate::Writable for SDIOCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIOCLKCTRL to value 0"]
impl crate::Resettable for SDIOCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
