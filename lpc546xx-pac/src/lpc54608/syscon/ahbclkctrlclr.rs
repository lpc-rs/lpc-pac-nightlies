#[doc = "Register `AHBCLKCTRLCLR[%s]` writer"]
pub struct W(crate::W<AHBCLKCTRLCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRLCLR_SPEC>;
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
impl From<crate::W<AHBCLKCTRLCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRLCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_CLR` writer - Writing ones to this register clears the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them."]
pub type CLK_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AHBCLKCTRLCLR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn clk_clr(&mut self) -> CLK_CLR_W<0> {
        CLK_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear bits in AHBCLKCTRLn\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrlclr](index.html) module"]
pub struct AHBCLKCTRLCLR_SPEC;
impl crate::RegisterSpec for AHBCLKCTRLCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrlclr::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRLCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRLCLR[%s]
to value 0"]
impl crate::Resettable for AHBCLKCTRLCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
