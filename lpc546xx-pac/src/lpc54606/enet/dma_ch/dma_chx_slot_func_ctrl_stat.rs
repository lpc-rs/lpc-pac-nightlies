#[doc = "Register `DMA_CHx_SLOT_FUNC_CTRL_STAT` reader"]
pub struct R(crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_SLOT_FUNC_CTRL_STAT` writer"]
pub struct W(crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>;
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
impl From<crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC` reader - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
pub type ESC_R = crate::BitReader<bool>;
#[doc = "Field `ESC` writer - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
pub type ESC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC, bool, O>;
#[doc = "Field `ASC` reader - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
pub type ASC_R = crate::BitReader<bool>;
#[doc = "Field `ASC` writer - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
pub type ASC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC, bool, O>;
#[doc = "Field `RSN` reader - Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
pub type RSN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    #[inline(always)]
    pub fn esc(&mut self) -> ESC_W<0> {
        ESC_W::new(self)
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W<1> {
        ASC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slot Function Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_slot_func_ctrl_stat](index.html) module"]
pub struct DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC;
impl crate::RegisterSpec for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_slot_func_ctrl_stat::R](R) reader structure"]
impl crate::Readable for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_slot_func_ctrl_stat::W](W) writer structure"]
impl crate::Writable for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_SLOT_FUNC_CTRL_STAT to value 0"]
impl crate::Resettable for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
