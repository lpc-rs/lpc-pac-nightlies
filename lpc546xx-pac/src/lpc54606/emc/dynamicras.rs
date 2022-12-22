#[doc = "Register `DYNAMICRAS` reader"]
pub struct R(crate::R<DYNAMICRAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICRAS` writer"]
pub struct W(crate::W<DYNAMICRAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRAS_SPEC>;
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
impl From<crate::W<DYNAMICRAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRAS` reader - Active to precharge command period."]
pub type TRAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRAS` writer - Active to precharge command period."]
pub type TRAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICRAS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Active to precharge command period."]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active to precharge command period."]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W<0> {
        TRAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Active to precharge command period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicras](index.html) module"]
pub struct DYNAMICRAS_SPEC;
impl crate::RegisterSpec for DYNAMICRAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicras::R](R) reader structure"]
impl crate::Readable for DYNAMICRAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicras::W](W) writer structure"]
impl crate::Writable for DYNAMICRAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICRAS to value 0x0f"]
impl crate::Resettable for DYNAMICRAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
