#[doc = "Register `STARTERP1` reader"]
pub struct R(crate::R<STARTERP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTERP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTERP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTERP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTERP1` writer"]
pub struct W(crate::W<STARTERP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERP1_SPEC>;
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
impl From<crate::W<STARTERP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0` reader - SPI0 interrupt wake-up"]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "SPI0 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => SPI0_A::DISABLED,
            true => SPI0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI0_A::ENABLED
    }
}
#[doc = "Field `SPI0` writer - SPI0 interrupt wake-up"]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, SPI0_A, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI0_A::ENABLED)
    }
}
#[doc = "Field `SPI1` reader - SPI1 interrupt wake-up"]
pub type SPI1_R = crate::BitReader<SPI1_A>;
#[doc = "SPI1 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => SPI1_A::DISABLED,
            true => SPI1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI1_A::ENABLED
    }
}
#[doc = "Field `SPI1` writer - SPI1 interrupt wake-up"]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, SPI1_A, O>;
impl<'a, const O: u8> SPI1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1_A::ENABLED)
    }
}
#[doc = "Field `USART0` reader - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
pub type USART0_R = crate::BitReader<USART0_A>;
#[doc = "USART0 interrupt wake-up. Configure USART in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USART0_A> for bool {
    #[inline(always)]
    fn from(variant: USART0_A) -> Self {
        variant as u8 != 0
    }
}
impl USART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0_A {
        match self.bits {
            false => USART0_A::DISABLED,
            true => USART0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART0_A::ENABLED
    }
}
#[doc = "Field `USART0` writer - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
pub type USART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, USART0_A, O>;
impl<'a, const O: u8> USART0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART0_A::ENABLED)
    }
}
#[doc = "Field `USART1` reader - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
pub type USART1_R = crate::BitReader<USART1_A>;
#[doc = "USART1 interrupt wake-up. Configure USART in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USART1_A> for bool {
    #[inline(always)]
    fn from(variant: USART1_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1_A {
        match self.bits {
            false => USART1_A::DISABLED,
            true => USART1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1_A::ENABLED
    }
}
#[doc = "Field `USART1` writer - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
pub type USART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, USART1_A, O>;
impl<'a, const O: u8> USART1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1_A::ENABLED)
    }
}
#[doc = "Field `USART2` reader - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
pub type USART2_R = crate::BitReader<USART2_A>;
#[doc = "USART2 interrupt wake-up. Configure USART in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USART2_A> for bool {
    #[inline(always)]
    fn from(variant: USART2_A) -> Self {
        variant as u8 != 0
    }
}
impl USART2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2_A {
        match self.bits {
            false => USART2_A::DISABLED,
            true => USART2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART2_A::ENABLED
    }
}
#[doc = "Field `USART2` writer - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
pub type USART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, USART2_A, O>;
impl<'a, const O: u8> USART2_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART2_A::ENABLED)
    }
}
#[doc = "Field `I2C0` reader - I2C0 interrupt wake-up."]
pub type I2C0_R = crate::BitReader<I2C0_A>;
#[doc = "I2C0 interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => I2C0_A::DISABLED,
            true => I2C0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C0_A::ENABLED
    }
}
#[doc = "Field `I2C0` writer - I2C0 interrupt wake-up."]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, I2C0_A, O>;
impl<'a, const O: u8> I2C0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C0_A::ENABLED)
    }
}
#[doc = "Field `WWDT` reader - WWDT interrupt wake-up"]
pub type WWDT_R = crate::BitReader<WWDT_A>;
#[doc = "WWDT interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => WWDT_A::DISABLED,
            true => WWDT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDT_A::ENABLED
    }
}
#[doc = "Field `WWDT` writer - WWDT interrupt wake-up"]
pub type WWDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, WWDT_A, O>;
impl<'a, const O: u8> WWDT_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLED)
    }
}
#[doc = "Field `BOD` reader - BOD interrupt wake-up"]
pub type BOD_R = crate::BitReader<BOD_A>;
#[doc = "BOD interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BOD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_A) -> Self {
        variant as u8 != 0
    }
}
impl BOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_A {
        match self.bits {
            false => BOD_A::DISABLED,
            true => BOD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOD_A::ENABLED
    }
}
#[doc = "Field `BOD` writer - BOD interrupt wake-up"]
pub type BOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, BOD_A, O>;
impl<'a, const O: u8> BOD_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOD_A::ENABLED)
    }
}
#[doc = "Field `WKT` reader - Self-wake-up timer interrupt wake-up"]
pub type WKT_R = crate::BitReader<WKT_A>;
#[doc = "Self-wake-up timer interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => WKT_A::DISABLED,
            true => WKT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKT_A::ENABLED
    }
}
#[doc = "Field `WKT` writer - Self-wake-up timer interrupt wake-up"]
pub type WKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTERP1_SPEC, WKT_A, O>;
impl<'a, const O: u8> WKT_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKT_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - SPI0 interrupt wake-up"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt wake-up"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C0 interrupt wake-up."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BOD interrupt wake-up"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Self-wake-up timer interrupt wake-up"]
    #[inline(always)]
    pub fn wkt(&self) -> WKT_R {
        WKT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 interrupt wake-up"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W<0> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 1 - SPI1 interrupt wake-up"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<1> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 3 - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart0(&mut self) -> USART0_W<3> {
        USART0_W::new(self)
    }
    #[doc = "Bit 4 - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart1(&mut self) -> USART1_W<4> {
        USART1_W::new(self)
    }
    #[doc = "Bit 5 - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart2(&mut self) -> USART2_W<5> {
        USART2_W::new(self)
    }
    #[doc = "Bit 8 - I2C0 interrupt wake-up."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W<8> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W<12> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 13 - BOD interrupt wake-up"]
    #[inline(always)]
    pub fn bod(&mut self) -> BOD_W<13> {
        BOD_W::new(self)
    }
    #[doc = "Bit 15 - Self-wake-up timer interrupt wake-up"]
    #[inline(always)]
    pub fn wkt(&mut self) -> WKT_W<15> {
        WKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 0 pin wake-up enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp1](index.html) module"]
pub struct STARTERP1_SPEC;
impl crate::RegisterSpec for STARTERP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starterp1::R](R) reader structure"]
impl crate::Readable for STARTERP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starterp1::W](W) writer structure"]
impl crate::Writable for STARTERP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTERP1 to value 0"]
impl crate::Resettable for STARTERP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
