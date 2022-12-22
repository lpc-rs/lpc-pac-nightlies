#[doc = "Register `IOCFG` reader"]
pub struct R(crate::R<IOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCFG` writer"]
pub struct W(crate::W<IOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCFG_SPEC>;
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
impl From<crate::W<IOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_BYPASS0` reader - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
pub type CLK_BYPASS0_R = crate::BitReader<bool>;
#[doc = "Field `CLK_BYPASS0` writer - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
pub type CLK_BYPASS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG_SPEC, bool, O>;
#[doc = "Field `CLK_BYPASS1` reader - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
pub type CLK_BYPASS1_R = crate::BitReader<bool>;
#[doc = "Field `CLK_BYPASS1` writer - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
pub type CLK_BYPASS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG_SPEC, bool, O>;
#[doc = "Field `STEREO_DATA0` reader - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
pub type STEREO_DATA0_R = crate::BitReader<bool>;
#[doc = "Field `STEREO_DATA0` writer - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
pub type STEREO_DATA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass0(&self) -> CLK_BYPASS0_R {
        CLK_BYPASS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass1(&self) -> CLK_BYPASS1_R {
        CLK_BYPASS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
    #[inline(always)]
    pub fn stereo_data0(&self) -> STEREO_DATA0_R {
        STEREO_DATA0_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass0(&mut self) -> CLK_BYPASS0_W<0> {
        CLK_BYPASS0_W::new(self)
    }
    #[doc = "Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass1(&mut self) -> CLK_BYPASS1_W<1> {
        CLK_BYPASS1_W::new(self)
    }
    #[doc = "Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
    #[inline(always)]
    pub fn stereo_data0(&mut self) -> STEREO_DATA0_W<2> {
        STEREO_DATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg](index.html) module"]
pub struct IOCFG_SPEC;
impl crate::RegisterSpec for IOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocfg::R](R) reader structure"]
impl crate::Readable for IOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocfg::W](W) writer structure"]
impl crate::Writable for IOCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCFG to value 0"]
impl crate::Resettable for IOCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
