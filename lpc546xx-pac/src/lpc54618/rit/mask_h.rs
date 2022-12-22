#[doc = "Register `MASK_H` reader"]
pub struct R(crate::R<MASK_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK_H` writer"]
pub struct W(crate::W<MASK_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_H_SPEC>;
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
impl From<crate::W<MASK_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIMASK` reader - Mask register."]
pub type RIMASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RIMASK` writer - Mask register."]
pub type RIMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK_H_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Mask register."]
    #[inline(always)]
    pub fn rimask(&self) -> RIMASK_R {
        RIMASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask register."]
    #[inline(always)]
    pub fn rimask(&mut self) -> RIMASK_W<0> {
        RIMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_h](index.html) module"]
pub struct MASK_H_SPEC;
impl crate::RegisterSpec for MASK_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask_h::R](R) reader structure"]
impl crate::Readable for MASK_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask_h::W](W) writer structure"]
impl crate::Writable for MASK_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK_H to value 0"]
impl crate::Resettable for MASK_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
