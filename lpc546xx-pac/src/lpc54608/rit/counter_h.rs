#[doc = "Register `COUNTER_H` reader"]
pub struct R(crate::R<COUNTER_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTER_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTER_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTER_H` writer"]
pub struct W(crate::W<COUNTER_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTER_H_SPEC>;
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
impl From<crate::W<COUNTER_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNTER_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RICOUNTER` reader - 16 LSBs of the up counter."]
pub type RICOUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RICOUNTER` writer - 16 LSBs of the up counter."]
pub type RICOUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COUNTER_H_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 16 LSBs of the up counter."]
    #[inline(always)]
    pub fn ricounter(&self) -> RICOUNTER_R {
        RICOUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 LSBs of the up counter."]
    #[inline(always)]
    pub fn ricounter(&mut self) -> RICOUNTER_W<0> {
        RICOUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter_h](index.html) module"]
pub struct COUNTER_H_SPEC;
impl crate::RegisterSpec for COUNTER_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter_h::R](R) reader structure"]
impl crate::Readable for COUNTER_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [counter_h::W](W) writer structure"]
impl crate::Writable for COUNTER_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNTER_H to value 0"]
impl crate::Resettable for COUNTER_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
