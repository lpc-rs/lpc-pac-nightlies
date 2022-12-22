#[doc = "Register `MCLKIO` reader"]
pub struct R(crate::R<MCLKIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKIO` writer"]
pub struct W(crate::W<MCLKIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKIO_SPEC>;
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
impl From<crate::W<MCLKIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - MCLK direction control."]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - MCLK direction control."]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCLKIO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MCLK direction control."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK direction control."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK input/output control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkio](index.html) module"]
pub struct MCLKIO_SPEC;
impl crate::RegisterSpec for MCLKIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkio::R](R) reader structure"]
impl crate::Readable for MCLKIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkio::W](W) writer structure"]
impl crate::Writable for MCLKIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCLKIO to value 0"]
impl crate::Resettable for MCLKIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
