#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITING` reader - This field indicates if interrupt should be enabled when waiting for input data."]
pub type WAITING_R = crate::BitReader<bool>;
#[doc = "Field `WAITING` writer - This field indicates if interrupt should be enabled when waiting for input data."]
pub type WAITING_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `DIGEST` reader - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence)."]
pub type DIGEST_R = crate::BitReader<bool>;
#[doc = "Field `DIGEST` writer - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence)."]
pub type DIGEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - This field indicates if interrupt is generated on an ERROR (as defined in STAT register)."]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - This field indicates if interrupt is generated on an ERROR (as defined in STAT register)."]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This field indicates if interrupt should be enabled when waiting for input data."]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence)."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field indicates if interrupt is generated on an ERROR (as defined in STAT register)."]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field indicates if interrupt should be enabled when waiting for input data."]
    #[inline(always)]
    pub fn waiting(&mut self) -> WAITING_W<0> {
        WAITING_W::new(self)
    }
    #[doc = "Bit 1 - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence)."]
    #[inline(always)]
    pub fn digest(&mut self) -> DIGEST_W<1> {
        DIGEST_W::new(self)
    }
    #[doc = "Bit 2 - This field indicates if interrupt is generated on an ERROR (as defined in STAT register)."]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W<2> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
