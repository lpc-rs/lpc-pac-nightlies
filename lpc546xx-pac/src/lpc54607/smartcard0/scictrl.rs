#[doc = "Register `SCICTRL` reader"]
pub struct R(crate::R<SCICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCICTRL` writer"]
pub struct W(crate::W<SCICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCICTRL_SPEC>;
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
impl From<crate::W<SCICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCIEN` reader - Smart Card Interface Enable."]
pub type SCIEN_R = crate::BitReader<bool>;
#[doc = "Field `SCIEN` writer - Smart Card Interface Enable."]
pub type SCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCICTRL_SPEC, bool, O>;
#[doc = "Field `NACKDIS` reader - NACK response disable."]
pub type NACKDIS_R = crate::BitReader<bool>;
#[doc = "Field `NACKDIS` writer - NACK response disable."]
pub type NACKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCICTRL_SPEC, bool, O>;
#[doc = "Field `PROTSEL` reader - Protocol selection as defined in the ISO7816-3 standard."]
pub type PROTSEL_R = crate::BitReader<bool>;
#[doc = "Field `PROTSEL` writer - Protocol selection as defined in the ISO7816-3 standard."]
pub type PROTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCICTRL_SPEC, bool, O>;
#[doc = "Field `TXRETRY` reader - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
pub type TXRETRY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXRETRY` writer - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
pub type TXRETRY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCICTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `GUARDTIME` reader - Extra guard time."]
pub type GUARDTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GUARDTIME` writer - Extra guard time."]
pub type GUARDTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCICTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&self) -> GUARDTIME_R {
        GUARDTIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> SCIEN_W<0> {
        SCIEN_W::new(self)
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> NACKDIS_W<1> {
        NACKDIS_W::new(self)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> PROTSEL_W<2> {
        PROTSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&mut self) -> TXRETRY_W<5> {
        TXRETRY_W::new(self)
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&mut self) -> GUARDTIME_W<8> {
        GUARDTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Smart Card Interface control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scictrl](index.html) module"]
pub struct SCICTRL_SPEC;
impl crate::RegisterSpec for SCICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scictrl::R](R) reader structure"]
impl crate::Readable for SCICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scictrl::W](W) writer structure"]
impl crate::Writable for SCICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCICTRL to value 0"]
impl crate::Resettable for SCICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
