#[doc = "Register `EMCDLYCTRL` reader"]
pub struct R(crate::R<EMCDLYCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCDLYCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCDLYCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCDLYCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMCDLYCTRL` writer"]
pub struct W(crate::W<EMCDLYCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCDLYCTRL_SPEC>;
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
impl From<crate::W<EMCDLYCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCDLYCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_DELAY` reader - Programmable delay value for EMC outputs in command delayed mode."]
pub type CMD_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_DELAY` writer - Programmable delay value for EMC outputs in command delayed mode."]
pub type CMD_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMCDLYCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `FBCLK_DELAY` reader - Programmable delay value for the feedback clock that controls input data sampling."]
pub type FBCLK_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FBCLK_DELAY` writer - Programmable delay value for the feedback clock that controls input data sampling."]
pub type FBCLK_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMCDLYCTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode."]
    #[inline(always)]
    pub fn cmd_delay(&self) -> CMD_DELAY_R {
        CMD_DELAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling."]
    #[inline(always)]
    pub fn fbclk_delay(&self) -> FBCLK_DELAY_R {
        FBCLK_DELAY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode."]
    #[inline(always)]
    pub fn cmd_delay(&mut self) -> CMD_DELAY_W<0> {
        CMD_DELAY_W::new(self)
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling."]
    #[inline(always)]
    pub fn fbclk_delay(&mut self) -> FBCLK_DELAY_W<8> {
        FBCLK_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMC clock delay control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcdlyctrl](index.html) module"]
pub struct EMCDLYCTRL_SPEC;
impl crate::RegisterSpec for EMCDLYCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emcdlyctrl::R](R) reader structure"]
impl crate::Readable for EMCDLYCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emcdlyctrl::W](W) writer structure"]
impl crate::Writable for EMCDLYCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMCDLYCTRL to value 0x0210"]
impl crate::Resettable for EMCDLYCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210
    }
}
