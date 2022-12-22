#[doc = "Register `USB0CLKSTAT` reader"]
pub struct R(crate::R<USB0CLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0CLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0CLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0CLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB0CLKSTAT` writer"]
pub struct W(crate::W<USB0CLKSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0CLKSTAT_SPEC>;
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
impl From<crate::W<USB0CLKSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0CLKSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_NEED_CLKST` reader - USB0 Device USB0_NEEDCLK signal status."]
pub type DEV_NEED_CLKST_R = crate::BitReader<bool>;
#[doc = "Field `DEV_NEED_CLKST` writer - USB0 Device USB0_NEEDCLK signal status."]
pub type DEV_NEED_CLKST_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB0CLKSTAT_SPEC, bool, O>;
#[doc = "Field `HOST_NEED_CLKST` reader - USB0 Host USB0_NEEDCLK signal status."]
pub type HOST_NEED_CLKST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_NEED_CLKST` writer - USB0 Host USB0_NEEDCLK signal status."]
pub type HOST_NEED_CLKST_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB0CLKSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn dev_need_clkst(&self) -> DEV_NEED_CLKST_R {
        DEV_NEED_CLKST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB0 Host USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn host_need_clkst(&self) -> HOST_NEED_CLKST_R {
        HOST_NEED_CLKST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn dev_need_clkst(&mut self) -> DEV_NEED_CLKST_W<0> {
        DEV_NEED_CLKST_W::new(self)
    }
    #[doc = "Bit 1 - USB0 Host USB0_NEEDCLK signal status."]
    #[inline(always)]
    pub fn host_need_clkst(&mut self) -> HOST_NEED_CLKST_W<1> {
        HOST_NEED_CLKST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB0 clock status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clkstat](index.html) module"]
pub struct USB0CLKSTAT_SPEC;
impl crate::RegisterSpec for USB0CLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0clkstat::R](R) reader structure"]
impl crate::Readable for USB0CLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0clkstat::W](W) writer structure"]
impl crate::Writable for USB0CLKSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB0CLKSTAT to value 0"]
impl crate::Resettable for USB0CLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
