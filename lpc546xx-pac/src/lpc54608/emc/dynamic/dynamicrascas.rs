#[doc = "Register `DYNAMICRASCAS` reader"]
pub struct R(crate::R<DYNAMICRASCAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRASCAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRASCAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRASCAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICRASCAS` writer"]
pub struct W(crate::W<DYNAMICRASCAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRASCAS_SPEC>;
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
impl From<crate::W<DYNAMICRASCAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRASCAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAS` reader - RAS latency (active to read/write delay)."]
pub type RAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAS` writer - RAS latency (active to read/write delay)."]
pub type RAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICRASCAS_SPEC, u8, u8, 2, O>;
#[doc = "Field `CAS` reader - CAS latency."]
pub type CAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAS` writer - CAS latency."]
pub type CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICRASCAS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&mut self) -> RAS_W<0> {
        RAS_W::new(self)
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<8> {
        CAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAS and CAS latencies for EMC_DYCSx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrascas](index.html) module"]
pub struct DYNAMICRASCAS_SPEC;
impl crate::RegisterSpec for DYNAMICRASCAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicrascas::R](R) reader structure"]
impl crate::Readable for DYNAMICRASCAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicrascas::W](W) writer structure"]
impl crate::Writable for DYNAMICRASCAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICRASCAS to value 0x0303"]
impl crate::Resettable for DYNAMICRASCAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0303
    }
}
