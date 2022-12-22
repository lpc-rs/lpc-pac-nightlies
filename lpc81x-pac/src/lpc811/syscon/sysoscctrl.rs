#[doc = "Register `SYSOSCCTRL` reader"]
pub struct R(crate::R<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSOSCCTRL` writer"]
pub struct W(crate::W<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSOSCCTRL_SPEC>;
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
impl From<crate::W<SYSOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Bypass system oscillator"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Bypass system oscillator"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSOSCCTRL_SPEC, bool, O>;
#[doc = "Field `FREQRANGE` reader - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
pub type FREQRANGE_R = crate::BitReader<bool>;
#[doc = "Field `FREQRANGE` writer - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
pub type FREQRANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSOSCCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 1 - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FREQRANGE_W<1> {
        FREQRANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscctrl](index.html) module"]
pub struct SYSOSCCTRL_SPEC;
impl crate::RegisterSpec for SYSOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysoscctrl::R](R) reader structure"]
impl crate::Readable for SYSOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysoscctrl::W](W) writer structure"]
impl crate::Writable for SYSOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSOSCCTRL to value 0"]
impl crate::Resettable for SYSOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
