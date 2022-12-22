#[doc = "Register `DYNAMICCONTROL` reader"]
pub struct R(crate::R<DYNAMICCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICCONTROL` writer"]
pub struct W(crate::W<DYNAMICCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICCONTROL_SPEC>;
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
impl From<crate::W<DYNAMICCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CE` reader - Dynamic memory clock enable."]
pub type CE_R = crate::BitReader<bool>;
#[doc = "Field `CE` writer - Dynamic memory clock enable."]
pub type CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYNAMICCONTROL_SPEC, bool, O>;
#[doc = "Field `CS` reader - Dynamic memory clock control."]
pub type CS_R = crate::BitReader<bool>;
#[doc = "Field `CS` writer - Dynamic memory clock control."]
pub type CS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYNAMICCONTROL_SPEC, bool, O>;
#[doc = "Field `SR` reader - Self-refresh request, EMCSREFREQ."]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Self-refresh request, EMCSREFREQ."]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYNAMICCONTROL_SPEC, bool, O>;
#[doc = "Field `MMC` reader - Memory clock control."]
pub type MMC_R = crate::BitReader<bool>;
#[doc = "Field `MMC` writer - Memory clock control."]
pub type MMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DYNAMICCONTROL_SPEC, bool, O>;
#[doc = "Field `I` reader - SDRAM initialization."]
pub type I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I` writer - SDRAM initialization."]
pub type I_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICCONTROL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dynamic memory clock control."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ."]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W<0> {
        CE_W::new(self)
    }
    #[doc = "Bit 1 - Dynamic memory clock control."]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<1> {
        CS_W::new(self)
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ."]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<2> {
        SR_W::new(self)
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W<5> {
        MMC_W::new(self)
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&mut self) -> I_W<7> {
        I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls dynamic memory operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamiccontrol](index.html) module"]
pub struct DYNAMICCONTROL_SPEC;
impl crate::RegisterSpec for DYNAMICCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamiccontrol::R](R) reader structure"]
impl crate::Readable for DYNAMICCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamiccontrol::W](W) writer structure"]
impl crate::Writable for DYNAMICCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICCONTROL to value 0x06"]
impl crate::Resettable for DYNAMICCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
