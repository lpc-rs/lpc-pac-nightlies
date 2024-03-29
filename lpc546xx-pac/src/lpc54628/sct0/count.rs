#[doc = "Register `COUNT` reader"]
pub struct R(crate::R<COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT` writer"]
pub struct W(crate::W<COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT_SPEC>;
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
impl From<crate::W<COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR_L` reader - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
pub type CTR_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTR_L` writer - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
pub type CTR_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNT_SPEC, u16, u16, 16, O>;
#[doc = "Field `CTR_H` reader - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
pub type CTR_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTR_H` writer - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
pub type CTR_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_l(&self) -> CTR_L_R {
        CTR_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_h(&self) -> CTR_H_R {
        CTR_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_l(&mut self) -> CTR_L_W<0> {
        CTR_L_W::new(self)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_h(&mut self) -> CTR_H_W<16> {
        CTR_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count](index.html) module"]
pub struct COUNT_SPEC;
impl crate::RegisterSpec for COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count::R](R) reader structure"]
impl crate::Readable for COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count::W](W) writer structure"]
impl crate::Writable for COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
