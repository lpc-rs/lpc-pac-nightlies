#[doc = "Register `ABORT0` writer"]
pub struct W(crate::W<ABORT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABORT0_SPEC>;
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
impl From<crate::W<ABORT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABORT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ABORTCTRL` writer - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
pub type ABORTCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABORT0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[inline(always)]
    pub fn abortctrl(&mut self) -> ABORTCTRL_W<0> {
        ABORTCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Abort control for all DMA channels.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abort0](index.html) module"]
pub struct ABORT0_SPEC;
impl crate::RegisterSpec for ABORT0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [abort0::W](W) writer structure"]
impl crate::Writable for ABORT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ABORT0 to value 0"]
impl crate::Resettable for ABORT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
