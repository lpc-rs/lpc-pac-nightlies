#[doc = "Register `AUTOCGOR` reader"]
pub struct R(crate::R<AUTOCGOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCGOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCGOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCGOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCGOR` writer"]
pub struct W(crate::W<AUTOCGOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCGOR_SPEC>;
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
impl From<crate::W<AUTOCGOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCGOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM0X` reader - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
pub type RAM0X_R = crate::BitReader<bool>;
#[doc = "Field `RAM0X` writer - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
pub type RAM0X_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCGOR_SPEC, bool, O>;
#[doc = "Field `RAM1` reader - When 1, automatic clock gating for RAM1 are turned off."]
pub type RAM1_R = crate::BitReader<bool>;
#[doc = "Field `RAM1` writer - When 1, automatic clock gating for RAM1 are turned off."]
pub type RAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCGOR_SPEC, bool, O>;
#[doc = "Field `RAM2` reader - When 1, automatic clock gating for RAM1 are turned off."]
pub type RAM2_R = crate::BitReader<bool>;
#[doc = "Field `RAM2` writer - When 1, automatic clock gating for RAM1 are turned off."]
pub type RAM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCGOR_SPEC, bool, O>;
#[doc = "Field `RAM3` reader - When 1, automatic clock gating for RAM1 are turned off."]
pub type RAM3_R = crate::BitReader<bool>;
#[doc = "Field `RAM3` writer - When 1, automatic clock gating for RAM1 are turned off."]
pub type RAM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCGOR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&self) -> RAM0X_R {
        RAM0X_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&mut self) -> RAM0X_W<1> {
        RAM0X_W::new(self)
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&mut self) -> RAM1_W<2> {
        RAM1_W::new(self)
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&mut self) -> RAM2_W<3> {
        RAM2_W::new(self)
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&mut self) -> RAM3_W<4> {
        RAM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto Clock-Gate Override Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocgor](index.html) module"]
pub struct AUTOCGOR_SPEC;
impl crate::RegisterSpec for AUTOCGOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocgor::R](R) reader structure"]
impl crate::Readable for AUTOCGOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocgor::W](W) writer structure"]
impl crate::Writable for AUTOCGOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOCGOR to value 0"]
impl crate::Resettable for AUTOCGOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
