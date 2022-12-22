#[doc = "Register `SYSAHBCLKCTRL` reader"]
pub struct R(crate::R<SYSAHBCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSAHBCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSAHBCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSAHBCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSAHBCLKCTRL` writer"]
pub struct W(crate::W<SYSAHBCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSAHBCLKCTRL_SPEC>;
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
impl From<crate::W<SYSAHBCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSAHBCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS` reader - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
pub type SYS_R = crate::BitReader<bool>;
#[doc = "Field `SYS` writer - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
pub type SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, bool, O>;
#[doc = "Field `ROM` reader - Enables clock for ROM."]
pub type ROM_R = crate::BitReader<ROM_A>;
#[doc = "Enables clock for ROM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "0: Disable."]
    ROM_0 = 0,
    #[doc = "1: Enable."]
    ROM_1 = 1,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::ROM_0,
            true => ROM_A::ROM_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROM_0`"]
    #[inline(always)]
    pub fn is_rom_0(&self) -> bool {
        *self == ROM_A::ROM_0
    }
    #[doc = "Checks if the value of the field is `ROM_1`"]
    #[inline(always)]
    pub fn is_rom_1(&self) -> bool {
        *self == ROM_A::ROM_1
    }
}
#[doc = "Field `ROM` writer - Enables clock for ROM."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, ROM_A, O>;
impl<'a, const O: u8> ROM_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn rom_0(self) -> &'a mut W {
        self.variant(ROM_A::ROM_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn rom_1(self) -> &'a mut W {
        self.variant(ROM_A::ROM_1)
    }
}
#[doc = "Field `RAM0_1` reader - Enables clock for SRAM0 and SRAM1."]
pub type RAM0_1_R = crate::BitReader<RAM0_1_A>;
#[doc = "Enables clock for SRAM0 and SRAM1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_1_A {
    #[doc = "0: Disable."]
    RAM0_1_0 = 0,
    #[doc = "1: Enable."]
    RAM0_1_1 = 1,
}
impl From<RAM0_1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_1_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_1_A {
        match self.bits {
            false => RAM0_1_A::RAM0_1_0,
            true => RAM0_1_A::RAM0_1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0_1_0`"]
    #[inline(always)]
    pub fn is_ram0_1_0(&self) -> bool {
        *self == RAM0_1_A::RAM0_1_0
    }
    #[doc = "Checks if the value of the field is `RAM0_1_1`"]
    #[inline(always)]
    pub fn is_ram0_1_1(&self) -> bool {
        *self == RAM0_1_A::RAM0_1_1
    }
}
#[doc = "Field `RAM0_1` writer - Enables clock for SRAM0 and SRAM1."]
pub type RAM0_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, RAM0_1_A, O>;
impl<'a, const O: u8> RAM0_1_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn ram0_1_0(self) -> &'a mut W {
        self.variant(RAM0_1_A::RAM0_1_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn ram0_1_1(self) -> &'a mut W {
        self.variant(RAM0_1_A::RAM0_1_1)
    }
}
#[doc = "Field `FLASHREG` reader - Enables clock for flash register interface."]
pub type FLASHREG_R = crate::BitReader<FLASHREG_A>;
#[doc = "Enables clock for flash register interface.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREG_A {
    #[doc = "0: Disable."]
    FLASHREG_0 = 0,
    #[doc = "1: Enable."]
    FLASHREG_1 = 1,
}
impl From<FLASHREG_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHREG_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHREG_A {
        match self.bits {
            false => FLASHREG_A::FLASHREG_0,
            true => FLASHREG_A::FLASHREG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLASHREG_0`"]
    #[inline(always)]
    pub fn is_flashreg_0(&self) -> bool {
        *self == FLASHREG_A::FLASHREG_0
    }
    #[doc = "Checks if the value of the field is `FLASHREG_1`"]
    #[inline(always)]
    pub fn is_flashreg_1(&self) -> bool {
        *self == FLASHREG_A::FLASHREG_1
    }
}
#[doc = "Field `FLASHREG` writer - Enables clock for flash register interface."]
pub type FLASHREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, FLASHREG_A, O>;
impl<'a, const O: u8> FLASHREG_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn flashreg_0(self) -> &'a mut W {
        self.variant(FLASHREG_A::FLASHREG_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn flashreg_1(self) -> &'a mut W {
        self.variant(FLASHREG_A::FLASHREG_1)
    }
}
#[doc = "Field `FLASH` reader - Enables clock for flash."]
pub type FLASH_R = crate::BitReader<FLASH_A>;
#[doc = "Enables clock for flash.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "0: Disable."]
    FLASH_0 = 0,
    #[doc = "1: Enable."]
    FLASH_1 = 1,
}
impl From<FLASH_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::FLASH_0,
            true => FLASH_A::FLASH_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_0`"]
    #[inline(always)]
    pub fn is_flash_0(&self) -> bool {
        *self == FLASH_A::FLASH_0
    }
    #[doc = "Checks if the value of the field is `FLASH_1`"]
    #[inline(always)]
    pub fn is_flash_1(&self) -> bool {
        *self == FLASH_A::FLASH_1
    }
}
#[doc = "Field `FLASH` writer - Enables clock for flash."]
pub type FLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, FLASH_A, O>;
impl<'a, const O: u8> FLASH_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn flash_0(self) -> &'a mut W {
        self.variant(FLASH_A::FLASH_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn flash_1(self) -> &'a mut W {
        self.variant(FLASH_A::FLASH_1)
    }
}
#[doc = "Field `I2C0` reader - Enables clock for I2C0."]
pub type I2C0_R = crate::BitReader<I2C0_A>;
#[doc = "Enables clock for I2C0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_A {
    #[doc = "0: Disable."]
    I2C0_0 = 0,
    #[doc = "1: Enable."]
    I2C0_1 = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::I2C0_0,
            true => I2C0_A::I2C0_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C0_0`"]
    #[inline(always)]
    pub fn is_i2c0_0(&self) -> bool {
        *self == I2C0_A::I2C0_0
    }
    #[doc = "Checks if the value of the field is `I2C0_1`"]
    #[inline(always)]
    pub fn is_i2c0_1(&self) -> bool {
        *self == I2C0_A::I2C0_1
    }
}
#[doc = "Field `I2C0` writer - Enables clock for I2C0."]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, I2C0_A, O>;
impl<'a, const O: u8> I2C0_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn i2c0_0(self) -> &'a mut W {
        self.variant(I2C0_A::I2C0_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn i2c0_1(self) -> &'a mut W {
        self.variant(I2C0_A::I2C0_1)
    }
}
#[doc = "Field `GPIO` reader - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
pub type GPIO_R = crate::BitReader<GPIO_A>;
#[doc = "Enables clock for GPIO port registers and GPIO pin interrupt registers.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_A {
    #[doc = "0: Disable."]
    GPIO_0 = 0,
    #[doc = "1: Enable."]
    GPIO_1 = 1,
}
impl From<GPIO_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_A {
        match self.bits {
            false => GPIO_A::GPIO_0,
            true => GPIO_A::GPIO_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_0`"]
    #[inline(always)]
    pub fn is_gpio_0(&self) -> bool {
        *self == GPIO_A::GPIO_0
    }
    #[doc = "Checks if the value of the field is `GPIO_1`"]
    #[inline(always)]
    pub fn is_gpio_1(&self) -> bool {
        *self == GPIO_A::GPIO_1
    }
}
#[doc = "Field `GPIO` writer - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, GPIO_A, O>;
impl<'a, const O: u8> GPIO_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn gpio_0(self) -> &'a mut W {
        self.variant(GPIO_A::GPIO_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn gpio_1(self) -> &'a mut W {
        self.variant(GPIO_A::GPIO_1)
    }
}
#[doc = "Field `SWM` reader - Enables clock for switch matrix."]
pub type SWM_R = crate::BitReader<SWM_A>;
#[doc = "Enables clock for switch matrix.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWM_A {
    #[doc = "0: Disable."]
    SWM_0 = 0,
    #[doc = "1: Enable."]
    SWM_1 = 1,
}
impl From<SWM_A> for bool {
    #[inline(always)]
    fn from(variant: SWM_A) -> Self {
        variant as u8 != 0
    }
}
impl SWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWM_A {
        match self.bits {
            false => SWM_A::SWM_0,
            true => SWM_A::SWM_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWM_0`"]
    #[inline(always)]
    pub fn is_swm_0(&self) -> bool {
        *self == SWM_A::SWM_0
    }
    #[doc = "Checks if the value of the field is `SWM_1`"]
    #[inline(always)]
    pub fn is_swm_1(&self) -> bool {
        *self == SWM_A::SWM_1
    }
}
#[doc = "Field `SWM` writer - Enables clock for switch matrix."]
pub type SWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SWM_A, O>;
impl<'a, const O: u8> SWM_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn swm_0(self) -> &'a mut W {
        self.variant(SWM_A::SWM_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn swm_1(self) -> &'a mut W {
        self.variant(SWM_A::SWM_1)
    }
}
#[doc = "Field `SCT` reader - Enables clock for state configurable timer SCTimer/PWM."]
pub type SCT_R = crate::BitReader<SCT_A>;
#[doc = "Enables clock for state configurable timer SCTimer/PWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_A {
    #[doc = "0: Disable."]
    SCT_0 = 0,
    #[doc = "1: Enable."]
    SCT_1 = 1,
}
impl From<SCT_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_A {
        match self.bits {
            false => SCT_A::SCT_0,
            true => SCT_A::SCT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCT_0`"]
    #[inline(always)]
    pub fn is_sct_0(&self) -> bool {
        *self == SCT_A::SCT_0
    }
    #[doc = "Checks if the value of the field is `SCT_1`"]
    #[inline(always)]
    pub fn is_sct_1(&self) -> bool {
        *self == SCT_A::SCT_1
    }
}
#[doc = "Field `SCT` writer - Enables clock for state configurable timer SCTimer/PWM."]
pub type SCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SCT_A, O>;
impl<'a, const O: u8> SCT_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn sct_0(self) -> &'a mut W {
        self.variant(SCT_A::SCT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn sct_1(self) -> &'a mut W {
        self.variant(SCT_A::SCT_1)
    }
}
#[doc = "Field `WKT` reader - Enables clock for self-wake-up timer."]
pub type WKT_R = crate::BitReader<WKT_A>;
#[doc = "Enables clock for self-wake-up timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_A {
    #[doc = "0: Disable."]
    WKT_0 = 0,
    #[doc = "1: Enable."]
    WKT_1 = 1,
}
impl From<WKT_A> for bool {
    #[inline(always)]
    fn from(variant: WKT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKT_A {
        match self.bits {
            false => WKT_A::WKT_0,
            true => WKT_A::WKT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKT_0`"]
    #[inline(always)]
    pub fn is_wkt_0(&self) -> bool {
        *self == WKT_A::WKT_0
    }
    #[doc = "Checks if the value of the field is `WKT_1`"]
    #[inline(always)]
    pub fn is_wkt_1(&self) -> bool {
        *self == WKT_A::WKT_1
    }
}
#[doc = "Field `WKT` writer - Enables clock for self-wake-up timer."]
pub type WKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, WKT_A, O>;
impl<'a, const O: u8> WKT_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn wkt_0(self) -> &'a mut W {
        self.variant(WKT_A::WKT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn wkt_1(self) -> &'a mut W {
        self.variant(WKT_A::WKT_1)
    }
}
#[doc = "Field `MRT` reader - Enables clock for multi-rate timer."]
pub type MRT_R = crate::BitReader<MRT_A>;
#[doc = "Enables clock for multi-rate timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_A {
    #[doc = "0: Disable."]
    MRT_0 = 0,
    #[doc = "1: Enable."]
    MRT_1 = 1,
}
impl From<MRT_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_A) -> Self {
        variant as u8 != 0
    }
}
impl MRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_A {
        match self.bits {
            false => MRT_A::MRT_0,
            true => MRT_A::MRT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRT_0`"]
    #[inline(always)]
    pub fn is_mrt_0(&self) -> bool {
        *self == MRT_A::MRT_0
    }
    #[doc = "Checks if the value of the field is `MRT_1`"]
    #[inline(always)]
    pub fn is_mrt_1(&self) -> bool {
        *self == MRT_A::MRT_1
    }
}
#[doc = "Field `MRT` writer - Enables clock for multi-rate timer."]
pub type MRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, MRT_A, O>;
impl<'a, const O: u8> MRT_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn mrt_0(self) -> &'a mut W {
        self.variant(MRT_A::MRT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn mrt_1(self) -> &'a mut W {
        self.variant(MRT_A::MRT_1)
    }
}
#[doc = "Field `SPI0` reader - Enables clock for SPI0."]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "Enables clock for SPI0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Disable."]
    SPI0_0 = 0,
    #[doc = "1: Enable."]
    SPI0_1 = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::SPI0_0,
            true => SPI0_A::SPI0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_0`"]
    #[inline(always)]
    pub fn is_spi0_0(&self) -> bool {
        *self == SPI0_A::SPI0_0
    }
    #[doc = "Checks if the value of the field is `SPI0_1`"]
    #[inline(always)]
    pub fn is_spi0_1(&self) -> bool {
        *self == SPI0_A::SPI0_1
    }
}
#[doc = "Field `SPI0` writer - Enables clock for SPI0."]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SPI0_A, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn spi0_0(self) -> &'a mut W {
        self.variant(SPI0_A::SPI0_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn spi0_1(self) -> &'a mut W {
        self.variant(SPI0_A::SPI0_1)
    }
}
#[doc = "Field `SPI1` reader - Enables clock for SPI1."]
pub type SPI1_R = crate::BitReader<SPI1_A>;
#[doc = "Enables clock for SPI1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Disable."]
    SPI1_0 = 0,
    #[doc = "1: Enable."]
    SPI1_1 = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::SPI1_0,
            true => SPI1_A::SPI1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_0`"]
    #[inline(always)]
    pub fn is_spi1_0(&self) -> bool {
        *self == SPI1_A::SPI1_0
    }
    #[doc = "Checks if the value of the field is `SPI1_1`"]
    #[inline(always)]
    pub fn is_spi1_1(&self) -> bool {
        *self == SPI1_A::SPI1_1
    }
}
#[doc = "Field `SPI1` writer - Enables clock for SPI1."]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SPI1_A, O>;
impl<'a, const O: u8> SPI1_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn spi1_0(self) -> &'a mut W {
        self.variant(SPI1_A::SPI1_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn spi1_1(self) -> &'a mut W {
        self.variant(SPI1_A::SPI1_1)
    }
}
#[doc = "Field `CRC` reader - Enables clock for CRC."]
pub type CRC_R = crate::BitReader<CRC_A>;
#[doc = "Enables clock for CRC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: Disable."]
    CRC_0 = 0,
    #[doc = "1: Enable."]
    CRC_1 = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::CRC_0,
            true => CRC_A::CRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_0`"]
    #[inline(always)]
    pub fn is_crc_0(&self) -> bool {
        *self == CRC_A::CRC_0
    }
    #[doc = "Checks if the value of the field is `CRC_1`"]
    #[inline(always)]
    pub fn is_crc_1(&self) -> bool {
        *self == CRC_A::CRC_1
    }
}
#[doc = "Field `CRC` writer - Enables clock for CRC."]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CRC_A, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn crc_0(self) -> &'a mut W {
        self.variant(CRC_A::CRC_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn crc_1(self) -> &'a mut W {
        self.variant(CRC_A::CRC_1)
    }
}
#[doc = "Field `UART0` reader - Enables clock for USART0."]
pub type UART0_R = crate::BitReader<UART0_A>;
#[doc = "Enables clock for USART0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_A {
    #[doc = "0: Disable."]
    UART0_0 = 0,
    #[doc = "1: Enable."]
    UART0_1 = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
impl UART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::UART0_0,
            true => UART0_A::UART0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_0`"]
    #[inline(always)]
    pub fn is_uart0_0(&self) -> bool {
        *self == UART0_A::UART0_0
    }
    #[doc = "Checks if the value of the field is `UART0_1`"]
    #[inline(always)]
    pub fn is_uart0_1(&self) -> bool {
        *self == UART0_A::UART0_1
    }
}
#[doc = "Field `UART0` writer - Enables clock for USART0."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, UART0_A, O>;
impl<'a, const O: u8> UART0_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn uart0_0(self) -> &'a mut W {
        self.variant(UART0_A::UART0_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn uart0_1(self) -> &'a mut W {
        self.variant(UART0_A::UART0_1)
    }
}
#[doc = "Field `UART1` reader - Enables clock for USART1."]
pub type UART1_R = crate::BitReader<UART1_A>;
#[doc = "Enables clock for USART1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_A {
    #[doc = "0: Disable."]
    UART1_0 = 0,
    #[doc = "1: Enable."]
    UART1_1 = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
impl UART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::UART1_0,
            true => UART1_A::UART1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_0`"]
    #[inline(always)]
    pub fn is_uart1_0(&self) -> bool {
        *self == UART1_A::UART1_0
    }
    #[doc = "Checks if the value of the field is `UART1_1`"]
    #[inline(always)]
    pub fn is_uart1_1(&self) -> bool {
        *self == UART1_A::UART1_1
    }
}
#[doc = "Field `UART1` writer - Enables clock for USART1."]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, UART1_A, O>;
impl<'a, const O: u8> UART1_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn uart1_0(self) -> &'a mut W {
        self.variant(UART1_A::UART1_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn uart1_1(self) -> &'a mut W {
        self.variant(UART1_A::UART1_1)
    }
}
#[doc = "Field `UART2` reader - Enables clock for USART2."]
pub type UART2_R = crate::BitReader<UART2_A>;
#[doc = "Enables clock for USART2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_A {
    #[doc = "0: Disable."]
    UART2_0 = 0,
    #[doc = "1: Enable."]
    UART2_1 = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
impl UART2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::UART2_0,
            true => UART2_A::UART2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART2_0`"]
    #[inline(always)]
    pub fn is_uart2_0(&self) -> bool {
        *self == UART2_A::UART2_0
    }
    #[doc = "Checks if the value of the field is `UART2_1`"]
    #[inline(always)]
    pub fn is_uart2_1(&self) -> bool {
        *self == UART2_A::UART2_1
    }
}
#[doc = "Field `UART2` writer - Enables clock for USART2."]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, UART2_A, O>;
impl<'a, const O: u8> UART2_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn uart2_0(self) -> &'a mut W {
        self.variant(UART2_A::UART2_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn uart2_1(self) -> &'a mut W {
        self.variant(UART2_A::UART2_1)
    }
}
#[doc = "Field `WWDT` reader - Enables clock for WWDT."]
pub type WWDT_R = crate::BitReader<WWDT_A>;
#[doc = "Enables clock for WWDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_A {
    #[doc = "0: Disable."]
    WWDT_0 = 0,
    #[doc = "1: Enable."]
    WWDT_1 = 1,
}
impl From<WWDT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::WWDT_0,
            true => WWDT_A::WWDT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WWDT_0`"]
    #[inline(always)]
    pub fn is_wwdt_0(&self) -> bool {
        *self == WWDT_A::WWDT_0
    }
    #[doc = "Checks if the value of the field is `WWDT_1`"]
    #[inline(always)]
    pub fn is_wwdt_1(&self) -> bool {
        *self == WWDT_A::WWDT_1
    }
}
#[doc = "Field `WWDT` writer - Enables clock for WWDT."]
pub type WWDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, WWDT_A, O>;
impl<'a, const O: u8> WWDT_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn wwdt_0(self) -> &'a mut W {
        self.variant(WWDT_A::WWDT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn wwdt_1(self) -> &'a mut W {
        self.variant(WWDT_A::WWDT_1)
    }
}
#[doc = "Field `IOCON` reader - Enables clock for IOCON block."]
pub type IOCON_R = crate::BitReader<IOCON_A>;
#[doc = "Enables clock for IOCON block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "0: Disable."]
    IOCON_0 = 0,
    #[doc = "1: Enable."]
    IOCON_1 = 1,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::IOCON_0,
            true => IOCON_A::IOCON_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOCON_0`"]
    #[inline(always)]
    pub fn is_iocon_0(&self) -> bool {
        *self == IOCON_A::IOCON_0
    }
    #[doc = "Checks if the value of the field is `IOCON_1`"]
    #[inline(always)]
    pub fn is_iocon_1(&self) -> bool {
        *self == IOCON_A::IOCON_1
    }
}
#[doc = "Field `IOCON` writer - Enables clock for IOCON block."]
pub type IOCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, IOCON_A, O>;
impl<'a, const O: u8> IOCON_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn iocon_0(self) -> &'a mut W {
        self.variant(IOCON_A::IOCON_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn iocon_1(self) -> &'a mut W {
        self.variant(IOCON_A::IOCON_1)
    }
}
#[doc = "Field `ACMP` reader - Enables clock to analog comparator."]
pub type ACMP_R = crate::BitReader<ACMP_A>;
#[doc = "Enables clock to analog comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_A {
    #[doc = "0: Disable."]
    ACMP_0 = 0,
    #[doc = "1: Enable."]
    ACMP_1 = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::ACMP_0,
            true => ACMP_A::ACMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_0`"]
    #[inline(always)]
    pub fn is_acmp_0(&self) -> bool {
        *self == ACMP_A::ACMP_0
    }
    #[doc = "Checks if the value of the field is `ACMP_1`"]
    #[inline(always)]
    pub fn is_acmp_1(&self) -> bool {
        *self == ACMP_A::ACMP_1
    }
}
#[doc = "Field `ACMP` writer - Enables clock to analog comparator."]
pub type ACMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, ACMP_A, O>;
impl<'a, const O: u8> ACMP_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn acmp_0(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn acmp_1(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables clock for SRAM0 and SRAM1."]
    #[inline(always)]
    pub fn ram0_1(&self) -> RAM0_1_R {
        RAM0_1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FLASHREG_R {
        FLASHREG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables clock for flash."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables clock for I2C0."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables clock for switch matrix."]
    #[inline(always)]
    pub fn swm(&self) -> SWM_R {
        SWM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables clock for state configurable timer SCTimer/PWM."]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables clock for self-wake-up timer."]
    #[inline(always)]
    pub fn wkt(&self) -> WKT_R {
        WKT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables clock for multi-rate timer."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables clock for SPI1."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables clock for CRC."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables clock for USART0."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables clock for USART1."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables clock for USART2."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables clock for IOCON block."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables clock to analog comparator."]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W<0> {
        SYS_W::new(self)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<1> {
        ROM_W::new(self)
    }
    #[doc = "Bit 2 - Enables clock for SRAM0 and SRAM1."]
    #[inline(always)]
    pub fn ram0_1(&mut self) -> RAM0_1_W<2> {
        RAM0_1_W::new(self)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&mut self) -> FLASHREG_W<3> {
        FLASHREG_W::new(self)
    }
    #[doc = "Bit 4 - Enables clock for flash."]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W<4> {
        FLASH_W::new(self)
    }
    #[doc = "Bit 5 - Enables clock for I2C0."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W<5> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W<6> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 7 - Enables clock for switch matrix."]
    #[inline(always)]
    pub fn swm(&mut self) -> SWM_W<7> {
        SWM_W::new(self)
    }
    #[doc = "Bit 8 - Enables clock for state configurable timer SCTimer/PWM."]
    #[inline(always)]
    pub fn sct(&mut self) -> SCT_W<8> {
        SCT_W::new(self)
    }
    #[doc = "Bit 9 - Enables clock for self-wake-up timer."]
    #[inline(always)]
    pub fn wkt(&mut self) -> WKT_W<9> {
        WKT_W::new(self)
    }
    #[doc = "Bit 10 - Enables clock for multi-rate timer."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W<10> {
        MRT_W::new(self)
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W<11> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 12 - Enables clock for SPI1."]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<12> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 13 - Enables clock for CRC."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<13> {
        CRC_W::new(self)
    }
    #[doc = "Bit 14 - Enables clock for USART0."]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<14> {
        UART0_W::new(self)
    }
    #[doc = "Bit 15 - Enables clock for USART1."]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W<15> {
        UART1_W::new(self)
    }
    #[doc = "Bit 16 - Enables clock for USART2."]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W<16> {
        UART2_W::new(self)
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W<17> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 18 - Enables clock for IOCON block."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W<18> {
        IOCON_W::new(self)
    }
    #[doc = "Bit 19 - Enables clock to analog comparator."]
    #[inline(always)]
    pub fn acmp(&mut self) -> ACMP_W<19> {
        ACMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl](index.html) module"]
pub struct SYSAHBCLKCTRL_SPEC;
impl crate::RegisterSpec for SYSAHBCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysahbclkctrl::R](R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl::W](W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSAHBCLKCTRL to value 0xdf"]
impl crate::Resettable for SYSAHBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xdf
    }
}
