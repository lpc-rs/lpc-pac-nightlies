#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITING` reader - Writing a 1 clears the interrupt enabled by the INTENSET register."]
pub type WAITING_R = crate::BitReader<bool>;
#[doc = "Field `WAITING` writer - Writing a 1 clears the interrupt enabled by the INTENSET register."]
pub type WAITING_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `DIGEST` reader - Writing a 1 clears the interrupt enabled by the INTENSET register."]
pub type DIGEST_R = crate::BitReader<bool>;
#[doc = "Field `DIGEST` writer - Writing a 1 clears the interrupt enabled by the INTENSET register."]
pub type DIGEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Writing a 1 clears the interrupt enabled by the INTENSET register."]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Writing a 1 clears the interrupt enabled by the INTENSET register."]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Writing a 1 clears the interrupt enabled by the INTENSET register."]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing a 1 clears the interrupt enabled by the INTENSET register."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing a 1 clears the interrupt enabled by the INTENSET register."]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 clears the interrupt enabled by the INTENSET register."]
    #[inline(always)]
    pub fn waiting(&mut self) -> WAITING_W<0> {
        WAITING_W::new(self)
    }
    #[doc = "Bit 1 - Writing a 1 clears the interrupt enabled by the INTENSET register."]
    #[inline(always)]
    pub fn digest(&mut self) -> DIGEST_W<1> {
        DIGEST_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 clears the interrupt enabled by the INTENSET register."]
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
#[doc = "Interrupt Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
