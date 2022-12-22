#[doc = "Register `MAC_HW_FEAT2` reader"]
pub struct R(crate::R<MAC_HW_FEAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_HW_FEAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_HW_FEAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_HW_FEAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXQCNT` reader - Number of MTL Receive Queues."]
pub type RXQCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXQCNT` reader - Number of MTL Transmit Queues."]
pub type TXQCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCHCNT` reader - Number of DMA Receive Channels."]
pub type RXCHCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCHCNT` reader - Number of DMA Transmit Channels."]
pub type TXCHCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSOUTNUM` reader - Number of PPS Outputs."]
pub type PPSOUTNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUXSNAPNUM` reader - Number of Auxiliary Snapshot Inputs."]
pub type AUXSNAPNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of MTL Receive Queues."]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Number of MTL Transmit Queues."]
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of DMA Receive Channels."]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Number of DMA Transmit Channels."]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Number of PPS Outputs."]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Number of Auxiliary Snapshot Inputs."]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "MAC hardware feature register 0x0201\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hw_feat2](index.html) module"]
pub struct MAC_HW_FEAT2_SPEC;
impl crate::RegisterSpec for MAC_HW_FEAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_hw_feat2::R](R) reader structure"]
impl crate::Readable for MAC_HW_FEAT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_HW_FEAT2 to value 0"]
impl crate::Resettable for MAC_HW_FEAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
