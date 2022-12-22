#[doc = "Register `CHANEN` reader"]
pub struct R(crate::R<CHANEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANEN` writer"]
pub struct W(crate::W<CHANEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANEN_SPEC>;
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
impl From<crate::W<CHANEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_CH0` reader - Enable channel 0. When 1, PDM channel 0 is enabled."]
pub type EN_CH0_R = crate::BitReader<bool>;
#[doc = "Field `EN_CH0` writer - Enable channel 0. When 1, PDM channel 0 is enabled."]
pub type EN_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, bool, O>;
#[doc = "Field `EN_CH1` reader - Enable channel 1. When 1, PDM channel 1 is enabled."]
pub type EN_CH1_R = crate::BitReader<bool>;
#[doc = "Field `EN_CH1` writer - Enable channel 1. When 1, PDM channel 1 is enabled."]
pub type EN_CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&self) -> EN_CH0_R {
        EN_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&self) -> EN_CH1_R {
        EN_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&mut self) -> EN_CH0_W<0> {
        EN_CH0_W::new(self)
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&mut self) -> EN_CH1_W<1> {
        EN_CH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chanen](index.html) module"]
pub struct CHANEN_SPEC;
impl crate::RegisterSpec for CHANEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chanen::R](R) reader structure"]
impl crate::Readable for CHANEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chanen::W](W) writer structure"]
impl crate::Writable for CHANEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANEN to value 0"]
impl crate::Resettable for CHANEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
