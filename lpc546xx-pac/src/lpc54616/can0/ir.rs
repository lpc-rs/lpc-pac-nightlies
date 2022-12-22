#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0N` reader - Rx FIFO 0 new message."]
pub type RF0N_R = crate::BitReader<bool>;
#[doc = "Field `RF0N` writer - Rx FIFO 0 new message."]
pub type RF0N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0W` reader - Rx FIFO 0 watermark reached."]
pub type RF0W_R = crate::BitReader<bool>;
#[doc = "Field `RF0W` writer - Rx FIFO 0 watermark reached."]
pub type RF0W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0F` reader - Rx FIFO 0 full."]
pub type RF0F_R = crate::BitReader<bool>;
#[doc = "Field `RF0F` writer - Rx FIFO 0 full."]
pub type RF0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost."]
pub type RF0L_R = crate::BitReader<bool>;
#[doc = "Field `RF0L` writer - Rx FIFO 0 message lost."]
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1N` reader - Rx FIFO 1 new message."]
pub type RF1N_R = crate::BitReader<bool>;
#[doc = "Field `RF1N` writer - Rx FIFO 1 new message."]
pub type RF1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1W` reader - Rx FIFO 1 watermark reached."]
pub type RF1W_R = crate::BitReader<bool>;
#[doc = "Field `RF1W` writer - Rx FIFO 1 watermark reached."]
pub type RF1W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1F` reader - Rx FIFO 1 full."]
pub type RF1F_R = crate::BitReader<bool>;
#[doc = "Field `RF1F` writer - Rx FIFO 1 full."]
pub type RF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost."]
pub type RF1L_R = crate::BitReader<bool>;
#[doc = "Field `RF1L` writer - Rx FIFO 1 message lost."]
pub type RF1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `HPM` reader - High priority message."]
pub type HPM_R = crate::BitReader<bool>;
#[doc = "Field `HPM` writer - High priority message."]
pub type HPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TC` reader - Transmission completed."]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - Transmission completed."]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TCF` reader - Transmission cancellation finished."]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - Transmission cancellation finished."]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Tx FIFO empty."]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Tx FIFO empty."]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFN` reader - Tx event FIFO new entry."]
pub type TEFN_R = crate::BitReader<bool>;
#[doc = "Field `TEFN` writer - Tx event FIFO new entry."]
pub type TEFN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFW` reader - Tx event FIFO watermark reached."]
pub type TEFW_R = crate::BitReader<bool>;
#[doc = "Field `TEFW` writer - Tx event FIFO watermark reached."]
pub type TEFW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFF` reader - Tx event FIFO full."]
pub type TEFF_R = crate::BitReader<bool>;
#[doc = "Field `TEFF` writer - Tx event FIFO full."]
pub type TEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFL` reader - Tx event FIFO element lost."]
pub type TEFL_R = crate::BitReader<bool>;
#[doc = "Field `TEFL` writer - Tx event FIFO element lost."]
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TSW` reader - Timestamp wraparound."]
pub type TSW_R = crate::BitReader<bool>;
#[doc = "Field `TSW` writer - Timestamp wraparound."]
pub type TSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `MRAF` reader - Message RAM access failure."]
pub type MRAF_R = crate::BitReader<bool>;
#[doc = "Field `MRAF` writer - Message RAM access failure."]
pub type MRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TOO` reader - Timeout occurred."]
pub type TOO_R = crate::BitReader<bool>;
#[doc = "Field `TOO` writer - Timeout occurred."]
pub type TOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `DRX` reader - Message stored in dedicated Rx buffer."]
pub type DRX_R = crate::BitReader<bool>;
#[doc = "Field `DRX` writer - Message stored in dedicated Rx buffer."]
pub type DRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BEC` reader - Bit error corrected."]
pub type BEC_R = crate::BitReader<bool>;
#[doc = "Field `BEC` writer - Bit error corrected."]
pub type BEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BEU` reader - Bit error uncorrected."]
pub type BEU_R = crate::BitReader<bool>;
#[doc = "Field `BEU` writer - Bit error uncorrected."]
pub type BEU_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ELO` reader - Error logging overflow."]
pub type ELO_R = crate::BitReader<bool>;
#[doc = "Field `ELO` writer - Error logging overflow."]
pub type ELO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `EP` reader - Error passive."]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EP` writer - Error passive."]
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `EW` reader - Warning status."]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `EW` writer - Warning status."]
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BO` reader - Bus_Off Status."]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `BO` writer - Bus_Off Status."]
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `WDI` reader - Watchdog interrupt."]
pub type WDI_R = crate::BitReader<bool>;
#[doc = "Field `WDI` writer - Watchdog interrupt."]
pub type WDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `PEA` reader - Protocol error in arbitration phase."]
pub type PEA_R = crate::BitReader<bool>;
#[doc = "Field `PEA` writer - Protocol error in arbitration phase."]
pub type PEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `PED` reader - Protocol error in data phase."]
pub type PED_R = crate::BitReader<bool>;
#[doc = "Field `PED` writer - Protocol error in data phase."]
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ARA` reader - Access to reserved address."]
pub type ARA_R = crate::BitReader<bool>;
#[doc = "Field `ARA` writer - Access to reserved address."]
pub type ARA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message."]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message."]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High priority message."]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission completed."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission cancellation finished."]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry."]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached."]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx event FIFO full."]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp wraparound."]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM access failure."]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout occurred."]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit error corrected."]
    #[inline(always)]
    pub fn bec(&self) -> BEC_R {
        BEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit error uncorrected."]
    #[inline(always)]
    pub fn beu(&self) -> BEU_R {
        BEU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error logging overflow."]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error passive."]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning status."]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status."]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog interrupt."]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase."]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol error in data phase."]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to reserved address."]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message."]
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W<0> {
        RF0N_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub fn rf0w(&mut self) -> RF0W_W<1> {
        RF0W_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W<2> {
        RF0F_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W<3> {
        RF0L_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message."]
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W<4> {
        RF1N_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub fn rf1w(&mut self) -> RF1W_W<5> {
        RF1W_W::new(self)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W<6> {
        RF1F_W::new(self)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W<7> {
        RF1L_W::new(self)
    }
    #[doc = "Bit 8 - High priority message."]
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W<8> {
        HPM_W::new(self)
    }
    #[doc = "Bit 9 - Transmission completed."]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<9> {
        TC_W::new(self)
    }
    #[doc = "Bit 10 - Transmission cancellation finished."]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<10> {
        TCF_W::new(self)
    }
    #[doc = "Bit 11 - Tx FIFO empty."]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<11> {
        TFE_W::new(self)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry."]
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W<12> {
        TEFN_W::new(self)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached."]
    #[inline(always)]
    pub fn tefw(&mut self) -> TEFW_W<13> {
        TEFW_W::new(self)
    }
    #[doc = "Bit 14 - Tx event FIFO full."]
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W<14> {
        TEFF_W::new(self)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W<15> {
        TEFL_W::new(self)
    }
    #[doc = "Bit 16 - Timestamp wraparound."]
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W<16> {
        TSW_W::new(self)
    }
    #[doc = "Bit 17 - Message RAM access failure."]
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W<17> {
        MRAF_W::new(self)
    }
    #[doc = "Bit 18 - Timeout occurred."]
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W<18> {
        TOO_W::new(self)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W<19> {
        DRX_W::new(self)
    }
    #[doc = "Bit 20 - Bit error corrected."]
    #[inline(always)]
    pub fn bec(&mut self) -> BEC_W<20> {
        BEC_W::new(self)
    }
    #[doc = "Bit 21 - Bit error uncorrected."]
    #[inline(always)]
    pub fn beu(&mut self) -> BEU_W<21> {
        BEU_W::new(self)
    }
    #[doc = "Bit 22 - Error logging overflow."]
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W<22> {
        ELO_W::new(self)
    }
    #[doc = "Bit 23 - Error passive."]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<23> {
        EP_W::new(self)
    }
    #[doc = "Bit 24 - Warning status."]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<24> {
        EW_W::new(self)
    }
    #[doc = "Bit 25 - Bus_Off Status."]
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<25> {
        BO_W::new(self)
    }
    #[doc = "Bit 26 - Watchdog interrupt."]
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W<26> {
        WDI_W::new(self)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase."]
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W<27> {
        PEA_W::new(self)
    }
    #[doc = "Bit 28 - Protocol error in data phase."]
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W<28> {
        PED_W::new(self)
    }
    #[doc = "Bit 29 - Access to reserved address."]
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W<29> {
        ARA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
