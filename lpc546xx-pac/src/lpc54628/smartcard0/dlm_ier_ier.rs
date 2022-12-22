#[doc = "Register `IER` reader"]
pub struct R(crate::R<DLM_IER_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLM_IER_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLM_IER_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLM_IER_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<DLM_IER_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLM_IER_IER_SPEC>;
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
impl From<crate::W<DLM_IER_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLM_IER_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable."]
pub type RBRIE_R = crate::BitReader<bool>;
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable."]
pub type RBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLM_IER_IER_SPEC, bool, O>;
#[doc = "Field `THREIE` reader - THRE Interrupt Enable."]
pub type THREIE_R = crate::BitReader<bool>;
#[doc = "Field `THREIE` writer - THRE Interrupt Enable."]
pub type THREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLM_IER_IER_SPEC, bool, O>;
#[doc = "Field `RXIE` reader - RX Line Status Interrupt Enable."]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - RX Line Status Interrupt Enable."]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLM_IER_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable."]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable."]
    #[inline(always)]
    pub fn threie(&self) -> THREIE_R {
        THREIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable."]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RBRIE_W<0> {
        RBRIE_W::new(self)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable."]
    #[inline(always)]
    pub fn threie(&mut self) -> THREIE_W<1> {
        THREIE_W::new(self)
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable."]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W<2> {
        RXIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlm_ier_ier](index.html) module"]
pub struct DLM_IER_IER_SPEC;
impl crate::RegisterSpec for DLM_IER_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlm_ier_ier::R](R) reader structure"]
impl crate::Readable for DLM_IER_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlm_ier_ier::W](W) writer structure"]
impl crate::Writable for DLM_IER_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for DLM_IER_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
