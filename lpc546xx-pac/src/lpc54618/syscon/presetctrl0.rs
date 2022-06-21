#[doc = "Register `PRESETCTRL0` reader"]
pub struct R(crate::R<PRESETCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL0` writer"]
pub struct W(crate::W<PRESETCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL0_SPEC>;
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
impl From<crate::W<PRESETCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_RST` reader - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FLASH_RST_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_RST` writer - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FLASH_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 7>;
#[doc = "Field `FMC_RST` reader - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FMC_RST_R = crate::BitReader<bool>;
#[doc = "Field `FMC_RST` writer - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type FMC_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 8>;
#[doc = "Field `EEPROM_RST` reader - EEPROM reset control."]
pub type EEPROM_RST_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_RST` writer - EEPROM reset control."]
pub type EEPROM_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 9>;
#[doc = "Field `SPIFI_RST` reader - SPIFI reset control."]
pub type SPIFI_RST_R = crate::BitReader<bool>;
#[doc = "Field `SPIFI_RST` writer - SPIFI reset control."]
pub type SPIFI_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 10>;
#[doc = "Field `MUX_RST` reader - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type MUX_RST_R = crate::BitReader<bool>;
#[doc = "Field `MUX_RST` writer - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type MUX_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 11>;
#[doc = "Field `IOCON_RST` reader - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type IOCON_RST_R = crate::BitReader<bool>;
#[doc = "Field `IOCON_RST` writer - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type IOCON_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 13>;
#[doc = "Field `GPIO0_RST` reader - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type GPIO0_RST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO0_RST` writer - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type GPIO0_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 14>;
#[doc = "Field `GPIO1_RST` reader - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type GPIO1_RST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO1_RST` writer - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type GPIO1_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 15>;
#[doc = "Field `GPIO2_RST` reader - GPIO2 reset control."]
pub type GPIO2_RST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO2_RST` writer - GPIO2 reset control."]
pub type GPIO2_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 16>;
#[doc = "Field `GPIO3_RST` reader - GPIO3 reset control."]
pub type GPIO3_RST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO3_RST` writer - GPIO3 reset control."]
pub type GPIO3_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 17>;
#[doc = "Field `PINT_RST` reader - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type PINT_RST_R = crate::BitReader<bool>;
#[doc = "Field `PINT_RST` writer - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type PINT_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 18>;
#[doc = "Field `GINT_RST` reader - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type GINT_RST_R = crate::BitReader<bool>;
#[doc = "Field `GINT_RST` writer - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type GINT_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 19>;
#[doc = "Field `DMA0_RST` reader - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type DMA0_RST_R = crate::BitReader<bool>;
#[doc = "Field `DMA0_RST` writer - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type DMA0_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 20>;
#[doc = "Field `CRC_RST` reader - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type CRC_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRC_RST` writer - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type CRC_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 21>;
#[doc = "Field `WWDT_RST` reader - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type WWDT_RST_R = crate::BitReader<bool>;
#[doc = "Field `WWDT_RST` writer - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type WWDT_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 22>;
#[doc = "Field `ADC0_RST` reader - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type ADC0_RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC0_RST` writer - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub type ADC0_RST_W<'a> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, bool, 27>;
impl R {
    #[doc = "Bit 7 - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn flash_rst(&self) -> FLASH_RST_R {
        FLASH_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fmc_rst(&self) -> FMC_RST_R {
        FMC_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EEPROM reset control."]
    #[inline(always)]
    pub fn eeprom_rst(&self) -> EEPROM_RST_R {
        EEPROM_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPIFI reset control."]
    #[inline(always)]
    pub fn spifi_rst(&self) -> SPIFI_RST_R {
        SPIFI_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mux_rst(&self) -> MUX_RST_R {
        MUX_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn iocon_rst(&self) -> IOCON_RST_R {
        IOCON_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio0_rst(&self) -> GPIO0_RST_R {
        GPIO0_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio1_rst(&self) -> GPIO1_RST_R {
        GPIO1_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&self) -> GPIO2_RST_R {
        GPIO2_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&self) -> GPIO3_RST_R {
        GPIO3_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn pint_rst(&self) -> PINT_RST_R {
        PINT_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gint_rst(&self) -> GINT_RST_R {
        GINT_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dma0_rst(&self) -> DMA0_RST_R {
        DMA0_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn crc_rst(&self) -> CRC_RST_R {
        CRC_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn wwdt_rst(&self) -> WWDT_RST_R {
        WWDT_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn adc0_rst(&self) -> ADC0_RST_R {
        ADC0_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn flash_rst(&mut self) -> FLASH_RST_W {
        FLASH_RST_W::new(self)
    }
    #[doc = "Bit 8 - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fmc_rst(&mut self) -> FMC_RST_W {
        FMC_RST_W::new(self)
    }
    #[doc = "Bit 9 - EEPROM reset control."]
    #[inline(always)]
    pub fn eeprom_rst(&mut self) -> EEPROM_RST_W {
        EEPROM_RST_W::new(self)
    }
    #[doc = "Bit 10 - SPIFI reset control."]
    #[inline(always)]
    pub fn spifi_rst(&mut self) -> SPIFI_RST_W {
        SPIFI_RST_W::new(self)
    }
    #[doc = "Bit 11 - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mux_rst(&mut self) -> MUX_RST_W {
        MUX_RST_W::new(self)
    }
    #[doc = "Bit 13 - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn iocon_rst(&mut self) -> IOCON_RST_W {
        IOCON_RST_W::new(self)
    }
    #[doc = "Bit 14 - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio0_rst(&mut self) -> GPIO0_RST_W {
        GPIO0_RST_W::new(self)
    }
    #[doc = "Bit 15 - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio1_rst(&mut self) -> GPIO1_RST_W {
        GPIO1_RST_W::new(self)
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&mut self) -> GPIO2_RST_W {
        GPIO2_RST_W::new(self)
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&mut self) -> GPIO3_RST_W {
        GPIO3_RST_W::new(self)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn pint_rst(&mut self) -> PINT_RST_W {
        PINT_RST_W::new(self)
    }
    #[doc = "Bit 19 - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gint_rst(&mut self) -> GINT_RST_W {
        GINT_RST_W::new(self)
    }
    #[doc = "Bit 20 - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dma0_rst(&mut self) -> DMA0_RST_W {
        DMA0_RST_W::new(self)
    }
    #[doc = "Bit 21 - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn crc_rst(&mut self) -> CRC_RST_W {
        CRC_RST_W::new(self)
    }
    #[doc = "Bit 22 - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn wwdt_rst(&mut self) -> WWDT_RST_W {
        WWDT_RST_W::new(self)
    }
    #[doc = "Bit 27 - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn adc0_rst(&mut self) -> ADC0_RST_W {
        ADC0_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl0](index.html) module"]
pub struct PRESETCTRL0_SPEC;
impl crate::RegisterSpec for PRESETCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl0::R](R) reader structure"]
impl crate::Readable for PRESETCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl0::W](W) writer structure"]
impl crate::Writable for PRESETCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL0 to value 0"]
impl crate::Resettable for PRESETCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
