#[doc = "Register `BACKENDPWR` reader"]
pub struct R(crate::R<BACKENDPWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKENDPWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKENDPWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKENDPWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKENDPWR` writer"]
pub struct W(crate::W<BACKENDPWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKENDPWR_SPEC>;
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
impl From<crate::W<BACKENDPWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKENDPWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKENDPWR` reader - Back-end Power control for card application."]
pub type BACKENDPWR_R = crate::BitReader<bool>;
#[doc = "Field `BACKENDPWR` writer - Back-end Power control for card application."]
pub type BACKENDPWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BACKENDPWR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Back-end Power control for card application."]
    #[inline(always)]
    pub fn backendpwr(&self) -> BACKENDPWR_R {
        BACKENDPWR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Back-end Power control for card application."]
    #[inline(always)]
    pub fn backendpwr(&mut self) -> BACKENDPWR_W<0> {
        BACKENDPWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backendpwr](index.html) module"]
pub struct BACKENDPWR_SPEC;
impl crate::RegisterSpec for BACKENDPWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backendpwr::R](R) reader structure"]
impl crate::Readable for BACKENDPWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backendpwr::W](W) writer structure"]
impl crate::Writable for BACKENDPWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKENDPWR to value 0"]
impl crate::Resettable for BACKENDPWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
