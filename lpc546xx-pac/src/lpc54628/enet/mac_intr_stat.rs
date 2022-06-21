#[doc = "Register `MAC_INTR_STAT` reader"]
pub struct R(crate::R<MAC_INTR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_INTR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_INTR_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_INTR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PHYIS` reader - PHY Interrupt."]
pub type PHYIS_R = crate::BitReader<bool>;
#[doc = "Field `PMTIS` reader - PMT Interrupt Status."]
pub type PMTIS_R = crate::BitReader<bool>;
#[doc = "Field `LPIIS` reader - LPI Interrupt Status."]
pub type LPIIS_R = crate::BitReader<bool>;
#[doc = "Field `TSIS` reader - Timestamp interrupt status."]
pub type TSIS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSIS` reader - Transmit Status Interrupt."]
pub type TXSTSIS_R = crate::BitReader<bool>;
#[doc = "Field `RXSTSIS` reader - Receive Status Interrupt."]
pub type RXSTSIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - PHY Interrupt."]
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Status."]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Status."]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Timestamp interrupt status."]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt."]
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt."]
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Interrupt status register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intr_stat](index.html) module"]
pub struct MAC_INTR_STAT_SPEC;
impl crate::RegisterSpec for MAC_INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_intr_stat::R](R) reader structure"]
impl crate::Readable for MAC_INTR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_INTR_STAT to value 0"]
impl crate::Resettable for MAC_INTR_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
