#[doc = "Register `SYSPLLSTAT` reader"]
pub struct R(crate::R<SYSPLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLSTAT` writer"]
pub struct W(crate::W<SYSPLLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLSTAT_SPEC>;
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
impl From<crate::W<SYSPLLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - PLL lock indicator."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - PLL lock indicator."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL lock indicator."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL lock indicator."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllstat](index.html) module"]
pub struct SYSPLLSTAT_SPEC;
impl crate::RegisterSpec for SYSPLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllstat::R](R) reader structure"]
impl crate::Readable for SYSPLLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllstat::W](W) writer structure"]
impl crate::Writable for SYSPLLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPLLSTAT to value 0"]
impl crate::Resettable for SYSPLLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
