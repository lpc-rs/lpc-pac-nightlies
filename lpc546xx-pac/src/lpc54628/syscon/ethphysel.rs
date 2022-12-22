#[doc = "Register `ETHPHYSEL` reader"]
pub struct R(crate::R<ETHPHYSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETHPHYSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETHPHYSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETHPHYSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETHPHYSEL` writer"]
pub struct W(crate::W<ETHPHYSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETHPHYSEL_SPEC>;
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
impl From<crate::W<ETHPHYSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETHPHYSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_SEL` reader - PHY interface select."]
pub type PHY_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PHY_SEL` writer - PHY interface select."]
pub type PHY_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETHPHYSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - PHY interface select."]
    #[inline(always)]
    pub fn phy_sel(&self) -> PHY_SEL_R {
        PHY_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PHY interface select."]
    #[inline(always)]
    pub fn phy_sel(&mut self) -> PHY_SEL_W<2> {
        PHY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ethphysel](index.html) module"]
pub struct ETHPHYSEL_SPEC;
impl crate::RegisterSpec for ETHPHYSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ethphysel::R](R) reader structure"]
impl crate::Readable for ETHPHYSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ethphysel::W](W) writer structure"]
impl crate::Writable for ETHPHYSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETHPHYSEL to value 0"]
impl crate::Resettable for ETHPHYSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
