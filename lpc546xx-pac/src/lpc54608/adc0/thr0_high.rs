#[doc = "Register `THR0_HIGH` reader"]
pub struct R(crate::R<THR0_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THR0_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THR0_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THR0_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THR0_HIGH` writer"]
pub struct W(crate::W<THR0_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THR0_HIGH_SPEC>;
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
impl From<crate::W<THR0_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THR0_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRHIGH` reader - High threshold value against which ADC results will be compared"]
pub type THRHIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THRHIGH` writer - High threshold value against which ADC results will be compared"]
pub type THRHIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THR0_HIGH_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 4:15 - High threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrhigh(&self) -> THRHIGH_R {
        THRHIGH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - High threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrhigh(&mut self) -> THRHIGH_W<4> {
        THRHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr0_high](index.html) module"]
pub struct THR0_HIGH_SPEC;
impl crate::RegisterSpec for THR0_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thr0_high::R](R) reader structure"]
impl crate::Readable for THR0_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thr0_high::W](W) writer structure"]
impl crate::Writable for THR0_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THR0_HIGH to value 0"]
impl crate::Resettable for THR0_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
