#[doc = "Register `DATA32` writer"]
pub struct W(crate::W<DATA32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA32_SPEC>;
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
impl From<crate::W<DATA32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA32` writer - Data register bits"]
pub type DATA32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA32_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Data register bits"]
    #[inline(always)]
    pub fn data32(&mut self) -> DATA32_W<0> {
        DATA32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register - word sized\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data32](index.html) module"]
pub struct DATA32_SPEC;
impl crate::RegisterSpec for DATA32_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data32::W](W) writer structure"]
impl crate::Writable for DATA32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA32 to value 0"]
impl crate::Resettable for DATA32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
