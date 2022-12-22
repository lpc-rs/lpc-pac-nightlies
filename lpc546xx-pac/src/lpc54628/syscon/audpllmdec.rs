#[doc = "Register `AUDPLLMDEC` reader"]
pub struct R(crate::R<AUDPLLMDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDPLLMDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDPLLMDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDPLLMDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDPLLMDEC` writer"]
pub struct W(crate::W<AUDPLLMDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDPLLMDEC_SPEC>;
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
impl From<crate::W<AUDPLLMDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDPLLMDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDEC` reader - Decoded M-divider coefficient value."]
pub type MDEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MDEC` writer - Decoded M-divider coefficient value."]
pub type MDEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUDPLLMDEC_SPEC, u32, u32, 17, O>;
#[doc = "Field `MREQ` reader - MDEC reload request."]
pub type MREQ_R = crate::BitReader<bool>;
#[doc = "Field `MREQ` writer - MDEC reload request."]
pub type MREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUDPLLMDEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:16 - Decoded M-divider coefficient value."]
    #[inline(always)]
    pub fn mdec(&self) -> MDEC_R {
        MDEC_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - MDEC reload request."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Decoded M-divider coefficient value."]
    #[inline(always)]
    pub fn mdec(&mut self) -> MDEC_W<0> {
        MDEC_W::new(self)
    }
    #[doc = "Bit 17 - MDEC reload request."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MREQ_W<17> {
        MREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL M divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllmdec](index.html) module"]
pub struct AUDPLLMDEC_SPEC;
impl crate::RegisterSpec for AUDPLLMDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audpllmdec::R](R) reader structure"]
impl crate::Readable for AUDPLLMDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audpllmdec::W](W) writer structure"]
impl crate::Writable for AUDPLLMDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDPLLMDEC to value 0"]
impl crate::Resettable for AUDPLLMDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
