#[doc = "Register `DMA_OTRIG_INMUX[%s]` reader"]
pub struct R(crate::R<DMA_OTRIG_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OTRIG_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OTRIG_INMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OTRIG_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_OTRIG_INMUX[%s]` writer"]
pub struct W(crate::W<DMA_OTRIG_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_OTRIG_INMUX_SPEC>;
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
impl From<crate::W<DMA_OTRIG_INMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_OTRIG_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP` reader - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 19)."]
pub type INP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INP` writer - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 19)."]
pub type INP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_OTRIG_INMUX_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 19)."]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 19)."]
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W<0> {
        INP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA output trigger selection to become DMA trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_otrig_inmux](index.html) module"]
pub struct DMA_OTRIG_INMUX_SPEC;
impl crate::RegisterSpec for DMA_OTRIG_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_otrig_inmux::R](R) reader structure"]
impl crate::Readable for DMA_OTRIG_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_otrig_inmux::W](W) writer structure"]
impl crate::Writable for DMA_OTRIG_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_OTRIG_INMUX[%s]
to value 0x1f"]
impl crate::Resettable for DMA_OTRIG_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
