#[doc = "Register `PCFG2` reader"]
pub struct R(crate::R<PCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG2` writer"]
pub struct W(crate::W<PCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG2_SPEC>;
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
impl From<crate::W<PCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POSITION` reader - Data Position."]
pub type POSITION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POSITION` writer - Data Position."]
pub type POSITION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCFG2_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 16:24 - Data Position."]
    #[inline(always)]
    pub fn position(&self) -> POSITION_R {
        POSITION_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:24 - Data Position."]
    #[inline(always)]
    pub fn position(&mut self) -> POSITION_W<16> {
        POSITION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 2 for channel pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg2](index.html) module"]
pub struct PCFG2_SPEC;
impl crate::RegisterSpec for PCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfg2::R](R) reader structure"]
impl crate::Readable for PCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg2::W](W) writer structure"]
impl crate::Writable for PCFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFG2 to value 0"]
impl crate::Resettable for PCFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
