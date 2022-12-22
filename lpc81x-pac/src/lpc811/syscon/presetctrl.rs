#[doc = "Register `PRESETCTRL` reader"]
pub struct R(crate::R<PRESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL` writer"]
pub struct W(crate::W<PRESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_SPEC>;
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
impl From<crate::W<PRESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0_RST_N` reader - SPI0 reset control."]
pub type SPI0_RST_N_R = crate::BitReader<SPI0_RST_N_A>;
#[doc = "SPI0 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_RST_N_A {
    #[doc = "0: Assert the SPI0 reset."]
    SPI0_RST_N_0 = 0,
    #[doc = "1: Clear the SPI0 reset."]
    SPI0_RST_N_1 = 1,
}
impl From<SPI0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_RST_N_A {
        match self.bits {
            false => SPI0_RST_N_A::SPI0_RST_N_0,
            true => SPI0_RST_N_A::SPI0_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_RST_N_0`"]
    #[inline(always)]
    pub fn is_spi0_rst_n_0(&self) -> bool {
        *self == SPI0_RST_N_A::SPI0_RST_N_0
    }
    #[doc = "Checks if the value of the field is `SPI0_RST_N_1`"]
    #[inline(always)]
    pub fn is_spi0_rst_n_1(&self) -> bool {
        *self == SPI0_RST_N_A::SPI0_RST_N_1
    }
}
#[doc = "Field `SPI0_RST_N` writer - SPI0 reset control."]
pub type SPI0_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, SPI0_RST_N_A, O>;
impl<'a, const O: u8> SPI0_RST_N_W<'a, O> {
    #[doc = "Assert the SPI0 reset."]
    #[inline(always)]
    pub fn spi0_rst_n_0(self) -> &'a mut W {
        self.variant(SPI0_RST_N_A::SPI0_RST_N_0)
    }
    #[doc = "Clear the SPI0 reset."]
    #[inline(always)]
    pub fn spi0_rst_n_1(self) -> &'a mut W {
        self.variant(SPI0_RST_N_A::SPI0_RST_N_1)
    }
}
#[doc = "Field `SPI1_RST_N` reader - SPI1 reset control."]
pub type SPI1_RST_N_R = crate::BitReader<SPI1_RST_N_A>;
#[doc = "SPI1 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_RST_N_A {
    #[doc = "0: Assert the SPI1 reset."]
    SPI1_RST_N_0 = 0,
    #[doc = "1: Clear the SPI1 reset."]
    SPI1_RST_N_1 = 1,
}
impl From<SPI1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_RST_N_A {
        match self.bits {
            false => SPI1_RST_N_A::SPI1_RST_N_0,
            true => SPI1_RST_N_A::SPI1_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_RST_N_0`"]
    #[inline(always)]
    pub fn is_spi1_rst_n_0(&self) -> bool {
        *self == SPI1_RST_N_A::SPI1_RST_N_0
    }
    #[doc = "Checks if the value of the field is `SPI1_RST_N_1`"]
    #[inline(always)]
    pub fn is_spi1_rst_n_1(&self) -> bool {
        *self == SPI1_RST_N_A::SPI1_RST_N_1
    }
}
#[doc = "Field `SPI1_RST_N` writer - SPI1 reset control."]
pub type SPI1_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, SPI1_RST_N_A, O>;
impl<'a, const O: u8> SPI1_RST_N_W<'a, O> {
    #[doc = "Assert the SPI1 reset."]
    #[inline(always)]
    pub fn spi1_rst_n_0(self) -> &'a mut W {
        self.variant(SPI1_RST_N_A::SPI1_RST_N_0)
    }
    #[doc = "Clear the SPI1 reset."]
    #[inline(always)]
    pub fn spi1_rst_n_1(self) -> &'a mut W {
        self.variant(SPI1_RST_N_A::SPI1_RST_N_1)
    }
}
#[doc = "Field `UARTFRG_RST_N` reader - USART fractional baud rate generator(UARTFRG) reset control."]
pub type UARTFRG_RST_N_R = crate::BitReader<UARTFRG_RST_N_A>;
#[doc = "USART fractional baud rate generator(UARTFRG) reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTFRG_RST_N_A {
    #[doc = "0: Assert the UARTFRG reset."]
    UARTFRG_RST_N_0 = 0,
    #[doc = "1: Clear the UARTFRG reset."]
    UARTFRG_RST_N_1 = 1,
}
impl From<UARTFRG_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UARTFRG_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl UARTFRG_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTFRG_RST_N_A {
        match self.bits {
            false => UARTFRG_RST_N_A::UARTFRG_RST_N_0,
            true => UARTFRG_RST_N_A::UARTFRG_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UARTFRG_RST_N_0`"]
    #[inline(always)]
    pub fn is_uartfrg_rst_n_0(&self) -> bool {
        *self == UARTFRG_RST_N_A::UARTFRG_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UARTFRG_RST_N_1`"]
    #[inline(always)]
    pub fn is_uartfrg_rst_n_1(&self) -> bool {
        *self == UARTFRG_RST_N_A::UARTFRG_RST_N_1
    }
}
#[doc = "Field `UARTFRG_RST_N` writer - USART fractional baud rate generator(UARTFRG) reset control."]
pub type UARTFRG_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, UARTFRG_RST_N_A, O>;
impl<'a, const O: u8> UARTFRG_RST_N_W<'a, O> {
    #[doc = "Assert the UARTFRG reset."]
    #[inline(always)]
    pub fn uartfrg_rst_n_0(self) -> &'a mut W {
        self.variant(UARTFRG_RST_N_A::UARTFRG_RST_N_0)
    }
    #[doc = "Clear the UARTFRG reset."]
    #[inline(always)]
    pub fn uartfrg_rst_n_1(self) -> &'a mut W {
        self.variant(UARTFRG_RST_N_A::UARTFRG_RST_N_1)
    }
}
#[doc = "Field `UART0_RST_N` reader - USART0 reset control."]
pub type UART0_RST_N_R = crate::BitReader<UART0_RST_N_A>;
#[doc = "USART0 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_RST_N_A {
    #[doc = "0: Assert the USART0 reset."]
    UART0_RST_N_0 = 0,
    #[doc = "1: Clear the USART0 reset."]
    UART0_RST_N_1 = 1,
}
impl From<UART0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl UART0_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_RST_N_A {
        match self.bits {
            false => UART0_RST_N_A::UART0_RST_N_0,
            true => UART0_RST_N_A::UART0_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_RST_N_0`"]
    #[inline(always)]
    pub fn is_uart0_rst_n_0(&self) -> bool {
        *self == UART0_RST_N_A::UART0_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UART0_RST_N_1`"]
    #[inline(always)]
    pub fn is_uart0_rst_n_1(&self) -> bool {
        *self == UART0_RST_N_A::UART0_RST_N_1
    }
}
#[doc = "Field `UART0_RST_N` writer - USART0 reset control."]
pub type UART0_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, UART0_RST_N_A, O>;
impl<'a, const O: u8> UART0_RST_N_W<'a, O> {
    #[doc = "Assert the USART0 reset."]
    #[inline(always)]
    pub fn uart0_rst_n_0(self) -> &'a mut W {
        self.variant(UART0_RST_N_A::UART0_RST_N_0)
    }
    #[doc = "Clear the USART0 reset."]
    #[inline(always)]
    pub fn uart0_rst_n_1(self) -> &'a mut W {
        self.variant(UART0_RST_N_A::UART0_RST_N_1)
    }
}
#[doc = "Field `UART1_RST_N` reader - USART1 reset control."]
pub type UART1_RST_N_R = crate::BitReader<UART1_RST_N_A>;
#[doc = "USART1 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_RST_N_A {
    #[doc = "0: Assert the USART1 reset."]
    UART1_RST_N_0 = 0,
    #[doc = "1: Clear the USART1 reset."]
    UART1_RST_N_1 = 1,
}
impl From<UART1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl UART1_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_RST_N_A {
        match self.bits {
            false => UART1_RST_N_A::UART1_RST_N_0,
            true => UART1_RST_N_A::UART1_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_RST_N_0`"]
    #[inline(always)]
    pub fn is_uart1_rst_n_0(&self) -> bool {
        *self == UART1_RST_N_A::UART1_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UART1_RST_N_1`"]
    #[inline(always)]
    pub fn is_uart1_rst_n_1(&self) -> bool {
        *self == UART1_RST_N_A::UART1_RST_N_1
    }
}
#[doc = "Field `UART1_RST_N` writer - USART1 reset control."]
pub type UART1_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, UART1_RST_N_A, O>;
impl<'a, const O: u8> UART1_RST_N_W<'a, O> {
    #[doc = "Assert the USART1 reset."]
    #[inline(always)]
    pub fn uart1_rst_n_0(self) -> &'a mut W {
        self.variant(UART1_RST_N_A::UART1_RST_N_0)
    }
    #[doc = "Clear the USART1 reset."]
    #[inline(always)]
    pub fn uart1_rst_n_1(self) -> &'a mut W {
        self.variant(UART1_RST_N_A::UART1_RST_N_1)
    }
}
#[doc = "Field `UART2_RST_N` reader - USART2 reset control."]
pub type UART2_RST_N_R = crate::BitReader<UART2_RST_N_A>;
#[doc = "USART2 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_RST_N_A {
    #[doc = "0: Assert the USART2 reset."]
    UART2_RST_N_0 = 0,
    #[doc = "1: Clear the USART2 reset."]
    UART2_RST_N_1 = 1,
}
impl From<UART2_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl UART2_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_RST_N_A {
        match self.bits {
            false => UART2_RST_N_A::UART2_RST_N_0,
            true => UART2_RST_N_A::UART2_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART2_RST_N_0`"]
    #[inline(always)]
    pub fn is_uart2_rst_n_0(&self) -> bool {
        *self == UART2_RST_N_A::UART2_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UART2_RST_N_1`"]
    #[inline(always)]
    pub fn is_uart2_rst_n_1(&self) -> bool {
        *self == UART2_RST_N_A::UART2_RST_N_1
    }
}
#[doc = "Field `UART2_RST_N` writer - USART2 reset control."]
pub type UART2_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, UART2_RST_N_A, O>;
impl<'a, const O: u8> UART2_RST_N_W<'a, O> {
    #[doc = "Assert the USART2 reset."]
    #[inline(always)]
    pub fn uart2_rst_n_0(self) -> &'a mut W {
        self.variant(UART2_RST_N_A::UART2_RST_N_0)
    }
    #[doc = "Clear the USART2 reset."]
    #[inline(always)]
    pub fn uart2_rst_n_1(self) -> &'a mut W {
        self.variant(UART2_RST_N_A::UART2_RST_N_1)
    }
}
#[doc = "Field `I2C0_RST_N` reader - I2C0 reset control."]
pub type I2C0_RST_N_R = crate::BitReader<I2C0_RST_N_A>;
#[doc = "I2C0 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_RST_N_A {
    #[doc = "0: Assert the I2C0 reset."]
    I2C0_RST_N_0 = 0,
    #[doc = "1: Clear the I2C0 reset."]
    I2C0_RST_N_1 = 1,
}
impl From<I2C0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_RST_N_A {
        match self.bits {
            false => I2C0_RST_N_A::I2C0_RST_N_0,
            true => I2C0_RST_N_A::I2C0_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C0_RST_N_0`"]
    #[inline(always)]
    pub fn is_i2c0_rst_n_0(&self) -> bool {
        *self == I2C0_RST_N_A::I2C0_RST_N_0
    }
    #[doc = "Checks if the value of the field is `I2C0_RST_N_1`"]
    #[inline(always)]
    pub fn is_i2c0_rst_n_1(&self) -> bool {
        *self == I2C0_RST_N_A::I2C0_RST_N_1
    }
}
#[doc = "Field `I2C0_RST_N` writer - I2C0 reset control."]
pub type I2C0_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, I2C0_RST_N_A, O>;
impl<'a, const O: u8> I2C0_RST_N_W<'a, O> {
    #[doc = "Assert the I2C0 reset."]
    #[inline(always)]
    pub fn i2c0_rst_n_0(self) -> &'a mut W {
        self.variant(I2C0_RST_N_A::I2C0_RST_N_0)
    }
    #[doc = "Clear the I2C0 reset."]
    #[inline(always)]
    pub fn i2c0_rst_n_1(self) -> &'a mut W {
        self.variant(I2C0_RST_N_A::I2C0_RST_N_1)
    }
}
#[doc = "Field `MRT_RST_N` reader - Multi-rate timer (MRT) reset control."]
pub type MRT_RST_N_R = crate::BitReader<MRT_RST_N_A>;
#[doc = "Multi-rate timer (MRT) reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RST_N_A {
    #[doc = "0: Assert the MRT reset."]
    MRT_RST_N_0 = 0,
    #[doc = "1: Clear the MRT reset."]
    MRT_RST_N_1 = 1,
}
impl From<MRT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl MRT_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_RST_N_A {
        match self.bits {
            false => MRT_RST_N_A::MRT_RST_N_0,
            true => MRT_RST_N_A::MRT_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRT_RST_N_0`"]
    #[inline(always)]
    pub fn is_mrt_rst_n_0(&self) -> bool {
        *self == MRT_RST_N_A::MRT_RST_N_0
    }
    #[doc = "Checks if the value of the field is `MRT_RST_N_1`"]
    #[inline(always)]
    pub fn is_mrt_rst_n_1(&self) -> bool {
        *self == MRT_RST_N_A::MRT_RST_N_1
    }
}
#[doc = "Field `MRT_RST_N` writer - Multi-rate timer (MRT) reset control."]
pub type MRT_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL_SPEC, MRT_RST_N_A, O>;
impl<'a, const O: u8> MRT_RST_N_W<'a, O> {
    #[doc = "Assert the MRT reset."]
    #[inline(always)]
    pub fn mrt_rst_n_0(self) -> &'a mut W {
        self.variant(MRT_RST_N_A::MRT_RST_N_0)
    }
    #[doc = "Clear the MRT reset."]
    #[inline(always)]
    pub fn mrt_rst_n_1(self) -> &'a mut W {
        self.variant(MRT_RST_N_A::MRT_RST_N_1)
    }
}
#[doc = "Field `SCT_RST_N` reader - SCT reset control."]
pub type SCT_RST_N_R = crate::BitReader<SCT_RST_N_A>;
#[doc = "SCT reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RST_N_A {
    #[doc = "0: Assert the SCT reset."]
    SCT_RST_N_0 = 0,
    #[doc = "1: Clear the SCT reset."]
    SCT_RST_N_1 = 1,
}
impl From<SCT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_RST_N_A {
        match self.bits {
            false => SCT_RST_N_A::SCT_RST_N_0,
            true => SCT_RST_N_A::SCT_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCT_RST_N_0`"]
    #[inline(always)]
    pub fn is_sct_rst_n_0(&self) -> bool {
        *self == SCT_RST_N_A::SCT_RST_N_0
    }
    #[doc = "Checks if the value of the field is `SCT_RST_N_1`"]
    #[inline(always)]
    pub fn is_sct_rst_n_1(&self) -> bool {
        *self == SCT_RST_N_A::SCT_RST_N_1
    }
}
#[doc = "Field `SCT_RST_N` writer - SCT reset control."]
pub type SCT_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL_SPEC, SCT_RST_N_A, O>;
impl<'a, const O: u8> SCT_RST_N_W<'a, O> {
    #[doc = "Assert the SCT reset."]
    #[inline(always)]
    pub fn sct_rst_n_0(self) -> &'a mut W {
        self.variant(SCT_RST_N_A::SCT_RST_N_0)
    }
    #[doc = "Clear the SCT reset."]
    #[inline(always)]
    pub fn sct_rst_n_1(self) -> &'a mut W {
        self.variant(SCT_RST_N_A::SCT_RST_N_1)
    }
}
#[doc = "Field `WKT_RST_N` reader - Self-wake-up timer (WKT) reset control."]
pub type WKT_RST_N_R = crate::BitReader<WKT_RST_N_A>;
#[doc = "Self-wake-up timer (WKT) reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_RST_N_A {
    #[doc = "0: Assert the WKT reset."]
    WKT_RST_N_0 = 0,
    #[doc = "1: Clear the WKT reset."]
    WKT_RST_N_1 = 1,
}
impl From<WKT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: WKT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl WKT_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKT_RST_N_A {
        match self.bits {
            false => WKT_RST_N_A::WKT_RST_N_0,
            true => WKT_RST_N_A::WKT_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKT_RST_N_0`"]
    #[inline(always)]
    pub fn is_wkt_rst_n_0(&self) -> bool {
        *self == WKT_RST_N_A::WKT_RST_N_0
    }
    #[doc = "Checks if the value of the field is `WKT_RST_N_1`"]
    #[inline(always)]
    pub fn is_wkt_rst_n_1(&self) -> bool {
        *self == WKT_RST_N_A::WKT_RST_N_1
    }
}
#[doc = "Field `WKT_RST_N` writer - Self-wake-up timer (WKT) reset control."]
pub type WKT_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL_SPEC, WKT_RST_N_A, O>;
impl<'a, const O: u8> WKT_RST_N_W<'a, O> {
    #[doc = "Assert the WKT reset."]
    #[inline(always)]
    pub fn wkt_rst_n_0(self) -> &'a mut W {
        self.variant(WKT_RST_N_A::WKT_RST_N_0)
    }
    #[doc = "Clear the WKT reset."]
    #[inline(always)]
    pub fn wkt_rst_n_1(self) -> &'a mut W {
        self.variant(WKT_RST_N_A::WKT_RST_N_1)
    }
}
#[doc = "Field `GPIO_RST_N` reader - GPIO and GPIO pin interrupt reset control."]
pub type GPIO_RST_N_R = crate::BitReader<GPIO_RST_N_A>;
#[doc = "GPIO and GPIO pin interrupt reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_RST_N_A {
    #[doc = "0: Assert the GPIO reset."]
    GPIO_RST_N_0 = 0,
    #[doc = "1: Clear the GPIO reset."]
    GPIO_RST_N_1 = 1,
}
impl From<GPIO_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_RST_N_A {
        match self.bits {
            false => GPIO_RST_N_A::GPIO_RST_N_0,
            true => GPIO_RST_N_A::GPIO_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_RST_N_0`"]
    #[inline(always)]
    pub fn is_gpio_rst_n_0(&self) -> bool {
        *self == GPIO_RST_N_A::GPIO_RST_N_0
    }
    #[doc = "Checks if the value of the field is `GPIO_RST_N_1`"]
    #[inline(always)]
    pub fn is_gpio_rst_n_1(&self) -> bool {
        *self == GPIO_RST_N_A::GPIO_RST_N_1
    }
}
#[doc = "Field `GPIO_RST_N` writer - GPIO and GPIO pin interrupt reset control."]
pub type GPIO_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, GPIO_RST_N_A, O>;
impl<'a, const O: u8> GPIO_RST_N_W<'a, O> {
    #[doc = "Assert the GPIO reset."]
    #[inline(always)]
    pub fn gpio_rst_n_0(self) -> &'a mut W {
        self.variant(GPIO_RST_N_A::GPIO_RST_N_0)
    }
    #[doc = "Clear the GPIO reset."]
    #[inline(always)]
    pub fn gpio_rst_n_1(self) -> &'a mut W {
        self.variant(GPIO_RST_N_A::GPIO_RST_N_1)
    }
}
#[doc = "Field `FLASH_RST_N` reader - Flash controller reset control."]
pub type FLASH_RST_N_R = crate::BitReader<FLASH_RST_N_A>;
#[doc = "Flash controller reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RST_N_A {
    #[doc = "0: Assert the flash controller reset."]
    FLASH_RST_N_0 = 0,
    #[doc = "1: Clear the flash controller reset."]
    FLASH_RST_N_1 = 1,
}
impl From<FLASH_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_RST_N_A {
        match self.bits {
            false => FLASH_RST_N_A::FLASH_RST_N_0,
            true => FLASH_RST_N_A::FLASH_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_RST_N_0`"]
    #[inline(always)]
    pub fn is_flash_rst_n_0(&self) -> bool {
        *self == FLASH_RST_N_A::FLASH_RST_N_0
    }
    #[doc = "Checks if the value of the field is `FLASH_RST_N_1`"]
    #[inline(always)]
    pub fn is_flash_rst_n_1(&self) -> bool {
        *self == FLASH_RST_N_A::FLASH_RST_N_1
    }
}
#[doc = "Field `FLASH_RST_N` writer - Flash controller reset control."]
pub type FLASH_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, FLASH_RST_N_A, O>;
impl<'a, const O: u8> FLASH_RST_N_W<'a, O> {
    #[doc = "Assert the flash controller reset."]
    #[inline(always)]
    pub fn flash_rst_n_0(self) -> &'a mut W {
        self.variant(FLASH_RST_N_A::FLASH_RST_N_0)
    }
    #[doc = "Clear the flash controller reset."]
    #[inline(always)]
    pub fn flash_rst_n_1(self) -> &'a mut W {
        self.variant(FLASH_RST_N_A::FLASH_RST_N_1)
    }
}
#[doc = "Field `ACMP_RST_N` reader - Analog comparator reset control."]
pub type ACMP_RST_N_R = crate::BitReader<ACMP_RST_N_A>;
#[doc = "Analog comparator reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_RST_N_A {
    #[doc = "0: Assert the analog comparator reset."]
    ACMP_RST_N_0 = 0,
    #[doc = "1: Clear the analog comparator controller reset."]
    ACMP_RST_N_1 = 1,
}
impl From<ACMP_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_RST_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_RST_N_A {
        match self.bits {
            false => ACMP_RST_N_A::ACMP_RST_N_0,
            true => ACMP_RST_N_A::ACMP_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_RST_N_0`"]
    #[inline(always)]
    pub fn is_acmp_rst_n_0(&self) -> bool {
        *self == ACMP_RST_N_A::ACMP_RST_N_0
    }
    #[doc = "Checks if the value of the field is `ACMP_RST_N_1`"]
    #[inline(always)]
    pub fn is_acmp_rst_n_1(&self) -> bool {
        *self == ACMP_RST_N_A::ACMP_RST_N_1
    }
}
#[doc = "Field `ACMP_RST_N` writer - Analog comparator reset control."]
pub type ACMP_RST_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_SPEC, ACMP_RST_N_A, O>;
impl<'a, const O: u8> ACMP_RST_N_W<'a, O> {
    #[doc = "Assert the analog comparator reset."]
    #[inline(always)]
    pub fn acmp_rst_n_0(self) -> &'a mut W {
        self.variant(ACMP_RST_N_A::ACMP_RST_N_0)
    }
    #[doc = "Clear the analog comparator controller reset."]
    #[inline(always)]
    pub fn acmp_rst_n_1(self) -> &'a mut W {
        self.variant(ACMP_RST_N_A::ACMP_RST_N_1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline(always)]
    pub fn spi0_rst_n(&self) -> SPI0_RST_N_R {
        SPI0_RST_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI1 reset control."]
    #[inline(always)]
    pub fn spi1_rst_n(&self) -> SPI1_RST_N_R {
        SPI1_RST_N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART fractional baud rate generator(UARTFRG) reset control."]
    #[inline(always)]
    pub fn uartfrg_rst_n(&self) -> UARTFRG_RST_N_R {
        UARTFRG_RST_N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART0 reset control."]
    #[inline(always)]
    pub fn uart0_rst_n(&self) -> UART0_RST_N_R {
        UART0_RST_N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 reset control."]
    #[inline(always)]
    pub fn uart1_rst_n(&self) -> UART1_RST_N_R {
        UART1_RST_N_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART2 reset control."]
    #[inline(always)]
    pub fn uart2_rst_n(&self) -> UART2_RST_N_R {
        UART2_RST_N_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C0 reset control."]
    #[inline(always)]
    pub fn i2c0_rst_n(&self) -> I2C0_RST_N_R {
        I2C0_RST_N_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multi-rate timer (MRT) reset control."]
    #[inline(always)]
    pub fn mrt_rst_n(&self) -> MRT_RST_N_R {
        MRT_RST_N_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst_n(&self) -> SCT_RST_N_R {
        SCT_RST_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control."]
    #[inline(always)]
    pub fn wkt_rst_n(&self) -> WKT_RST_N_R {
        WKT_RST_N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO and GPIO pin interrupt reset control."]
    #[inline(always)]
    pub fn gpio_rst_n(&self) -> GPIO_RST_N_R {
        GPIO_RST_N_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst_n(&self) -> FLASH_RST_N_R {
        FLASH_RST_N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Analog comparator reset control."]
    #[inline(always)]
    pub fn acmp_rst_n(&self) -> ACMP_RST_N_R {
        ACMP_RST_N_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline(always)]
    pub fn spi0_rst_n(&mut self) -> SPI0_RST_N_W<0> {
        SPI0_RST_N_W::new(self)
    }
    #[doc = "Bit 1 - SPI1 reset control."]
    #[inline(always)]
    pub fn spi1_rst_n(&mut self) -> SPI1_RST_N_W<1> {
        SPI1_RST_N_W::new(self)
    }
    #[doc = "Bit 2 - USART fractional baud rate generator(UARTFRG) reset control."]
    #[inline(always)]
    pub fn uartfrg_rst_n(&mut self) -> UARTFRG_RST_N_W<2> {
        UARTFRG_RST_N_W::new(self)
    }
    #[doc = "Bit 3 - USART0 reset control."]
    #[inline(always)]
    pub fn uart0_rst_n(&mut self) -> UART0_RST_N_W<3> {
        UART0_RST_N_W::new(self)
    }
    #[doc = "Bit 4 - USART1 reset control."]
    #[inline(always)]
    pub fn uart1_rst_n(&mut self) -> UART1_RST_N_W<4> {
        UART1_RST_N_W::new(self)
    }
    #[doc = "Bit 5 - USART2 reset control."]
    #[inline(always)]
    pub fn uart2_rst_n(&mut self) -> UART2_RST_N_W<5> {
        UART2_RST_N_W::new(self)
    }
    #[doc = "Bit 6 - I2C0 reset control."]
    #[inline(always)]
    pub fn i2c0_rst_n(&mut self) -> I2C0_RST_N_W<6> {
        I2C0_RST_N_W::new(self)
    }
    #[doc = "Bit 7 - Multi-rate timer (MRT) reset control."]
    #[inline(always)]
    pub fn mrt_rst_n(&mut self) -> MRT_RST_N_W<7> {
        MRT_RST_N_W::new(self)
    }
    #[doc = "Bit 8 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst_n(&mut self) -> SCT_RST_N_W<8> {
        SCT_RST_N_W::new(self)
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control."]
    #[inline(always)]
    pub fn wkt_rst_n(&mut self) -> WKT_RST_N_W<9> {
        WKT_RST_N_W::new(self)
    }
    #[doc = "Bit 10 - GPIO and GPIO pin interrupt reset control."]
    #[inline(always)]
    pub fn gpio_rst_n(&mut self) -> GPIO_RST_N_W<10> {
        GPIO_RST_N_W::new(self)
    }
    #[doc = "Bit 11 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst_n(&mut self) -> FLASH_RST_N_W<11> {
        FLASH_RST_N_W::new(self)
    }
    #[doc = "Bit 12 - Analog comparator reset control."]
    #[inline(always)]
    pub fn acmp_rst_n(&mut self) -> ACMP_RST_N_W<12> {
        ACMP_RST_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl](index.html) module"]
pub struct PRESETCTRL_SPEC;
impl crate::RegisterSpec for PRESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL to value 0x2101_dfff"]
impl crate::Resettable for PRESETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101_dfff
    }
}
