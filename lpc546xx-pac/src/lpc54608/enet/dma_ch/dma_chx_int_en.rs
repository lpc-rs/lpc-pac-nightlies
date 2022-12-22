#[doc = "Register `DMA_CHx_INT_EN` reader"]
pub struct R(crate::R<DMA_CHX_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_INT_EN` writer"]
pub struct W(crate::W<DMA_CHX_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_INT_EN_SPEC>;
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
impl From<crate::W<DMA_CHX_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled."]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled."]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `TSE` reader - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled."]
pub type TSE_R = crate::BitReader<bool>;
#[doc = "Field `TSE` writer - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled."]
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `TBUE` reader - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled."]
pub type TBUE_R = crate::BitReader<bool>;
#[doc = "Field `TBUE` writer - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled."]
pub type TBUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `RIE` reader - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled."]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled."]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `RBUE` reader - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled."]
pub type RBUE_R = crate::BitReader<bool>;
#[doc = "Field `RBUE` writer - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled."]
pub type RBUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `RSE` reader - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled."]
pub type RSE_R = crate::BitReader<bool>;
#[doc = "Field `RSE` writer - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled."]
pub type RSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `RWTE` reader - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled."]
pub type RWTE_R = crate::BitReader<bool>;
#[doc = "Field `RWTE` writer - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled."]
pub type RWTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `ETIE` reader - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled."]
pub type ETIE_R = crate::BitReader<bool>;
#[doc = "Field `ETIE` writer - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled."]
pub type ETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `ERIE` reader - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled."]
pub type ERIE_R = crate::BitReader<bool>;
#[doc = "Field `ERIE` writer - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled."]
pub type ERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `FBEE` reader - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled."]
pub type FBEE_R = crate::BitReader<bool>;
#[doc = "Field `FBEE` writer - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled."]
pub type FBEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `AIE` reader - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt summary is enabled."]
pub type AIE_R = crate::BitReader<bool>;
#[doc = "Field `AIE` writer - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt summary is enabled."]
pub type AIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
#[doc = "Field `NIE` reader - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled."]
pub type NIE_R = crate::BitReader<bool>;
#[doc = "Field `NIE` writer - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled."]
pub type NIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_CHX_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled."]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled."]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt summary is enabled."]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled."]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<1> {
        TSE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn tbue(&mut self) -> TBUE_W<2> {
        TBUE_W::new(self)
    }
    #[doc = "Bit 6 - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn rbue(&mut self) -> RBUE_W<7> {
        RBUE_W::new(self)
    }
    #[doc = "Bit 8 - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W<8> {
        RSE_W::new(self)
    }
    #[doc = "Bit 9 - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled."]
    #[inline(always)]
    pub fn rwte(&mut self) -> RWTE_W<9> {
        RWTE_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn etie(&mut self) -> ETIE_W<10> {
        ETIE_W::new(self)
    }
    #[doc = "Bit 11 - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W<11> {
        ERIE_W::new(self)
    }
    #[doc = "Bit 12 - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled."]
    #[inline(always)]
    pub fn fbee(&mut self) -> FBEE_W<12> {
        FBEE_W::new(self)
    }
    #[doc = "Bit 14 - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt summary is enabled."]
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W<14> {
        AIE_W::new(self)
    }
    #[doc = "Bit 15 - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled."]
    #[inline(always)]
    pub fn nie(&mut self) -> NIE_W<15> {
        NIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channelx Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_int_en](index.html) module"]
pub struct DMA_CHX_INT_EN_SPEC;
impl crate::RegisterSpec for DMA_CHX_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_int_en::R](R) reader structure"]
impl crate::Readable for DMA_CHX_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_int_en::W](W) writer structure"]
impl crate::Writable for DMA_CHX_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_INT_EN to value 0"]
impl crate::Resettable for DMA_CHX_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
