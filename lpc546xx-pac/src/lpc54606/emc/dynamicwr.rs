#[doc = "Register `DYNAMICWR` reader"]
pub struct R(crate::R<DYNAMICWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICWR` writer"]
pub struct W(crate::W<DYNAMICWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICWR_SPEC>;
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
impl From<crate::W<DYNAMICWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWR` reader - Write recovery time."]
pub type TWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWR` writer - Write recovery time."]
pub type TWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DYNAMICWR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Write recovery time."]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write recovery time."]
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W<0> {
        TWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write recovery time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicwr](index.html) module"]
pub struct DYNAMICWR_SPEC;
impl crate::RegisterSpec for DYNAMICWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicwr::R](R) reader structure"]
impl crate::Readable for DYNAMICWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicwr::W](W) writer structure"]
impl crate::Writable for DYNAMICWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICWR to value 0x0f"]
impl crate::Resettable for DYNAMICWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
