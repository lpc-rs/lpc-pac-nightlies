#[doc = "Register `LE` reader"]
pub struct R(crate::R<LE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LE` writer"]
pub struct W(crate::W<LE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LE_SPEC>;
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
impl From<crate::W<LE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LED` reader - Line-end delay."]
pub type LED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LED` writer - Line-end delay."]
pub type LED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LE_SPEC, u8, u8, 7, O>;
#[doc = "Field `LEE` reader - LCD Line end enable."]
pub type LEE_R = crate::BitReader<bool>;
#[doc = "Field `LEE` writer - LCD Line end enable."]
pub type LEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Line-end delay."]
    #[inline(always)]
    pub fn led(&self) -> LED_R {
        LED_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - LCD Line end enable."]
    #[inline(always)]
    pub fn lee(&self) -> LEE_R {
        LEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Line-end delay."]
    #[inline(always)]
    pub fn led(&mut self) -> LED_W<0> {
        LED_W::new(self)
    }
    #[doc = "Bit 16 - LCD Line end enable."]
    #[inline(always)]
    pub fn lee(&mut self) -> LEE_W<16> {
        LEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line End Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le](index.html) module"]
pub struct LE_SPEC;
impl crate::RegisterSpec for LE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [le::R](R) reader structure"]
impl crate::Readable for LE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [le::W](W) writer structure"]
impl crate::Writable for LE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LE to value 0"]
impl crate::Resettable for LE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
