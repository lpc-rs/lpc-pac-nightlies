#[doc = "Register `ETSCC` reader"]
pub struct R(crate::R<ETSCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETSCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETSCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETSCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETSCC` writer"]
pub struct W(crate::W<ETSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETSCC_SPEC>;
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
impl From<crate::W<ETSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETSCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETCP` reader - External timestamp prescaler value."]
pub type ETCP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ETCP` writer - External timestamp prescaler value."]
pub type ETCP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETSCC_SPEC, u16, u16, 11, O>;
#[doc = "Field `ETCE` reader - External timestamp counter enable."]
pub type ETCE_R = crate::BitReader<bool>;
#[doc = "Field `ETCE` writer - External timestamp counter enable."]
pub type ETCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETSCC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - External timestamp prescaler value."]
    #[inline(always)]
    pub fn etcp(&self) -> ETCP_R {
        ETCP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - External timestamp counter enable."]
    #[inline(always)]
    pub fn etce(&self) -> ETCE_R {
        ETCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - External timestamp prescaler value."]
    #[inline(always)]
    pub fn etcp(&mut self) -> ETCP_W<0> {
        ETCP_W::new(self)
    }
    #[doc = "Bit 31 - External timestamp counter enable."]
    #[inline(always)]
    pub fn etce(&mut self) -> ETCE_W<31> {
        ETCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Timestamp Counter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etscc](index.html) module"]
pub struct ETSCC_SPEC;
impl crate::RegisterSpec for ETSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etscc::R](R) reader structure"]
impl crate::Readable for ETSCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etscc::W](W) writer structure"]
impl crate::Writable for ETSCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETSCC to value 0"]
impl crate::Resettable for ETSCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
