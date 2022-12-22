#[doc = "Register `STARTER0` reader"]
pub struct R(crate::R<STARTER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTER0` writer"]
pub struct W(crate::W<STARTER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTER0_SPEC>;
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
impl From<crate::W<STARTER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_BOD` reader - WWDT and BOD interrupt wake-up."]
pub type WDT_BOD_R = crate::BitReader<bool>;
#[doc = "Field `WDT_BOD` writer - WWDT and BOD interrupt wake-up."]
pub type WDT_BOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `DMA` reader - DMA wake-up."]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - DMA wake-up."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `GINT0` reader - Group interrupt 0 wake-up."]
pub type GINT0_R = crate::BitReader<bool>;
#[doc = "Field `GINT0` writer - Group interrupt 0 wake-up."]
pub type GINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `GINT1` reader - Group interrupt 1 wake-up."]
pub type GINT1_R = crate::BitReader<bool>;
#[doc = "Field `GINT1` writer - Group interrupt 1 wake-up."]
pub type GINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `PIN_INT0` reader - GPIO pin interrupt 0 wake-up."]
pub type PIN_INT0_R = crate::BitReader<bool>;
#[doc = "Field `PIN_INT0` writer - GPIO pin interrupt 0 wake-up."]
pub type PIN_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `PIN_INT1` reader - GPIO pin interrupt 1 wake-up."]
pub type PIN_INT1_R = crate::BitReader<bool>;
#[doc = "Field `PIN_INT1` writer - GPIO pin interrupt 1 wake-up."]
pub type PIN_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `PIN_INT2` reader - GPIO pin interrupt 2 wake-up."]
pub type PIN_INT2_R = crate::BitReader<bool>;
#[doc = "Field `PIN_INT2` writer - GPIO pin interrupt 2 wake-up."]
pub type PIN_INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `PIN_INT3` reader - GPIO pin interrupt 3 wake-up."]
pub type PIN_INT3_R = crate::BitReader<bool>;
#[doc = "Field `PIN_INT3` writer - GPIO pin interrupt 3 wake-up."]
pub type PIN_INT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `UTICK` reader - Micro-tick Timer wake-up."]
pub type UTICK_R = crate::BitReader<bool>;
#[doc = "Field `UTICK` writer - Micro-tick Timer wake-up."]
pub type UTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `MRT` reader - Multi-Rate Timer wake-up."]
pub type MRT_R = crate::BitReader<bool>;
#[doc = "Field `MRT` writer - Multi-Rate Timer wake-up."]
pub type MRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `CTIMER0` reader - Standard counter/timer CTIMER0 wake-up."]
pub type CTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER0` writer - Standard counter/timer CTIMER0 wake-up."]
pub type CTIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `CTIMER1` reader - Standard counter/timer CTIMER1 wake-up."]
pub type CTIMER1_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER1` writer - Standard counter/timer CTIMER1 wake-up."]
pub type CTIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `SCT0` reader - SCT0 wake-up."]
pub type SCT0_R = crate::BitReader<bool>;
#[doc = "Field `SCT0` writer - SCT0 wake-up."]
pub type SCT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `CTIMER3` reader - Standard counter/timer CTIMER3 wake-up."]
pub type CTIMER3_R = crate::BitReader<bool>;
#[doc = "Field `CTIMER3` writer - Standard counter/timer CTIMER3 wake-up."]
pub type CTIMER3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM0` reader - Flexcomm0 peripheral interrupt wake-up."]
pub type FLEXCOMM0_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM0` writer - Flexcomm0 peripheral interrupt wake-up."]
pub type FLEXCOMM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM1` reader - Flexcomm1 peripheral interrupt wake-up."]
pub type FLEXCOMM1_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM1` writer - Flexcomm1 peripheral interrupt wake-up."]
pub type FLEXCOMM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM2` reader - Flexcomm2 peripheral interrupt wake-up."]
pub type FLEXCOMM2_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM2` writer - Flexcomm2 peripheral interrupt wake-up."]
pub type FLEXCOMM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM3` reader - Flexcomm3 peripheral interrupt wake-up."]
pub type FLEXCOMM3_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM3` writer - Flexcomm3 peripheral interrupt wake-up."]
pub type FLEXCOMM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM4` reader - Flexcomm4 peripheral interrupt wake-up."]
pub type FLEXCOMM4_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM4` writer - Flexcomm4 peripheral interrupt wake-up."]
pub type FLEXCOMM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM5` reader - Flexcomm5 peripheral interrupt wake-up."]
pub type FLEXCOMM5_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM5` writer - Flexcomm5 peripheral interrupt wake-up."]
pub type FLEXCOMM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM6` reader - Flexcomm6 peripheral interrupt wake-up."]
pub type FLEXCOMM6_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM6` writer - Flexcomm6 peripheral interrupt wake-up."]
pub type FLEXCOMM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `FLEXCOMM7` reader - Flexcomm7 peripheral interrupt wake-up."]
pub type FLEXCOMM7_R = crate::BitReader<bool>;
#[doc = "Field `FLEXCOMM7` writer - Flexcomm7 peripheral interrupt wake-up."]
pub type FLEXCOMM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `ADC0_SEQA` reader - ADC0 sequence A interrupt wake-up."]
pub type ADC0_SEQA_R = crate::BitReader<bool>;
#[doc = "Field `ADC0_SEQA` writer - ADC0 sequence A interrupt wake-up."]
pub type ADC0_SEQA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `ADC0_SEQB` reader - ADC0 sequence B interrupt wake-up."]
pub type ADC0_SEQB_R = crate::BitReader<bool>;
#[doc = "Field `ADC0_SEQB` writer - ADC0 sequence B interrupt wake-up."]
pub type ADC0_SEQB_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `ADC0_THCMP` reader - ADC0 threshold and error interrupt wake-up."]
pub type ADC0_THCMP_R = crate::BitReader<bool>;
#[doc = "Field `ADC0_THCMP` writer - ADC0 threshold and error interrupt wake-up."]
pub type ADC0_THCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `DMIC` reader - Digital microphone interrupt wake-up."]
pub type DMIC_R = crate::BitReader<bool>;
#[doc = "Field `DMIC` writer - Digital microphone interrupt wake-up."]
pub type DMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `HWVAD` reader - Hardware voice activity detect interrupt wake-up."]
pub type HWVAD_R = crate::BitReader<bool>;
#[doc = "Field `HWVAD` writer - Hardware voice activity detect interrupt wake-up."]
pub type HWVAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `USB0_NEEDCLK` reader - USB activity interrupt wake-up."]
pub type USB0_NEEDCLK_R = crate::BitReader<bool>;
#[doc = "Field `USB0_NEEDCLK` writer - USB activity interrupt wake-up."]
pub type USB0_NEEDCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `USB0` reader - USB function interrupt wake-up."]
pub type USB0_R = crate::BitReader<bool>;
#[doc = "Field `USB0` writer - USB function interrupt wake-up."]
pub type USB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
#[doc = "Field `RTC` reader - RTC interrupt alarm and wake-up timer."]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - RTC interrupt alarm and wake-up timer."]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTER0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&self) -> WDT_BOD_R {
        WDT_BOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&self) -> GINT0_R {
        GINT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&self) -> GINT1_R {
        GINT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&self) -> PIN_INT0_R {
        PIN_INT0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&self) -> PIN_INT1_R {
        PIN_INT1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&self) -> PIN_INT2_R {
        PIN_INT2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&self) -> PIN_INT3_R {
        PIN_INT3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&self) -> ADC0_SEQA_R {
        ADC0_SEQA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&self) -> ADC0_SEQB_R {
        ADC0_SEQB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&self) -> ADC0_THCMP_R {
        ADC0_THCMP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&self) -> HWVAD_R {
        HWVAD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&mut self) -> WDT_BOD_W<0> {
        WDT_BOD_W::new(self)
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<1> {
        DMA_W::new(self)
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&mut self) -> GINT0_W<2> {
        GINT0_W::new(self)
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&mut self) -> GINT1_W<3> {
        GINT1_W::new(self)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&mut self) -> PIN_INT0_W<4> {
        PIN_INT0_W::new(self)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&mut self) -> PIN_INT1_W<5> {
        PIN_INT1_W::new(self)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&mut self) -> PIN_INT2_W<6> {
        PIN_INT2_W::new(self)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&mut self) -> PIN_INT3_W<7> {
        PIN_INT3_W::new(self)
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&mut self) -> UTICK_W<8> {
        UTICK_W::new(self)
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W<9> {
        MRT_W::new(self)
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> CTIMER0_W<10> {
        CTIMER0_W::new(self)
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> CTIMER1_W<11> {
        CTIMER1_W::new(self)
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W<12> {
        SCT0_W::new(self)
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> CTIMER3_W<13> {
        CTIMER3_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W<14> {
        FLEXCOMM0_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W<15> {
        FLEXCOMM1_W::new(self)
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W<16> {
        FLEXCOMM2_W::new(self)
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W<17> {
        FLEXCOMM3_W::new(self)
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W<18> {
        FLEXCOMM4_W::new(self)
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W<19> {
        FLEXCOMM5_W::new(self)
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W<20> {
        FLEXCOMM6_W::new(self)
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W<21> {
        FLEXCOMM7_W::new(self)
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&mut self) -> ADC0_SEQA_W<22> {
        ADC0_SEQA_W::new(self)
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&mut self) -> ADC0_SEQB_W<23> {
        ADC0_SEQB_W::new(self)
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&mut self) -> ADC0_THCMP_W<24> {
        ADC0_THCMP_W::new(self)
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&mut self) -> DMIC_W<25> {
        DMIC_W::new(self)
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&mut self) -> HWVAD_W<26> {
        HWVAD_W::new(self)
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&mut self) -> USB0_NEEDCLK_W<27> {
        USB0_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&mut self) -> USB0_W<28> {
        USB0_W::new(self)
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<29> {
        RTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 0 wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starter0](index.html) module"]
pub struct STARTER0_SPEC;
impl crate::RegisterSpec for STARTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starter0::R](R) reader structure"]
impl crate::Readable for STARTER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starter0::W](W) writer structure"]
impl crate::Writable for STARTER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTER0 to value 0"]
impl crate::Resettable for STARTER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
