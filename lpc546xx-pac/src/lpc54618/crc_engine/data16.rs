#[doc = "Register `DATA16` writer"]
pub struct W(crate::W<DATA16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA16_SPEC>;
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
impl From<crate::W<DATA16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA16` writer - Data register bits"]
pub type DATA16_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DATA16_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Data register bits"]
    #[inline(always)]
    pub fn data16(&mut self) -> DATA16_W<0> {
        DATA16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register - half-word sized\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data16](index.html) module"]
pub struct DATA16_SPEC;
impl crate::RegisterSpec for DATA16_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [data16::W](W) writer structure"]
impl crate::Writable for DATA16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA16 to value 0"]
impl crate::Resettable for DATA16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
