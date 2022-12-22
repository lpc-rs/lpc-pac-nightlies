#[doc = "Register `STATICWAITRD` reader"]
pub struct R(crate::R<STATICWAITRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICWAITRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICWAITRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICWAITRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICWAITRD` writer"]
pub struct W(crate::W<STATICWAITRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICWAITRD_SPEC>;
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
impl From<crate::W<STATICWAITRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICWAITRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITRD` reader - ."]
pub type WAITRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAITRD` writer - ."]
pub type WAITRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATICWAITRD_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn waitrd(&self) -> WAITRD_R {
        WAITRD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ."]
    #[inline(always)]
    pub fn waitrd(&mut self) -> WAITRD_W<0> {
        WAITRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay from EMC_CSx to a read access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitrd](index.html) module"]
pub struct STATICWAITRD_SPEC;
impl crate::RegisterSpec for STATICWAITRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticwaitrd::R](R) reader structure"]
impl crate::Readable for STATICWAITRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticwaitrd::W](W) writer structure"]
impl crate::Writable for STATICWAITRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICWAITRD to value 0x1f"]
impl crate::Resettable for STATICWAITRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
