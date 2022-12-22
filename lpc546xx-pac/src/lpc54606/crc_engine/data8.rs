#[doc = "Register `DATA8` writer"]
pub struct W(crate::W<DATA8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA8_SPEC>;
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
impl From<crate::W<DATA8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA8` writer - Data register bits"]
pub type DATA8_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DATA8_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Data register bits"]
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W<0> {
        DATA8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register - byte sized\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data8](index.html) module"]
pub struct DATA8_SPEC;
impl crate::RegisterSpec for DATA8_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [data8::W](W) writer structure"]
impl crate::Writable for DATA8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA8 to value 0"]
impl crate::Resettable for DATA8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
