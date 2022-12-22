#[doc = "Register `DMA_CHx_CTRL` reader"]
pub struct R(crate::R<DMA_CHX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_CTRL` writer"]
pub struct W(crate::W<DMA_CHX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_CTRL_SPEC>;
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
impl From<crate::W<DMA_CHX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBLx8` reader - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
pub type PBLX8_R = crate::BitReader<bool>;
#[doc = "Field `PBLx8` writer - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
pub type PBLX8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_CTRL_SPEC, bool, O>;
#[doc = "Field `DSL` reader - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
pub type DSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSL` writer - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_CHX_CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<16> {
        PBLX8_W::new(self)
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<18> {
        DSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channelx Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_ctrl](index.html) module"]
pub struct DMA_CHX_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CHX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_CHX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_CHX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_CTRL to value 0"]
impl crate::Resettable for DMA_CHX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
