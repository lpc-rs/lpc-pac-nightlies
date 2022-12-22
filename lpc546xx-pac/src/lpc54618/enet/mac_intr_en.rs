#[doc = "Register `MAC_INTR_EN` reader"]
pub struct R(crate::R<MAC_INTR_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_INTR_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_INTR_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_INTR_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_INTR_EN` writer"]
pub struct W(crate::W<MAC_INTR_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_INTR_EN_SPEC>;
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
impl From<crate::W<MAC_INTR_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_INTR_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYIE` reader - PHY Interrupt Enable."]
pub type PHYIE_R = crate::BitReader<bool>;
#[doc = "Field `PHYIE` writer - PHY Interrupt Enable."]
pub type PHYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_INTR_EN_SPEC, bool, O>;
#[doc = "Field `PMTIE` reader - PMT Interrupt Enable."]
pub type PMTIE_R = crate::BitReader<bool>;
#[doc = "Field `PMTIE` writer - PMT Interrupt Enable."]
pub type PMTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_INTR_EN_SPEC, bool, O>;
#[doc = "Field `LPIIE` reader - LPI Interrupt Enable."]
pub type LPIIE_R = crate::BitReader<bool>;
#[doc = "Field `LPIIE` writer - LPI Interrupt Enable."]
pub type LPIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_INTR_EN_SPEC, bool, O>;
#[doc = "Field `TSIE` reader - Timestamp Interrupt Enable."]
pub type TSIE_R = crate::BitReader<bool>;
#[doc = "Field `TSIE` writer - Timestamp Interrupt Enable."]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_INTR_EN_SPEC, bool, O>;
#[doc = "Field `TXSTSIE` reader - Transmit Status Interrupt Enable."]
pub type TXSTSIE_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSIE` writer - Transmit Status Interrupt Enable."]
pub type TXSTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_INTR_EN_SPEC, bool, O>;
#[doc = "Field `RXSTSIS` reader - Receive Status Interrupt Enable."]
pub type RXSTSIS_R = crate::BitReader<bool>;
#[doc = "Field `RXSTSIS` writer - Receive Status Interrupt Enable."]
pub type RXSTSIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAC_INTR_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - PHY Interrupt Enable."]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable."]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable."]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable."]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable."]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable."]
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PHY Interrupt Enable."]
    #[inline(always)]
    pub fn phyie(&mut self) -> PHYIE_W<3> {
        PHYIE_W::new(self)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable."]
    #[inline(always)]
    pub fn pmtie(&mut self) -> PMTIE_W<4> {
        PMTIE_W::new(self)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable."]
    #[inline(always)]
    pub fn lpiie(&mut self) -> LPIIE_W<5> {
        LPIIE_W::new(self)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable."]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<12> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable."]
    #[inline(always)]
    pub fn txstsie(&mut self) -> TXSTSIE_W<13> {
        TXSTSIE_W::new(self)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable."]
    #[inline(always)]
    pub fn rxstsis(&mut self) -> RXSTSIS_W<14> {
        RXSTSIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intr_en](index.html) module"]
pub struct MAC_INTR_EN_SPEC;
impl crate::RegisterSpec for MAC_INTR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_intr_en::R](R) reader structure"]
impl crate::Readable for MAC_INTR_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_intr_en::W](W) writer structure"]
impl crate::Writable for MAC_INTR_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_INTR_EN to value 0"]
impl crate::Resettable for MAC_INTR_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
