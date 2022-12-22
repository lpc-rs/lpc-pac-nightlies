#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDR` reader - Receiver Data Ready."]
pub type RDR_R = crate::BitReader<bool>;
#[doc = "Field `OE` reader - Overrun Error."]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `PE` reader - Parity Error."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `FE` reader - Framing Error."]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `THRE` reader - Transmitter Holding Register Empty."]
pub type THRE_R = crate::BitReader<bool>;
#[doc = "Field `TEMT` reader - Transmitter Empty."]
pub type TEMT_R = crate::BitReader<bool>;
#[doc = "Field `RXFE` reader - Error in RX FIFO."]
pub type RXFE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Data Ready."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
