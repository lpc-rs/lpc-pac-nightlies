#[doc = "Register `SYSPLLPDEC` reader"]
pub struct R(crate::R<SYSPLLPDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLPDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLPDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLPDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLPDEC` writer"]
pub struct W(crate::W<SYSPLLPDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLPDEC_SPEC>;
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
impl From<crate::W<SYSPLLPDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLPDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDEC` reader - Decoded P-divider coefficient value."]
pub type PDEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDEC` writer - Decoded P-divider coefficient value."]
pub type PDEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLLPDEC_SPEC, u8, u8, 7, O>;
#[doc = "Field `PREQ` reader - ."]
pub type PREQ_R = crate::BitReader<bool>;
#[doc = "Field `PREQ` writer - ."]
pub type PREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLLPDEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Decoded P-divider coefficient value."]
    #[inline(always)]
    pub fn pdec(&self) -> PDEC_R {
        PDEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - ."]
    #[inline(always)]
    pub fn preq(&self) -> PREQ_R {
        PREQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Decoded P-divider coefficient value."]
    #[inline(always)]
    pub fn pdec(&mut self) -> PDEC_W<0> {
        PDEC_W::new(self)
    }
    #[doc = "Bit 7 - ."]
    #[inline(always)]
    pub fn preq(&mut self) -> PREQ_W<7> {
        PREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllpdec](index.html) module"]
pub struct SYSPLLPDEC_SPEC;
impl crate::RegisterSpec for SYSPLLPDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllpdec::R](R) reader structure"]
impl crate::Readable for SYSPLLPDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllpdec::W](W) writer structure"]
impl crate::Writable for SYSPLLPDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPLLPDEC to value 0"]
impl crate::Resettable for SYSPLLPDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
