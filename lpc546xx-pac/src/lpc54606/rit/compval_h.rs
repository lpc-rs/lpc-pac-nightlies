#[doc = "Register `COMPVAL_H` reader"]
pub struct R(crate::R<COMPVAL_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPVAL_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPVAL_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPVAL_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPVAL_H` writer"]
pub struct W(crate::W<COMPVAL_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPVAL_H_SPEC>;
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
impl From<crate::W<COMPVAL_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPVAL_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RICOMP` reader - Compare value MSB register."]
pub type RICOMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RICOMP` writer - Compare value MSB register."]
pub type RICOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMPVAL_H_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Compare value MSB register."]
    #[inline(always)]
    pub fn ricomp(&self) -> RICOMP_R {
        RICOMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare value MSB register."]
    #[inline(always)]
    pub fn ricomp(&mut self) -> RICOMP_W<0> {
        RICOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare value MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compval_h](index.html) module"]
pub struct COMPVAL_H_SPEC;
impl crate::RegisterSpec for COMPVAL_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compval_h::R](R) reader structure"]
impl crate::Readable for COMPVAL_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compval_h::W](W) writer structure"]
impl crate::Writable for COMPVAL_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPVAL_H to value 0xffff"]
impl crate::Resettable for COMPVAL_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
