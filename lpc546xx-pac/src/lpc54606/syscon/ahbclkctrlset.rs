#[doc = "Register `AHBCLKCTRLSET[%s]` writer"]
pub struct W(crate::W<AHBCLKCTRLSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRLSET_SPEC>;
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
impl From<crate::W<AHBCLKCTRLSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRLSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_SET` writer - Writing ones to this register sets the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them."]
pub type CLK_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AHBCLKCTRLSET_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn clk_set(&mut self) -> CLK_SET_W<0> {
        CLK_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bits in AHBCLKCTRLn\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrlset](index.html) module"]
pub struct AHBCLKCTRLSET_SPEC;
impl crate::RegisterSpec for AHBCLKCTRLSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrlset::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRLSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRLSET[%s]
to value 0"]
impl crate::Resettable for AHBCLKCTRLSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
