#[doc = "Register `AHBCLKCTRL0` reader"]
pub struct R(crate::R<AHBCLKCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL0` writer"]
pub struct W(crate::W<AHBCLKCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL0_SPEC>;
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
impl From<crate::W<AHBCLKCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM` reader - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
pub type ROM_R = crate::BitReader<bool>;
#[doc = "Field `ROM` writer - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `SRAM1` reader - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
pub type SRAM1_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1` writer - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
pub type SRAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `SRAM2` reader - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
pub type SRAM2_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2` writer - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
pub type SRAM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `SRAM3` reader - Enables the clock for SRAM3."]
pub type SRAM3_R = crate::BitReader<bool>;
#[doc = "Field `SRAM3` writer - Enables the clock for SRAM3."]
pub type SRAM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `FLASH` reader - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
pub type FLASH_R = crate::BitReader<bool>;
#[doc = "Field `FLASH` writer - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
pub type FLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `FMC` reader - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
pub type FMC_R = crate::BitReader<bool>;
#[doc = "Field `FMC` writer - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
pub type FMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `EEPROM` reader - Enables the clock for EEPROM."]
pub type EEPROM_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM` writer - Enables the clock for EEPROM."]
pub type EEPROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `SPIFI` reader - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
pub type SPIFI_R = crate::BitReader<bool>;
#[doc = "Field `SPIFI` writer - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
pub type SPIFI_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `INPUTMUX` reader - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
pub type INPUTMUX_R = crate::BitReader<bool>;
#[doc = "Field `INPUTMUX` writer - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
pub type INPUTMUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `IOCON` reader - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
pub type IOCON_R = crate::BitReader<bool>;
#[doc = "Field `IOCON` writer - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
pub type IOCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `GPIO0` reader - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
pub type GPIO0_R = crate::BitReader<bool>;
#[doc = "Field `GPIO0` writer - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
pub type GPIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `GPIO1` reader - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
pub type GPIO1_R = crate::BitReader<bool>;
#[doc = "Field `GPIO1` writer - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
pub type GPIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `GPIO2` reader - Enables the clock for the GPIO2 port registers."]
pub type GPIO2_R = crate::BitReader<bool>;
#[doc = "Field `GPIO2` writer - Enables the clock for the GPIO2 port registers."]
pub type GPIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `GPIO3` reader - Enables the clock for the GPIO3 port registers."]
pub type GPIO3_R = crate::BitReader<bool>;
#[doc = "Field `GPIO3` writer - Enables the clock for the GPIO3 port registers."]
pub type GPIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `PINT` reader - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
pub type PINT_R = crate::BitReader<bool>;
#[doc = "Field `PINT` writer - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
pub type PINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `GINT` reader - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
pub type GINT_R = crate::BitReader<bool>;
#[doc = "Field `GINT` writer - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
pub type GINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `DMA` reader - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `CRC` reader - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
pub type CRC_R = crate::BitReader<bool>;
#[doc = "Field `CRC` writer - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `WWDT` reader - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
pub type WWDT_R = crate::BitReader<bool>;
#[doc = "Field `WWDT` writer - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
pub type WWDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `RTC` reader - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
#[doc = "Field `ADC0` reader - Enables the clock for the ADC0 register interface."]
pub type ADC0_R = crate::BitReader<bool>;
#[doc = "Field `ADC0` writer - Enables the clock for the ADC0 register interface."]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for SRAM3."]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
    #[inline(always)]
    pub fn fmc(&self) -> FMC_R {
        FMC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for EEPROM."]
    #[inline(always)]
    pub fn eeprom(&self) -> EEPROM_R {
        EEPROM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn spifi(&self) -> SPIFI_R {
        SPIFI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn inputmux(&self) -> INPUTMUX_R {
        INPUTMUX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2 port registers."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3 port registers."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC0 register interface."]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<1> {
        ROM_W::new(self)
    }
    #[doc = "Bit 3 - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W<3> {
        SRAM1_W::new(self)
    }
    #[doc = "Bit 4 - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W<4> {
        SRAM2_W::new(self)
    }
    #[doc = "Bit 5 - Enables the clock for SRAM3."]
    #[inline(always)]
    pub fn sram3(&mut self) -> SRAM3_W<5> {
        SRAM3_W::new(self)
    }
    #[doc = "Bit 7 - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W<7> {
        FLASH_W::new(self)
    }
    #[doc = "Bit 8 - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
    #[inline(always)]
    pub fn fmc(&mut self) -> FMC_W<8> {
        FMC_W::new(self)
    }
    #[doc = "Bit 9 - Enables the clock for EEPROM."]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EEPROM_W<9> {
        EEPROM_W::new(self)
    }
    #[doc = "Bit 10 - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn spifi(&mut self) -> SPIFI_W<10> {
        SPIFI_W::new(self)
    }
    #[doc = "Bit 11 - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn inputmux(&mut self) -> INPUTMUX_W<11> {
        INPUTMUX_W::new(self)
    }
    #[doc = "Bit 13 - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W<13> {
        IOCON_W::new(self)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W<14> {
        GPIO0_W::new(self)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W<15> {
        GPIO1_W::new(self)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2 port registers."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W<16> {
        GPIO2_W::new(self)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3 port registers."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W<17> {
        GPIO3_W::new(self)
    }
    #[doc = "Bit 18 - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn pint(&mut self) -> PINT_W<18> {
        PINT_W::new(self)
    }
    #[doc = "Bit 19 - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W<19> {
        GINT_W::new(self)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<20> {
        DMA_W::new(self)
    }
    #[doc = "Bit 21 - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<21> {
        CRC_W::new(self)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W<22> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 23 - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<23> {
        RTC_W::new(self)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC0 register interface."]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W<27> {
        ADC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl0](index.html) module"]
pub struct AHBCLKCTRL0_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl0::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl0::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL0 to value 0x0183"]
impl crate::Resettable for AHBCLKCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0183
    }
}
