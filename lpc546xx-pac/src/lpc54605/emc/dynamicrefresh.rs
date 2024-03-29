#[doc = "Register `DYNAMICREFRESH` reader"]
pub struct R(crate::R<DYNAMICREFRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICREFRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICREFRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICREFRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICREFRESH` writer"]
pub struct W(crate::W<DYNAMICREFRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICREFRESH_SPEC>;
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
impl From<crate::W<DYNAMICREFRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICREFRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFRESH` reader - Refresh timer."]
pub type REFRESH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REFRESH` writer - Refresh timer."]
pub type REFRESH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DYNAMICREFRESH_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Refresh timer."]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Refresh timer."]
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W<0> {
        REFRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures dynamic memory refresh\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrefresh](index.html) module"]
pub struct DYNAMICREFRESH_SPEC;
impl crate::RegisterSpec for DYNAMICREFRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicrefresh::R](R) reader structure"]
impl crate::Readable for DYNAMICREFRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicrefresh::W](W) writer structure"]
impl crate::Writable for DYNAMICREFRESH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICREFRESH to value 0"]
impl crate::Resettable for DYNAMICREFRESH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
