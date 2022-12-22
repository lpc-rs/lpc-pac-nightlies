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
#[doc = "Field `SYS` reader - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M3 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
pub type SYS_R = crate::BitReader<SYS_A>;
#[doc = "Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M3 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_A {
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SYS_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_A) -> Self {
        variant as u8 != 0
    }
}
impl SYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_A {
        match self.bits {
            true => SYS_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYS_A::ENABLED
    }
}
#[doc = "Field `SYS` writer - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M3 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
pub type SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SYS_A, O>;
impl<'a, const O: u8> SYS_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYS_A::ENABLED)
    }
}
#[doc = "Field `ROM` reader - Enables clock for ROM."]
pub type ROM_R = crate::BitReader<ROM_A>;
#[doc = "Enables clock for ROM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => ROM_A::DISABLED,
            true => ROM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROM_A::ENABLED
    }
}
#[doc = "Field `ROM` writer - Enables clock for ROM."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, ROM_A, O>;
impl<'a, const O: u8> ROM_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROM_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROM_A::ENABLED)
    }
}
#[doc = "Field `RAM` reader - Enables clock for RAM."]
pub type RAM_R = crate::BitReader<RAM_A>;
#[doc = "Enables clock for RAM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<RAM_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_A {
        match self.bits {
            false => RAM_A::DISABLED,
            true => RAM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAM_A::ENABLED
    }
}
#[doc = "Field `RAM` writer - Enables clock for RAM."]
pub type RAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, RAM_A, O>;
impl<'a, const O: u8> RAM_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAM_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAM_A::ENABLED)
    }
}
#[doc = "Field `FLASHREG` reader - Enables clock for flash register interface."]
pub type FLASHREG_R = crate::BitReader<FLASHREG_A>;
#[doc = "Enables clock for flash register interface.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREG_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => FLASHREG_A::DISABLED,
            true => FLASHREG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHREG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHREG_A::ENABLED
    }
}
#[doc = "Field `FLASHREG` writer - Enables clock for flash register interface."]
pub type FLASHREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, FLASHREG_A, O>;
impl<'a, const O: u8> FLASHREG_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::ENABLED)
    }
}
#[doc = "Field `FLASHARRAY` reader - Enables clock for flash array access."]
pub type FLASHARRAY_R = crate::BitReader<FLASHARRAY_A>;
#[doc = "Enables clock for flash array access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHARRAY_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<FLASHARRAY_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHARRAY_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHARRAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHARRAY_A {
        match self.bits {
            false => FLASHARRAY_A::DISABLED,
            true => FLASHARRAY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHARRAY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHARRAY_A::ENABLED
    }
}
#[doc = "Field `FLASHARRAY` writer - Enables clock for flash array access."]
pub type FLASHARRAY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, FLASHARRAY_A, O>;
impl<'a, const O: u8> FLASHARRAY_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::ENABLED)
    }
}
#[doc = "Field `I2C` reader - Enables clock for I2C."]
pub type I2C_R = crate::BitReader<I2C_A>;
#[doc = "Enables clock for I2C.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<I2C_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_A {
        match self.bits {
            false => I2C_A::DISABLED,
            true => I2C_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C_A::ENABLED
    }
}
#[doc = "Field `I2C` writer - Enables clock for I2C."]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, I2C_A, O>;
impl<'a, const O: u8> I2C_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C_A::ENABLED)
    }
}
#[doc = "Field `GPIO` reader - Enables clock for GPIO."]
pub type GPIO_R = crate::BitReader<GPIO_A>;
#[doc = "Enables clock for GPIO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => GPIO_A::DISABLED,
            true => GPIO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIO_A::ENABLED
    }
}
#[doc = "Field `GPIO` writer - Enables clock for GPIO."]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, GPIO_A, O>;
impl<'a, const O: u8> GPIO_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO_A::ENABLED)
    }
}
#[doc = "Field `CT16B0` reader - Enables clock for 16-bit counter/timer 0."]
pub type CT16B0_R = crate::BitReader<CT16B0_A>;
#[doc = "Enables clock for 16-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CT16B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B0_A) -> Self {
        variant as u8 != 0
    }
}
impl CT16B0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B0_A {
        match self.bits {
            false => CT16B0_A::DISABLED,
            true => CT16B0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT16B0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT16B0_A::ENABLED
    }
}
#[doc = "Field `CT16B0` writer - Enables clock for 16-bit counter/timer 0."]
pub type CT16B0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT16B0_A, O>;
impl<'a, const O: u8> CT16B0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B0_A::ENABLED)
    }
}
#[doc = "Field `CT16B1` reader - Enables clock for 16-bit counter/timer 1."]
pub type CT16B1_R = crate::BitReader<CT16B1_A>;
#[doc = "Enables clock for 16-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CT16B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B1_A) -> Self {
        variant as u8 != 0
    }
}
impl CT16B1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B1_A {
        match self.bits {
            false => CT16B1_A::DISABLED,
            true => CT16B1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT16B1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT16B1_A::ENABLED
    }
}
#[doc = "Field `CT16B1` writer - Enables clock for 16-bit counter/timer 1."]
pub type CT16B1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT16B1_A, O>;
impl<'a, const O: u8> CT16B1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B1_A::ENABLED)
    }
}
#[doc = "Field `CT32B0` reader - Enables clock for 32-bit counter/timer 0."]
pub type CT32B0_R = crate::BitReader<CT32B0_A>;
#[doc = "Enables clock for 32-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CT32B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B0_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32B0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B0_A {
        match self.bits {
            false => CT32B0_A::DISABLED,
            true => CT32B0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT32B0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT32B0_A::ENABLED
    }
}
#[doc = "Field `CT32B0` writer - Enables clock for 32-bit counter/timer 0."]
pub type CT32B0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT32B0_A, O>;
impl<'a, const O: u8> CT32B0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B0_A::ENABLED)
    }
}
#[doc = "Field `CT32B1` reader - Enables clock for 32-bit counter/timer 1."]
pub type CT32B1_R = crate::BitReader<CT32B1_A>;
#[doc = "Enables clock for 32-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CT32B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B1_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32B1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B1_A {
        match self.bits {
            false => CT32B1_A::DISABLED,
            true => CT32B1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CT32B1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CT32B1_A::ENABLED
    }
}
#[doc = "Field `CT32B1` writer - Enables clock for 32-bit counter/timer 1."]
pub type CT32B1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, CT32B1_A, O>;
impl<'a, const O: u8> CT32B1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B1_A::ENABLED)
    }
}
#[doc = "Field `SSP` reader - Enables clock for SSP."]
pub type SSP_R = crate::BitReader<SSP_A>;
#[doc = "Enables clock for SSP.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SSP_A> for bool {
    #[inline(always)]
    fn from(variant: SSP_A) -> Self {
        variant as u8 != 0
    }
}
impl SSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP_A {
        match self.bits {
            false => SSP_A::DISABLED,
            true => SSP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSP_A::ENABLED
    }
}
#[doc = "Field `SSP` writer - Enables clock for SSP."]
pub type SSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SSP_A, O>;
impl<'a, const O: u8> SSP_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSP_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSP_A::ENABLED)
    }
}
#[doc = "Field `UART` reader - Enables clock for UART. Note that the UART pins must be configured in the IOCON block before the UART clock can be enabled."]
pub type UART_R = crate::BitReader<UART_A>;
#[doc = "Enables clock for UART. Note that the UART pins must be configured in the IOCON block before the UART clock can be enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<UART_A> for bool {
    #[inline(always)]
    fn from(variant: UART_A) -> Self {
        variant as u8 != 0
    }
}
impl UART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_A {
        match self.bits {
            false => UART_A::DISABLED,
            true => UART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART_A::ENABLED
    }
}
#[doc = "Field `UART` writer - Enables clock for UART. Note that the UART pins must be configured in the IOCON block before the UART clock can be enabled."]
pub type UART_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, UART_A, O>;
impl<'a, const O: u8> UART_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UART_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UART_A::ENABLED)
    }
}
#[doc = "Field `ADC` reader - Enables clock for ADC."]
pub type ADC_R = crate::BitReader<ADC_A>;
#[doc = "Enables clock for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLED,
            true => ADC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_A::ENABLED
    }
}
#[doc = "Field `ADC` writer - Enables clock for ADC."]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, ADC_A, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_A::ENABLED)
    }
}
#[doc = "Field `USB_REG` reader - Enables clock for USB_REG."]
pub type USB_REG_R = crate::BitReader<USB_REG_A>;
#[doc = "Enables clock for USB_REG.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_REG_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USB_REG_A> for bool {
    #[inline(always)]
    fn from(variant: USB_REG_A) -> Self {
        variant as u8 != 0
    }
}
impl USB_REG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_REG_A {
        match self.bits {
            false => USB_REG_A::DISABLED,
            true => USB_REG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USB_REG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USB_REG_A::ENABLED
    }
}
#[doc = "Field `USB_REG` writer - Enables clock for USB_REG."]
pub type USB_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, USB_REG_A, O>;
impl<'a, const O: u8> USB_REG_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB_REG_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB_REG_A::ENABLED)
    }
}
#[doc = "Field `WDT` reader - Enables clock for WDT."]
pub type WDT_R = crate::BitReader<WDT_A>;
#[doc = "Enables clock for WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::DISABLED,
            true => WDT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDT_A::ENABLED
    }
}
#[doc = "Field `WDT` writer - Enables clock for WDT."]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, WDT_A, O>;
impl<'a, const O: u8> WDT_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDT_A::ENABLED)
    }
}
#[doc = "Field `IOCON` reader - Enables clock for IO configuration block."]
pub type IOCON_R = crate::BitReader<IOCON_A>;
#[doc = "Enables clock for IO configuration block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => IOCON_A::DISABLED,
            true => IOCON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOCON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOCON_A::ENABLED
    }
}
#[doc = "Field `IOCON` writer - Enables clock for IO configuration block."]
pub type IOCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, IOCON_A, O>;
impl<'a, const O: u8> IOCON_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLED)
    }
}
#[doc = "Field `SSP1` reader - Enables clock for SPISP1."]
pub type SSP1_R = crate::BitReader<SSP1_A>;
#[doc = "Enables clock for SPISP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SSP1_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_A) -> Self {
        variant as u8 != 0
    }
}
impl SSP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_A {
        match self.bits {
            false => SSP1_A::DISABLE,
            true => SSP1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSP1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSP1_A::ENABLE
    }
}
#[doc = "Field `SSP1` writer - Enables clock for SPISP1."]
pub type SSP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSAHBCLKCTRL_SPEC, SSP1_A, O>;
impl<'a, const O: u8> SSP1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSP1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSP1_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M3 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FLASHREG_R {
        FLASHREG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    pub fn flasharray(&self) -> FLASHARRAY_R {
        FLASHARRAY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables clock for GPIO."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&self) -> CT16B0_R {
        CT16B0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&self) -> CT16B1_R {
        CT16B1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&self) -> CT32B0_R {
        CT32B0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&self) -> CT32B1_R {
        CT32B1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables clock for SSP."]
    #[inline(always)]
    pub fn ssp(&self) -> SSP_R {
        SSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables clock for UART. Note that the UART pins must be configured in the IOCON block before the UART clock can be enabled."]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables clock for USB_REG."]
    #[inline(always)]
    pub fn usb_reg(&self) -> USB_REG_R {
        USB_REG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables clock for WDT."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables clock for IO configuration block."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables clock for SPISP1."]
    #[inline(always)]
    pub fn ssp1(&self) -> SSP1_R {
        SSP1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables clock for AHB to APB bridge, to the AHB matrix, to the Cortex-M3 FCLK and HCLK, to the SysCon, and to the PMU. This bit is read only."]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W<0> {
        SYS_W::new(self)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<1> {
        ROM_W::new(self)
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W<2> {
        RAM_W::new(self)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&mut self) -> FLASHREG_W<3> {
        FLASHREG_W::new(self)
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    pub fn flasharray(&mut self) -> FLASHARRAY_W<4> {
        FLASHARRAY_W::new(self)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W<5> {
        I2C_W::new(self)
    }
    #[doc = "Bit 6 - Enables clock for GPIO."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W<6> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&mut self) -> CT16B0_W<7> {
        CT16B0_W::new(self)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&mut self) -> CT16B1_W<8> {
        CT16B1_W::new(self)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&mut self) -> CT32B0_W<9> {
        CT32B0_W::new(self)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&mut self) -> CT32B1_W<10> {
        CT32B1_W::new(self)
    }
    #[doc = "Bit 11 - Enables clock for SSP."]
    #[inline(always)]
    pub fn ssp(&mut self) -> SSP_W<11> {
        SSP_W::new(self)
    }
    #[doc = "Bit 12 - Enables clock for UART. Note that the UART pins must be configured in the IOCON block before the UART clock can be enabled."]
    #[inline(always)]
    pub fn uart(&mut self) -> UART_W<12> {
        UART_W::new(self)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W<13> {
        ADC_W::new(self)
    }
    #[doc = "Bit 14 - Enables clock for USB_REG."]
    #[inline(always)]
    pub fn usb_reg(&mut self) -> USB_REG_W<14> {
        USB_REG_W::new(self)
    }
    #[doc = "Bit 15 - Enables clock for WDT."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<15> {
        WDT_W::new(self)
    }
    #[doc = "Bit 16 - Enables clock for IO configuration block."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W<16> {
        IOCON_W::new(self)
    }
    #[doc = "Bit 18 - Enables clock for SPISP1."]
    #[inline(always)]
    pub fn ssp1(&mut self) -> SSP1_W<18> {
        SSP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System AHB clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl](index.html) module"]
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
#[doc = "`reset()` method sets SYSAHBCLKCTRL to value 0x485f"]
impl crate::Resettable for SYSAHBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x485f
    }
}
