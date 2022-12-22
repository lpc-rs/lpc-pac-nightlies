#[doc = "Register `STATICWAITWR` reader"]
pub struct R(crate::R<STATICWAITWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICWAITWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICWAITWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICWAITWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICWAITWR` writer"]
pub struct W(crate::W<STATICWAITWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICWAITWR_SPEC>;
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
impl From<crate::W<STATICWAITWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICWAITWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITWR` reader - Write wait states."]
pub type WAITWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAITWR` writer - Write wait states."]
pub type WAITWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATICWAITWR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Write wait states."]
    #[inline(always)]
    pub fn waitwr(&self) -> WAITWR_R {
        WAITWR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write wait states."]
    #[inline(always)]
    pub fn waitwr(&mut self) -> WAITWR_W<0> {
        WAITWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay from EMC_CSx to a write access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitwr](index.html) module"]
pub struct STATICWAITWR_SPEC;
impl crate::RegisterSpec for STATICWAITWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticwaitwr::R](R) reader structure"]
impl crate::Readable for STATICWAITWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticwaitwr::W](W) writer structure"]
impl crate::Writable for STATICWAITWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICWAITWR to value 0x1f"]
impl crate::Resettable for STATICWAITWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
