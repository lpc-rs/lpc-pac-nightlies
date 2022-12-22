#[doc = "Register `ITCTRL` reader"]
pub struct R(crate::R<ITCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITCTRL` writer"]
pub struct W(crate::W<ITCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITCTRL_SPEC>;
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
impl From<crate::W<ITCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Mode` reader - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0."]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `Mode` writer - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itctrl](index.html) module"]
pub struct ITCTRL_SPEC;
impl crate::RegisterSpec for ITCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itctrl::R](R) reader structure"]
impl crate::Readable for ITCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itctrl::W](W) writer structure"]
impl crate::Writable for ITCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITCTRL to value 0"]
impl crate::Resettable for ITCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
