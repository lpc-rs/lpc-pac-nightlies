#[doc = "Register `FCR` writer"]
pub struct W(crate::W<IIR_FCR_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIR_FCR_FCR_SPEC>;
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
impl From<crate::W<IIR_FCR_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIR_FCR_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOEN` writer - FIFO Enable."]
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIR_FCR_FCR_SPEC, bool, O>;
#[doc = "Field `RXFIFORES` writer - RX FIFO Reset."]
pub type RXFIFORES_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIR_FCR_FCR_SPEC, bool, O>;
#[doc = "Field `TXFIFORES` writer - TX FIFO Reset."]
pub type TXFIFORES_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIR_FCR_FCR_SPEC, bool, O>;
#[doc = "Field `DMAMODE` writer - DMA Mode Select."]
pub type DMAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIR_FCR_FCR_SPEC, bool, O>;
#[doc = "Field `RXTRIGLVL` writer - RX Trigger Level."]
pub type RXTRIGLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IIR_FCR_FCR_SPEC, u8, u8, 2, O>;
impl W {
    #[doc = "Bit 0 - FIFO Enable."]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<0> {
        FIFOEN_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline(always)]
    pub fn rxfifores(&mut self) -> RXFIFORES_W<1> {
        RXFIFORES_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline(always)]
    pub fn txfifores(&mut self) -> TXFIFORES_W<2> {
        TXFIFORES_W::new(self)
    }
    #[doc = "Bit 3 - DMA Mode Select."]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DMAMODE_W<3> {
        DMAMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - RX Trigger Level."]
    #[inline(always)]
    pub fn rxtriglvl(&mut self) -> RXTRIGLVL_W<6> {
        RXTRIGLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir_fcr_fcr](index.html) module"]
pub struct IIR_FCR_FCR_SPEC;
impl crate::RegisterSpec for IIR_FCR_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iir_fcr_fcr::W](W) writer structure"]
impl crate::Writable for IIR_FCR_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for IIR_FCR_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
