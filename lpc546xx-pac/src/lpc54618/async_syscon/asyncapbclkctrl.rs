#[doc = "Register `ASYNCAPBCLKCTRL` reader"]
pub struct R(crate::R<ASYNCAPBCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCAPBCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCAPBCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCAPBCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCAPBCLKCTRL` writer"]
pub struct W(crate::W<ASYNCAPBCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCAPBCLKCTRL_SPEC>;
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
impl From<crate::W<ASYNCAPBCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCAPBCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIMER3` reader - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable."]
pub type CTIMER3_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER3` writer - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable."]
pub type CTIMER3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNCAPBCLKCTRL_SPEC, bool, O>;
#[doc = "Field `CTIMER4` reader - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable."]
pub type CTIMER4_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER4` writer - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable."]
pub type CTIMER4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASYNCAPBCLKCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 13 - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> CTIMER3_W<13> {
        CTIMER3_W::new(self)
    }
    #[doc = "Bit 14 - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer4(&mut self) -> CTIMER4_W<14> {
        CTIMER4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Async peripheral clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbclkctrl](index.html) module"]
pub struct ASYNCAPBCLKCTRL_SPEC;
impl crate::RegisterSpec for ASYNCAPBCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asyncapbclkctrl::R](R) reader structure"]
impl crate::Readable for ASYNCAPBCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncapbclkctrl::W](W) writer structure"]
impl crate::Writable for ASYNCAPBCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCAPBCLKCTRL to value 0"]
impl crate::Resettable for ASYNCAPBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
