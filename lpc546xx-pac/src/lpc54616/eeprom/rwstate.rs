#[doc = "Register `RWSTATE` reader"]
pub struct R(crate::R<RWSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWSTATE` writer"]
pub struct W(crate::W<RWSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWSTATE_SPEC>;
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
impl From<crate::W<RWSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWSTATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPHASE2` reader - Wait states 2 (minus 1 encoded)."]
pub type RPHASE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPHASE2` writer - Wait states 2 (minus 1 encoded)."]
pub type RPHASE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RWSTATE_SPEC, u8, u8, 8, O>;
#[doc = "Field `RPHASE1` reader - Wait states 1 (minus 1 encoded)."]
pub type RPHASE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPHASE1` writer - Wait states 1 (minus 1 encoded)."]
pub type RPHASE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RWSTATE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Wait states 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase2(&self) -> RPHASE2_R {
        RPHASE2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase1(&self) -> RPHASE1_R {
        RPHASE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase2(&mut self) -> RPHASE2_W<0> {
        RPHASE2_W::new(self)
    }
    #[doc = "Bits 8:15 - Wait states 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase1(&mut self) -> RPHASE1_W<8> {
        RPHASE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM read wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwstate](index.html) module"]
pub struct RWSTATE_SPEC;
impl crate::RegisterSpec for RWSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwstate::R](R) reader structure"]
impl crate::Readable for RWSTATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwstate::W](W) writer structure"]
impl crate::Writable for RWSTATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RWSTATE to value 0x0e07"]
impl crate::Resettable for RWSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e07
    }
}
