#[doc = "Register `INT_PTD_SKIP_MAP` reader"]
pub struct R(crate::R<INT_PTD_SKIP_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_PTD_SKIP_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_PTD_SKIP_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_PTD_SKIP_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_PTD_SKIP_MAP` writer"]
pub struct W(crate::W<INT_PTD_SKIP_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_PTD_SKIP_MAP_SPEC>;
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
impl From<crate::W<INT_PTD_SKIP_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_PTD_SKIP_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_SKIP` reader - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type INT_SKIP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INT_SKIP` writer - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type INT_SKIP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_PTD_SKIP_MAP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&self) -> INT_SKIP_R {
        INT_SKIP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&mut self) -> INT_SKIP_W<0> {
        INT_SKIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Skip map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ptd_skip_map](index.html) module"]
pub struct INT_PTD_SKIP_MAP_SPEC;
impl crate::RegisterSpec for INT_PTD_SKIP_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ptd_skip_map::R](R) reader structure"]
impl crate::Readable for INT_PTD_SKIP_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ptd_skip_map::W](W) writer structure"]
impl crate::Writable for INT_PTD_SKIP_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_PTD_SKIP_MAP to value 0"]
impl crate::Resettable for INT_PTD_SKIP_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
