#[doc = "Register `EMCSYSCTRL` reader"]
pub struct R(crate::R<EMCSYSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCSYSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCSYSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCSYSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMCSYSCTRL` writer"]
pub struct W(crate::W<EMCSYSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCSYSCTRL_SPEC>;
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
impl From<crate::W<EMCSYSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCSYSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMCSC` reader - EMC Shift Control."]
pub type EMCSC_R = crate::BitReader<bool>;
#[doc = "Field `EMCSC` writer - EMC Shift Control."]
pub type EMCSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMCSYSCTRL_SPEC, bool, O>;
#[doc = "Field `EMCRD` reader - EMC Reset Disable."]
pub type EMCRD_R = crate::BitReader<bool>;
#[doc = "Field `EMCRD` writer - EMC Reset Disable."]
pub type EMCRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMCSYSCTRL_SPEC, bool, O>;
#[doc = "Field `EMCBC` reader - External Memory Controller burst control."]
pub type EMCBC_R = crate::BitReader<bool>;
#[doc = "Field `EMCBC` writer - External Memory Controller burst control."]
pub type EMCBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMCSYSCTRL_SPEC, bool, O>;
#[doc = "Field `EMCFBCLKINSEL` reader - External Memory Controller clock select."]
pub type EMCFBCLKINSEL_R = crate::BitReader<bool>;
#[doc = "Field `EMCFBCLKINSEL` writer - External Memory Controller clock select."]
pub type EMCFBCLKINSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMCSYSCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&self) -> EMCSC_R {
        EMCSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&self) -> EMCRD_R {
        EMCRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&self) -> EMCBC_R {
        EMCBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&self) -> EMCFBCLKINSEL_R {
        EMCFBCLKINSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&mut self) -> EMCSC_W<0> {
        EMCSC_W::new(self)
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&mut self) -> EMCRD_W<1> {
        EMCRD_W::new(self)
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&mut self) -> EMCBC_W<2> {
        EMCBC_W::new(self)
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&mut self) -> EMCFBCLKINSEL_W<3> {
        EMCFBCLKINSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMC system control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcsysctrl](index.html) module"]
pub struct EMCSYSCTRL_SPEC;
impl crate::RegisterSpec for EMCSYSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emcsysctrl::R](R) reader structure"]
impl crate::Readable for EMCSYSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emcsysctrl::W](W) writer structure"]
impl crate::Writable for EMCSYSCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMCSYSCTRL to value 0x01"]
impl crate::Resettable for EMCSYSCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
