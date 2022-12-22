#[doc = "Register `PDRUNCFGSET0` reader"]
pub struct R(crate::R<PDRUNCFGSET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRUNCFGSET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRUNCFGSET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRUNCFGSET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRUNCFGSET0` writer"]
pub struct W(crate::W<PDRUNCFGSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFGSET0_SPEC>;
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
impl From<crate::W<PDRUNCFGSET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFGSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDEN_FRO` reader - FRO oscillator."]
pub type PDEN_FRO_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_FRO` writer - FRO oscillator."]
pub type PDEN_FRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_TS` reader - Temp sensor."]
pub type PDEN_TS_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_TS` writer - Temp sensor."]
pub type PDEN_TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_BOD_RST` reader - Brown-out Detect reset."]
pub type PDEN_BOD_RST_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_BOD_RST` writer - Brown-out Detect reset."]
pub type PDEN_BOD_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_BOD_INTR` reader - Brown-out Detect interrupt."]
pub type PDEN_BOD_INTR_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_BOD_INTR` writer - Brown-out Detect interrupt."]
pub type PDEN_BOD_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_VD2_ANA` reader - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
pub type PDEN_VD2_ANA_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_VD2_ANA` writer - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
pub type PDEN_VD2_ANA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_ADC0` reader - ADC power."]
pub type PDEN_ADC0_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_ADC0` writer - ADC power."]
pub type PDEN_ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_SRAMX` reader - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
pub type PDEN_SRAMX_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_SRAMX` writer - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
pub type PDEN_SRAMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_SRAM0` reader - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
pub type PDEN_SRAM0_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_SRAM0` writer - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
pub type PDEN_SRAM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_SRAM1_2_3` reader - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
pub type PDEN_SRAM1_2_3_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_SRAM1_2_3` writer - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
pub type PDEN_SRAM1_2_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_USB_RAM` reader - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
pub type PDEN_USB_RAM_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_USB_RAM` writer - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
pub type PDEN_USB_RAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_ROM` reader - ROM (also enable/disable bit 27)."]
pub type PDEN_ROM_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_ROM` writer - ROM (also enable/disable bit 27)."]
pub type PDEN_ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_VDDA` reader - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
pub type PDEN_VDDA_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_VDDA` writer - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
pub type PDEN_VDDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_WDT_OSC` reader - Watchdog oscillator."]
pub type PDEN_WDT_OSC_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_WDT_OSC` writer - Watchdog oscillator."]
pub type PDEN_WDT_OSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_USB0_PHY` reader - USB0 PHY power (also enable/disable bit 28)."]
pub type PDEN_USB0_PHY_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_USB0_PHY` writer - USB0 PHY power (also enable/disable bit 28)."]
pub type PDEN_USB0_PHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_SYS_PLL` reader - System PLL (PLL0) power (also enable/disable bit 26)."]
pub type PDEN_SYS_PLL_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_SYS_PLL` writer - System PLL (PLL0) power (also enable/disable bit 26)."]
pub type PDEN_SYS_PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_VREFP` reader - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
pub type PDEN_VREFP_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_VREFP` writer - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
pub type PDEN_VREFP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_VD3` reader - Power control for all PLLs."]
pub type PDEN_VD3_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_VD3` writer - Power control for all PLLs."]
pub type PDEN_VD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_VD4` reader - Power control for all SRAMs and ROM."]
pub type PDEN_VD4_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_VD4` writer - Power control for all SRAMs and ROM."]
pub type PDEN_VD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_VD5` reader - Power control both USB0 PHY and USB1 PHY."]
pub type PDEN_VD5_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_VD5` writer - Power control both USB0 PHY and USB1 PHY."]
pub type PDEN_VD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
#[doc = "Field `PDEN_VD6` reader - Power control for EEPROM."]
pub type PDEN_VD6_R = crate::BitReader<bool>;
#[doc = "Field `PDEN_VD6` writer - Power control for EEPROM."]
pub type PDEN_VD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFGSET0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - FRO oscillator."]
    #[inline(always)]
    pub fn pden_fro(&self) -> PDEN_FRO_R {
        PDEN_FRO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Temp sensor."]
    #[inline(always)]
    pub fn pden_ts(&self) -> PDEN_TS_R {
        PDEN_TS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Brown-out Detect reset."]
    #[inline(always)]
    pub fn pden_bod_rst(&self) -> PDEN_BOD_RST_R {
        PDEN_BOD_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Brown-out Detect interrupt."]
    #[inline(always)]
    pub fn pden_bod_intr(&self) -> PDEN_BOD_INTR_R {
        PDEN_BOD_INTR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
    #[inline(always)]
    pub fn pden_vd2_ana(&self) -> PDEN_VD2_ANA_R {
        PDEN_VD2_ANA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC power."]
    #[inline(always)]
    pub fn pden_adc0(&self) -> PDEN_ADC0_R {
        PDEN_ADC0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sramx(&self) -> PDEN_SRAMX_R {
        PDEN_SRAMX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram0(&self) -> PDEN_SRAM0_R {
        PDEN_SRAM0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram1_2_3(&self) -> PDEN_SRAM1_2_3_R {
        PDEN_SRAM1_2_3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_usb_ram(&self) -> PDEN_USB_RAM_R {
        PDEN_USB_RAM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ROM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_rom(&self) -> PDEN_ROM_R {
        PDEN_ROM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
    #[inline(always)]
    pub fn pden_vdda(&self) -> PDEN_VDDA_R {
        PDEN_VDDA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog oscillator."]
    #[inline(always)]
    pub fn pden_wdt_osc(&self) -> PDEN_WDT_OSC_R {
        PDEN_WDT_OSC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB0 PHY power (also enable/disable bit 28)."]
    #[inline(always)]
    pub fn pden_usb0_phy(&self) -> PDEN_USB0_PHY_R {
        PDEN_USB0_PHY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - System PLL (PLL0) power (also enable/disable bit 26)."]
    #[inline(always)]
    pub fn pden_sys_pll(&self) -> PDEN_SYS_PLL_R {
        PDEN_SYS_PLL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
    #[inline(always)]
    pub fn pden_vrefp(&self) -> PDEN_VREFP_R {
        PDEN_VREFP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Power control for all PLLs."]
    #[inline(always)]
    pub fn pden_vd3(&self) -> PDEN_VD3_R {
        PDEN_VD3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Power control for all SRAMs and ROM."]
    #[inline(always)]
    pub fn pden_vd4(&self) -> PDEN_VD4_R {
        PDEN_VD4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control both USB0 PHY and USB1 PHY."]
    #[inline(always)]
    pub fn pden_vd5(&self) -> PDEN_VD5_R {
        PDEN_VD5_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Power control for EEPROM."]
    #[inline(always)]
    pub fn pden_vd6(&self) -> PDEN_VD6_R {
        PDEN_VD6_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - FRO oscillator."]
    #[inline(always)]
    pub fn pden_fro(&mut self) -> PDEN_FRO_W<4> {
        PDEN_FRO_W::new(self)
    }
    #[doc = "Bit 6 - Temp sensor."]
    #[inline(always)]
    pub fn pden_ts(&mut self) -> PDEN_TS_W<6> {
        PDEN_TS_W::new(self)
    }
    #[doc = "Bit 7 - Brown-out Detect reset."]
    #[inline(always)]
    pub fn pden_bod_rst(&mut self) -> PDEN_BOD_RST_W<7> {
        PDEN_BOD_RST_W::new(self)
    }
    #[doc = "Bit 8 - Brown-out Detect interrupt."]
    #[inline(always)]
    pub fn pden_bod_intr(&mut self) -> PDEN_BOD_INTR_W<8> {
        PDEN_BOD_INTR_W::new(self)
    }
    #[doc = "Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
    #[inline(always)]
    pub fn pden_vd2_ana(&mut self) -> PDEN_VD2_ANA_W<9> {
        PDEN_VD2_ANA_W::new(self)
    }
    #[doc = "Bit 10 - ADC power."]
    #[inline(always)]
    pub fn pden_adc0(&mut self) -> PDEN_ADC0_W<10> {
        PDEN_ADC0_W::new(self)
    }
    #[doc = "Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sramx(&mut self) -> PDEN_SRAMX_W<13> {
        PDEN_SRAMX_W::new(self)
    }
    #[doc = "Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram0(&mut self) -> PDEN_SRAM0_W<14> {
        PDEN_SRAM0_W::new(self)
    }
    #[doc = "Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram1_2_3(&mut self) -> PDEN_SRAM1_2_3_W<15> {
        PDEN_SRAM1_2_3_W::new(self)
    }
    #[doc = "Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_usb_ram(&mut self) -> PDEN_USB_RAM_W<16> {
        PDEN_USB_RAM_W::new(self)
    }
    #[doc = "Bit 17 - ROM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_rom(&mut self) -> PDEN_ROM_W<17> {
        PDEN_ROM_W::new(self)
    }
    #[doc = "Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
    #[inline(always)]
    pub fn pden_vdda(&mut self) -> PDEN_VDDA_W<19> {
        PDEN_VDDA_W::new(self)
    }
    #[doc = "Bit 20 - Watchdog oscillator."]
    #[inline(always)]
    pub fn pden_wdt_osc(&mut self) -> PDEN_WDT_OSC_W<20> {
        PDEN_WDT_OSC_W::new(self)
    }
    #[doc = "Bit 21 - USB0 PHY power (also enable/disable bit 28)."]
    #[inline(always)]
    pub fn pden_usb0_phy(&mut self) -> PDEN_USB0_PHY_W<21> {
        PDEN_USB0_PHY_W::new(self)
    }
    #[doc = "Bit 22 - System PLL (PLL0) power (also enable/disable bit 26)."]
    #[inline(always)]
    pub fn pden_sys_pll(&mut self) -> PDEN_SYS_PLL_W<22> {
        PDEN_SYS_PLL_W::new(self)
    }
    #[doc = "Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
    #[inline(always)]
    pub fn pden_vrefp(&mut self) -> PDEN_VREFP_W<23> {
        PDEN_VREFP_W::new(self)
    }
    #[doc = "Bit 26 - Power control for all PLLs."]
    #[inline(always)]
    pub fn pden_vd3(&mut self) -> PDEN_VD3_W<26> {
        PDEN_VD3_W::new(self)
    }
    #[doc = "Bit 27 - Power control for all SRAMs and ROM."]
    #[inline(always)]
    pub fn pden_vd4(&mut self) -> PDEN_VD4_W<27> {
        PDEN_VD4_W::new(self)
    }
    #[doc = "Bit 28 - Power control both USB0 PHY and USB1 PHY."]
    #[inline(always)]
    pub fn pden_vd5(&mut self) -> PDEN_VD5_W<28> {
        PDEN_VD5_W::new(self)
    }
    #[doc = "Bit 29 - Power control for EEPROM."]
    #[inline(always)]
    pub fn pden_vd6(&mut self) -> PDEN_VD6_W<29> {
        PDEN_VD6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power configuration set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgset0](index.html) module"]
pub struct PDRUNCFGSET0_SPEC;
impl crate::RegisterSpec for PDRUNCFGSET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdruncfgset0::R](R) reader structure"]
impl crate::Readable for PDRUNCFGSET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdruncfgset0::W](W) writer structure"]
impl crate::Writable for PDRUNCFGSET0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDRUNCFGSET0 to value 0"]
impl crate::Resettable for PDRUNCFGSET0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
